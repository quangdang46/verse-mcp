#!/usr/bin/env bash
set -euo pipefail
umask 022

BINARY_NAME="vm"
MCP_SERVER_NAME="verse-mcp"
OWNER="quangdang46"
REPO="verse-mcp"
DEST="${DEST:-$HOME/.local/bin}"
VERSION="${VERSION:-}"
QUIET=0
EASY=0
VERIFY=0
FROM_SOURCE=0
UNINSTALL=0
MAX_RETRIES=3
DOWNLOAD_TIMEOUT=120
LOCK_DIR="/tmp/${BINARY_NAME}-install.lock.d"
TMP=""

log_info()    { [ "$QUIET" -eq 1 ] && return; echo "[${BINARY_NAME}] $*" >&2; }
log_warn()    { echo "[${BINARY_NAME}] WARN: $*" >&2; }
log_success() { [ "$QUIET" -eq 1 ] && return; echo "✓ $*" >&2; }
die()         { echo "ERROR: $*" >&2; exit 1; }

cleanup() { rm -rf "$TMP" "$LOCK_DIR" 2>/dev/null || true; }
trap cleanup EXIT

acquire_lock() {
    if mkdir "$LOCK_DIR" 2>/dev/null; then
        echo $$ > "$LOCK_DIR/pid"
        return 0
    fi
    die "Another install is running. If stuck: rm -rf $LOCK_DIR"
}

usage() {
    cat <<'EOF'
Usage: install.sh [OPTIONS]

Options:
  --dest DIR           Install into DIR
  --dest=DIR           Install into DIR
  --version TAG        Install a specific release tag
  --version=TAG        Install a specific release tag
  --system             Install into /usr/local/bin
  --easy-mode          Add destination to shell rc PATH
  --verify             Run installed binary with --version
  --from-source        Build from source instead of downloading a release
  --quiet, -q          Reduce output
  --uninstall          Remove installed binary
  -h, --help           Show this help
EOF
    exit 0
}

while [ $# -gt 0 ]; do
    case "$1" in
        --dest)
            DEST="$2"
            shift 2
            ;;
        --dest=*)
            DEST="${1#*=}"
            shift
            ;;
        --version)
            VERSION="$2"
            shift 2
            ;;
        --version=*)
            VERSION="${1#*=}"
            shift
            ;;
        --system)
            DEST="/usr/local/bin"
            shift
            ;;
        --easy-mode)
            EASY=1
            shift
            ;;
        --verify)
            VERIFY=1
            shift
            ;;
        --from-source)
            FROM_SOURCE=1
            shift
            ;;
        --quiet|-q)
            QUIET=1
            shift
            ;;
        --uninstall)
            UNINSTALL=1
            shift
            ;;
        -h|--help)
            usage
            ;;
        *)
            die "Unknown option: $1"
            ;;
    esac
done

do_uninstall() {
    rm -f "$DEST/$BINARY_NAME"
    for rc in "$HOME/.bashrc" "$HOME/.zshrc"; do
        [ -f "$rc" ] && sed -i "/${BINARY_NAME} installer/d" "$rc" 2>/dev/null || true
    done
    log_success "Uninstalled"
    exit 0
}
[ "$UNINSTALL" -eq 1 ] && do_uninstall

detect_platform() {
    local os arch
    case "$(uname -s)" in
        Linux*) os="linux" ;;
        Darwin*) os="macos" ;;
        MINGW*|MSYS*|CYGWIN*) os="windows" ;;
        *) die "Unsupported OS: $(uname -s)" ;;
    esac
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *) die "Unsupported arch: $(uname -m)" ;;
    esac
    echo "${os}-${arch}"
}

resolve_version() {
    [ -n "$VERSION" ] && return 0
    VERSION=$(curl -fsSL \
        --connect-timeout 10 --max-time 30 \
        -H "Accept: application/vnd.github.v3+json" \
        "https://api.github.com/repos/${OWNER}/${REPO}/releases/latest" \
        2>/dev/null | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/') || true
    if [ -z "$VERSION" ]; then
        VERSION=$(curl -fsSL -o /dev/null -w '%{url_effective}' \
            "https://github.com/${OWNER}/${REPO}/releases/latest" \
            2>/dev/null | sed -E 's|.*/tag/||') || true
    fi
    [[ "$VERSION" =~ ^v[0-9] ]] || die "Could not resolve version"
    log_info "Latest: $VERSION"
}

download_file() {
    local url="$1" dest="$2"
    local partial="${dest}.part"
    local attempt=0
    while [ $attempt -lt $MAX_RETRIES ]; do
        attempt=$((attempt + 1))
        curl -fL \
            --connect-timeout 30 \
            --max-time "$DOWNLOAD_TIMEOUT" \
            --retry 2 \
            $( [ -s "$partial" ] && echo "--continue-at -" ) \
            $( [ "$QUIET" -eq 0 ] && [ -t 2 ] && echo "--progress-bar" || echo "-sS" ) \
            -o "$partial" "$url" && mv -f "$partial" "$dest" && return 0
        [ $attempt -lt $MAX_RETRIES ] && { log_warn "Retrying in 3s..."; sleep 3; }
    done
    return 1
}

install_binary_atomic() {
    local src="$1" dest="$2"
    local tmp="${dest}.tmp.$$"
    install -m 0755 "$src" "$tmp"
    mv -f "$tmp" "$dest" || { rm -f "$tmp"; die "Failed to install binary"; }
}

maybe_add_path() {
    case ":$PATH:" in *":$DEST:"*) return 0 ;; esac
    if [ "$EASY" -eq 1 ]; then
        for rc in "$HOME/.zshrc" "$HOME/.bashrc"; do
            [ -f "$rc" ] && [ -w "$rc" ] || continue
            grep -qF "$DEST" "$rc" && continue
            printf '\nexport PATH="%s:$PATH"  # %s installer\n' "$DEST" "$BINARY_NAME" >> "$rc"
        done
        log_warn "PATH updated — restart shell or: export PATH=\"$DEST:\$PATH\""
    else
        log_warn "Add to PATH: export PATH=\"$DEST:\$PATH\""
    fi
}

build_from_source() {
    command -v cargo >/dev/null || die "Rust/cargo not found. Install: https://rustup.rs"
    command -v git >/dev/null || die "git not found"
    git clone --depth 1 "https://github.com/${OWNER}/${REPO}.git" "$TMP/src"
    (
        cd "$TMP/src"
        CARGO_TARGET_DIR="$TMP/target" cargo build --release -p mcp_server
    )
    install_binary_atomic "$TMP/target/release/$BINARY_NAME" "$DEST/$BINARY_NAME"
}

find_python3_tool() {
    if command -v python3 >/dev/null 2>&1; then
        echo "python3"
        return 0
    fi

    if command -v python >/dev/null 2>&1 && \
        python -c 'import sys; raise SystemExit(0 if sys.version_info[0] >= 3 else 1)' \
            >/dev/null 2>&1; then
        echo "python"
        return 0
    fi

    return 1
}

append_unique_host() {
    local host="$1"
    local existing
    for existing in "${MCP_HOSTS[@]:-}"; do
        [ "$existing" = "$host" ] && return 0
    done
    MCP_HOSTS+=("$host")
}

detect_mcp_hosts() {
    MCP_HOSTS=()

    if [ -n "${VERSE_MCP_HOST:-}" ]; then
        local raw host
        IFS=',' read -r -a raw <<< "${VERSE_MCP_HOST}"
        for host in "${raw[@]}"; do
            host="$(printf '%s' "$host" | xargs)"
            [ -n "$host" ] || continue
            case "$host" in
                claude-code|cursor|windsurf|vscode|gemini|opencode|codex|amp|droid)
                    append_unique_host "$host"
                    ;;
                *)
                    log_warn "Ignoring unknown MCP host override: $host"
                    ;;
            esac
        done
    else
        [ -d "$HOME/.codex" ] && append_unique_host "codex"
        [ -f "$HOME/.claude.json" ] && append_unique_host "claude-code"
        [ -d "$HOME/.cursor" ] && append_unique_host "cursor"
        [ -d "$HOME/.codeium/windsurf" ] && append_unique_host "windsurf"
        [ -d "$PWD/.vscode" ] && append_unique_host "vscode"
        [ -d "$HOME/.gemini" ] && append_unique_host "gemini"
        [ -f "$HOME/.opencode.json" ] && append_unique_host "opencode"
        [ -d "$HOME/.config/amp" ] && append_unique_host "amp"
        [ -d "$HOME/.factory" ] && append_unique_host "droid"
    fi

    return 0
}

upsert_json_mcp_server() {
    local python_cmd="$1"
    local path="$2"
    local servers_key="$3"
    local command_path="$4"

    "$python_cmd" - "$path" "$servers_key" "$MCP_SERVER_NAME" "$command_path" <<'PY'
import json
import os
import sys

path, servers_key, server_name, command, *args = sys.argv[1:]
entry = {"command": command, "args": args}

if os.path.exists(path):
    with open(path, "r", encoding="utf-8") as fh:
        data = json.load(fh)
else:
    data = {}

if not isinstance(data, dict):
    raise SystemExit(f"config root is not a JSON object: {path}")

servers = data.setdefault(servers_key, {})
if not isinstance(servers, dict):
    raise SystemExit(f"{servers_key} is not a JSON object: {path}")

existing = servers.get(server_name)
if existing is None:
    status = "installed"
elif existing == entry:
    status = "unchanged"
else:
    status = "updated"

servers[server_name] = entry

with open(path, "w", encoding="utf-8") as fh:
    json.dump(data, fh, indent=2)
    fh.write("\n")

print(status)
PY
}

upsert_toml_mcp_server() {
    local python_cmd="$1"
    local path="$2"
    local command_path="$3"

    "$python_cmd" - "$path" "$MCP_SERVER_NAME" "$command_path" <<'PY'
import os
import sys

path, server_name, command, *args = sys.argv[1:]
header = f"[mcp_servers.{server_name}]"
args_rendered = ", ".join(f'"{arg.replace("\\\\", "\\\\\\\\").replace("\"", "\\\\\"")}"' for arg in args)
command_rendered = command.replace("\\", "\\\\").replace('"', '\\"')
section = f'{header}\ncommand = "{command_rendered}"\nargs = [{args_rendered}]\n'

existing = ""
if os.path.exists(path):
    with open(path, "r", encoding="utf-8") as fh:
        existing = fh.read()

status = "installed"
if header in existing:
    start = existing.index(header)
    rest = existing[start + len(header):]
    next_section = rest.find("\n[")
    end = len(existing) if next_section == -1 else start + len(header) + next_section + 1
    current = existing[start:end]
    if current.strip() == section.strip():
        print("unchanged")
        raise SystemExit(0)
    updated = existing[:start] + section + existing[end:]
    status = "updated"
else:
    separator = ""
    if existing and not existing.endswith("\n"):
        separator = "\n"
    if existing:
        separator += "\n"
    updated = existing + separator + section

with open(path, "w", encoding="utf-8") as fh:
    fh.write(updated)

print(status)
PY
}

install_mcp_host() {
    local python_cmd="$1"
    local host="$2"
    local installed_path="$3"
    local path format servers_key note status

    case "$host" in
        claude-code)
            path="$HOME/.claude.json"
            format="json"
            servers_key="mcpServers"
            note="User scope."
            ;;
        cursor)
            path="$HOME/.cursor/mcp.json"
            format="json"
            servers_key="mcpServers"
            note="Global scope."
            ;;
        windsurf)
            path="$HOME/.codeium/windsurf/mcp_config.json"
            format="json"
            servers_key="mcpServers"
            note="Global scope."
            ;;
        vscode)
            path="$PWD/.vscode/mcp.json"
            format="json"
            servers_key="servers"
            note="Project scope."
            ;;
        gemini)
            path="$HOME/.gemini/settings.json"
            format="json"
            servers_key="mcpServers"
            note="User scope."
            ;;
        opencode)
            path="$HOME/.opencode.json"
            format="json"
            servers_key="mcpServers"
            note="User scope."
            ;;
        codex)
            path="$HOME/.codex/config.toml"
            format="toml"
            servers_key=""
            note="User scope."
            ;;
        amp)
            path="$HOME/.config/amp/settings.json"
            format="json"
            servers_key="amp.mcpServers"
            note="User scope."
            ;;
        droid)
            path="$HOME/.factory/mcp.json"
            format="json"
            servers_key="mcpServers"
            note="User scope."
            ;;
        *)
            log_warn "Skipping unsupported MCP host: $host"
            return 1
            ;;
    esac

    mkdir -p "$(dirname "$path")"
    if [ "$format" = "json" ]; then
        status=$(upsert_json_mcp_server "$python_cmd" "$path" "$servers_key" "$installed_path") || {
            log_warn "Failed to update ${host} MCP config at ${path}"
            return 1
        }
    else
        status=$(upsert_toml_mcp_server "$python_cmd" "$path" "$installed_path") || {
            log_warn "Failed to update ${host} MCP config at ${path}"
            return 1
        }
    fi

    log_info "MCP ${status} for ${host}: ${path}"
    [ -n "$note" ] && log_info "  ${note}"
}

run_mcp_auto_install() {
    local installed_path="$1"
    local python_cmd
    local host
    local failed=0

    detect_mcp_hosts
    if [ "${#MCP_HOSTS[@]}" -eq 0 ]; then
        log_info "No supported MCP providers detected. Skipped auto-install."
        return 0
    fi

    if ! python_cmd=$(find_python3_tool); then
        log_warn "Skipping MCP auto-install: python3 is required to update provider configs"
        return 1
    fi

    log_info "Auto-installing MCP provider configs..."
    for host in "${MCP_HOSTS[@]}"; do
        install_mcp_host "$python_cmd" "$host" "$installed_path" || failed=1
    done

    [ "$failed" -eq 0 ]
}

print_summary() {
    echo ""
    echo "✓ ${BINARY_NAME} installed → $DEST/$BINARY_NAME"
    echo "  Version: $("$DEST/$BINARY_NAME" --version 2>/dev/null || echo 'unknown')"
    echo ""
    echo "  Quick start:"
    echo "    $BINARY_NAME --help"
}

main() {
    acquire_lock
    TMP=$(mktemp -d)
    mkdir -p "$DEST"

    local platform archive url bin_path expected actual
    platform=$(detect_platform)
    log_info "Platform: $platform | Dest: $DEST"

    if [ "$FROM_SOURCE" -eq 0 ]; then
        resolve_version
        case "$platform" in
            linux-*) archive="${BINARY_NAME}-${platform}.tar.gz" ;;
            macos-*) archive="${BINARY_NAME}-${platform}.tar.gz" ;;
            windows-*) archive="${BINARY_NAME}-${platform}.zip" ;;
            *) die "Unsupported platform: $platform" ;;
        esac
        url="https://github.com/${OWNER}/${REPO}/releases/download/${VERSION}/${archive}"

        if download_file "$url" "$TMP/$archive"; then
            if download_file "${url}.sha256" "$TMP/checksum.sha256" 2>/dev/null; then
                expected=$(awk '{print $1}' "$TMP/checksum.sha256")
                actual=$(sha256sum "$TMP/$archive" 2>/dev/null | awk '{print $1}' || shasum -a 256 "$TMP/$archive" | awk '{print $1}')
                [ "$expected" = "$actual" ] || die "Checksum mismatch"
                log_info "Checksum verified"
            fi

            case "$archive" in
                *.tar.gz)
                    tar -xzf "$TMP/$archive" -C "$TMP"
                    bin_path="$TMP/$BINARY_NAME"
                    ;;
                *.zip)
                    unzip -q "$TMP/$archive" -d "$TMP"
                    bin_path="$TMP/${BINARY_NAME}.exe"
                    ;;
            esac

            [ -f "$bin_path" ] || die "Binary not found after extract"
            install_binary_atomic "$bin_path" "$DEST/$BINARY_NAME"
        else
            log_warn "Binary download failed — building from source..."
            build_from_source
        fi
    else
        build_from_source
    fi

    maybe_add_path

    [ "$VERIFY" -eq 1 ] && "$DEST/$BINARY_NAME" --version

    run_mcp_auto_install "$DEST/$BINARY_NAME" || true
    print_summary
}

if [[ "${BASH_SOURCE[0]:-}" == "${0:-}" ]] || [[ -z "${BASH_SOURCE[0]:-}" ]]; then
    { main "$@"; }
fi
