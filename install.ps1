<#
.SYNOPSIS
    Install Verse MCP Server for Windows

.DESCRIPTION
    Downloads and installs the latest version of vm (Verse MCP Server) for Windows.
    Automatically detects architecture and installs to user's local bin directory.
#>

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

# Configuration
$Repo = "quangdang46/verse-mcp"
$BinaryName = "vm"
$Version = if ($env:VERSION) { $env:VERSION } else { "latest" }
$InstallDir = if ($env:INSTALL_DIR) { $env:INSTALL_DIR } else { "$env:USERPROFILE\.local\bin" }

# Colors
$ErrorColor = "Red"
$SuccessColor = "Green"
$WarningColor = "Yellow"
$InfoColor = "White"

function Write-ColorOutput {
    param([string]$Message, [string]$Color = "White")
    Write-Host $Message -ForegroundColor $Color
}

function Get-ArtifactName {
    $Arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
    "vm-${Arch}-windows"
}

function Resolve-Version {
    if ([string]::IsNullOrWhiteSpace($Version) -or $Version -eq "latest") {
        # Try GitHub API first
        try {
            $headers = @{ "Accept" = "application/vnd.github+json"; "User-Agent" = "verse-mcp-installer" }
            $latest = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest" -Headers $headers -ErrorAction Stop
            $Version = $latest.tag_name
        } catch {
            $Version = $null
        }
        # Fallback: follow GitHub redirect from /releases/latest to /tag/vX.Y.Z
        if ([string]::IsNullOrWhiteSpace($Version)) {
            try {
                $resp = Invoke-WebRequest -Uri ("https://github.com/" + $Repo + "/releases/latest") -MaximumRedirection 0 -Method Head -UseBasicParsing -ErrorAction Stop
            } catch {
                $resp = $_.Exception.Response
            }
            $loc = $null
            if ($resp -and $resp.Headers) {
                if ($resp.Headers["Location"]) { $loc = $resp.Headers["Location"] }
                elseif ($resp.Headers.Location) { $loc = $resp.Headers.Location }
            }
            if ($loc) {
                $m = [regex]::Match($loc, "/tag/(?<tag>.+)$")
                if ($m.Success) { $Version = $m.Groups["tag"].Value }
            }
        }
        if ([string]::IsNullOrWhiteSpace($Version) -or $Version -eq "latest") {
            Write-ColorOutput "Could not resolve latest release tag automatically. Re-run with: -Version v0.1.0 or set $env:VERSION" $ErrorColor
            exit 1
        }
    }
}

function Download-And-Install {
    $Artifact = Get-ArtifactName
    # Build download URL: use releases/latest/download when Version is 'latest' or unresolved
    if ([string]::IsNullOrWhiteSpace($Version) -or $Version -eq "latest") {
        $ZipUrl = "https://github.com/$Repo/releases/latest/download/$Artifact.zip"
    } else {
        if ($Version -notmatch '^v') { $Version = 'v' + $Version }
        $ZipUrl = "https://github.com/$Repo/releases/download/$Version/$Artifact.zip"
    }

    Write-ColorOutput "Downloading ${Artifact} from ${ZipUrl}..." $SuccessColor

    # Create temp directory
    $TmpDir = Join-Path $env:TEMP "vm-install-$([guid]::NewGuid())"
    New-Item -ItemType Directory -Path $TmpDir -Force | Out-Null
    Push-Location $TmpDir

    try {
        # Download (use Invoke-WebRequest for -OutFile compatibility on Windows PowerShell 5.1)
        Invoke-WebRequest -Uri $ZipUrl -OutFile "vm.zip" -UseBasicParsing

        # Extract
        Expand-Archive -Path "vm.zip" -DestinationPath "." -Force

        # Create install directory
        if (-not (Test-Path $InstallDir)) {
            New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        }

        # Move binary
        Write-ColorOutput "Installing ${BinaryName} to ${InstallDir}..." $SuccessColor
        Move-Item "${BinaryName}.exe" "${InstallDir}\${BinaryName}.exe" -Force

    } finally {
        Pop-Location
        Remove-Item $TmpDir -Recurse -Force
    }
}

function Show-PathInstructions {
    $PathEnv = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($PathEnv -notlike "*$InstallDir*") {
        Write-ColorOutput "`n${BinaryName} has been installed to ${InstallDir}" $WarningColor
        Write-ColorOutput "To add it to your current session PATH, run:" $InfoColor
        Write-ColorOutput ('$env:Path += ";' + $InstallDir + '"') $InfoColor
        Write-ColorOutput "`nOr add it permanently:" $InfoColor
        Write-ColorOutput ('[Environment]::SetEnvironmentVariable("Path", "' + $env:Path + ';' + $InstallDir + '", "User")') $InfoColor
    } else {
        Write-ColorOutput "`n${BinaryName} has been installed to ${InstallDir}" $SuccessColor
        Write-ColorOutput "The directory is already in your PATH" $SuccessColor
    }
}

function Show-Help {
    Write-Host "Usage: .\install.ps1 [OPTIONS]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -Version VERSION   Specific version to install (default: latest)"
    Write-Host "  -Dir DIRECTORY     Installation directory (default: ~/.local/bin)"
    Write-Host "  -Help              Show this help message"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host "  .\install.ps1                    Install latest version"
    Write-Host "  .\install.ps1 -Version v0.1.0    Install specific version"
    Write-Host "  .\install.ps1 -Dir C:\Tools      Install to custom directory"
}

function Ensure-ArchiveSupport {
    if (-not (Get-Command Expand-Archive -ErrorAction SilentlyContinue)) {
        Write-ColorOutput "This script requires PowerShell 5.0 or later" $ErrorColor
        Write-ColorOutput "Please update PowerShell or download the binary manually from GitHub Releases" $ErrorColor
        exit 1
    }
}

# Parse arguments
for ($i = 0; $i -lt $args.Count; $i++) {
    switch ($args[$i]) {
        "-Version" {
            $Version = $args[$i + 1]
            $i++
        }
        "-Dir" {
            $InstallDir = $args[$i + 1]
            $i++
        }
        "-Help" {
            Show-Help
            exit 0
        }
        default {
            Write-ColorOutput "Unknown option: $($args[$i])" $ErrorColor
            Show-Help
            exit 1
        }
    }
}

# Main
Ensure-ArchiveSupport
Resolve-Version
Download-And-Install
Show-PathInstructions

Write-ColorOutput "`nInstallation complete!" $SuccessColor
Write-ColorOutput "Run: vm --version to verify." $SuccessColor

# Refresh PATH for current session
$env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
