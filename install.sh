#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
REPO="quangdang46/verse-mcp"
BINARY_NAME="vm"
VERSION="${VERSION:-latest}"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux*)
            PLATFORM="linux"
            ;;
        Darwin*)
            PLATFORM="macos"
            ;;
       MINGW*|MSYS*|CYGWIN*)
            PLATFORM="windows"
            echo -e "${YELLOW}For Windows, please download the binary manually from GitHub Releases${NC}"
            exit 1
            ;;
        *)
            echo -e "${RED}Unsupported OS: $OS${NC}"
            exit 1
            ;;
    esac

    case "$ARCH" in
        x86_64|amd64)
            ARCH_SUFFIX="x86_64"
            ;;
        arm64|aarch64)
            ARCH_SUFFIX="aarch64"
            ;;
        *)
            echo -e "${RED}Unsupported architecture: $ARCH${NC}"
            exit 1
            ;;
    esac

    # Check for macOS Intel on ARM64 (Rosetta)
    if [ "$PLATFORM" = "macos" ] && [ "$ARCH_SUFFIX" = "aarch64" ]; then
        if sysctl -n machdep.cpu.brand_string 2>/dev/null | grep -q "Intel"; then
            ARCH_SUFFIX="x86_64"
        fi
    fi

    ARTIFACT="${BINARY_NAME}-${ARCH_SUFFIX}-${PLATFORM}"
}

# Download and install
download_and_install() {
    local base_url="https://github.com/${REPO}/releases/download/${VERSION}"
    local archive_url

    if [ "$PLATFORM" = "windows" ]; then
        archive_url="${base_url}/${ARTIFACT}.zip"
    else
        archive_url="${base_url}/${ARTIFACT}.tar.gz"
    fi

    echo -e "${GREEN}Downloading ${ARTIFACT} from ${archive_url}...${NC}"

    # Create temp directory
    TMPDIR=$(mktemp -d)
    cd "$TMPDIR"

    # Download
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL -o "archive.${PLATFORM}" "$archive_url"
    elif command -v wget >/dev/null 2>&1; then
        wget -q -O "archive.${PLATFORM}" "$archive_url"
    else
        echo -e "${RED}Neither curl nor wget found. Please install one of them.${NC}"
        exit 1
    fi

    # Extract
    if [ "$PLATFORM" = "windows" ]; then
        if command -v unzip >/dev/null 2>&1; then
            unzip -q "archive.${PLATFORM}"
        else
            echo -e "${RED}unzip not found. Please install it.${NC}"
            exit 1
        fi
    else
        tar -xzf "archive.${PLATFORM}"
    fi

    # Create install directory if it doesn't exist
    mkdir -p "$INSTALL_DIR"

    # Move binary
    echo -e "${GREEN}Installing ${BINARY_NAME} to ${INSTALL_DIR}...${NC}"
    mv "$BINARY_NAME" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

    # Cleanup
    cd -
    rm -rf "$TMPDIR"
}

# Check if install directory is in PATH
check_path() {
    if [[ ":$PATH:" == *":${INSTALL_DIR}:"* ]]; then
        echo -e "${GREEN}${BINARY_NAME} has been installed to ${INSTALL_DIR}${NC}"
        echo -e "${GREEN}The directory is already in your PATH${NC}"
    else
        echo -e "${YELLOW}${BINARY_NAME} has been installed to ${INSTALL_DIR}${NC}"
        echo -e "${YELLOW}Please add the following to your shell profile:${NC}"
        echo -e "${YELLOW}  export PATH=\"${INSTALL_DIR}:\$PATH\"${NC}"
    fi
}

# Print usage
print_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -v, --version VERSION    Specific version to install (default: latest)"
    echo "  -d, --dir DIRECTORY      Installation directory (default: ~/.local/bin)"
    echo "  -h, --help              Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                      # Install latest version"
    echo "  $0 -v v0.1.0           # Install specific version"
    echo "  $0 -d /usr/local/bin   # Install to system directory"
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -d|--dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            print_usage
            exit 1
            ;;
    esac
done

# Main execution
detect_platform
download_and_install
check_path

echo ""
echo -e "${GREEN}Installation complete!${NC}"
echo -e "Run: ${GREEN}${BINARY_NAME} --version${NC} to verify."
