# Zed Windows Auto Updater

![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![Platform: Windows](https://img.shields.io/badge/Platform-Windows-blue.svg)
![Rust Version: 1.68+](https://img.shields.io/badge/Rust-1.68+-orange.svg)

A robust and reliable auto-update solution for the [Zed Editor](https://zed.dev/) on Windows. This utility automatically detects, downloads, and installs updates to ensure Windows users always have the latest Zed version without manual intervention.

## Features

- **Automatic Version Detection**: Compares local Zed installation with the latest GitHub release
- **Smart Download Management**: Downloads only when updates are available with bandwidth-efficient transfers
- **Visual Progress Tracking**: Shows download progress with an attractive console-based progress bar
- **Process Safety**: Checks if Zed is running before updating and offers to close it safely
- **Silent Installation**: Handles installation quietly using NSIS's silent install parameters
- **User Notifications**: Provides both console and native Windows notifications
- **Integrity Verification**: Ensures downloaded files are complete and valid before installation
- **Error Handling**: Robust error reporting and recovery mechanisms

## Requirements

- Windows 10 or newer
- Administrator privileges (for installation)
- Internet connection
- Minimum 100MB free disk space

## Installation

### Download Prebuilt Binary

1. Download the latest release from the [Releases page](https://github.com/ishaanko/zed-windows-auto-updater/releases)
2. Place the executable in your Zed installation directory
3. For auto-startup, create a shortcut in your startup folder:
   - Press `Win+R`, type `shell:startup` and press Enter
   - Create a shortcut to the updater executable

### Build from Source

```bash
# Clone the repository
git clone https://github.com/ishaanko/zed-windows-auto-updater.git

# Build the project
cargo build --release

# The executable will be in target/release/zed-auto-updater.exe
# Note: The official releases rename this to zed-windows-auto-updater.exe
```

## Usage

### Manual Update Check

Simply run the executable to check for and apply updates:

```bash
zed-auto-updater.exe
```

### Silent Mode

Run without notifications (useful for scheduled tasks):

```bash
zed-auto-updater.exe --silent
```

### Update to Specific Version

Force update to a specific version:

```bash
zed-auto-updater.exe --version
```

### Command Line Options

| Option | Description |
|--------|-------------|
| `--silent` | Run without user interface notifications |
| `--version <ver>` | Update to specific version |
| `--check-only` | Only check for updates without installing |
| `--force` | Force update even if latest version is installed |
| `--log <path>` | Write logs to specified file |

## How It Works

1. **Version Check**: The updater reads the current version from `zed-version.txt` and compares it with the latest release from GitHub API
2. **Download Decision**: If a newer version is available, it fetches the Windows installer
3. **Download Process**: Shows a progress bar during download, storing the installer in a temporary directory
4. **Pre-Installation**: Checks if Zed is running and offers to close it automatically
5. **Installation**: Runs the installer with silent parameters (`/S`)
6. **Verification**: Confirms the installation was successful by checking for the existence of key files
7. **Cleanup**: Removes temporary files and updates the local version record

## Architecture

The project is organized into several modules:

- **`updater`**: Core functionality for checking, downloading, and installing updates
  - `github.rs`: Handles GitHub API integration for fetching releases
  - `installer.rs`: Manages the installation process
  - `version.rs`: Handles version comparison logic

- **`ui`**: User interface components
  - `progress.rs`: Manages the download progress display

- **`utils`**: Helper utilities
  - `file_helper.rs`: File system operations
  - `process_helper.rs`: Process management (checking if Zed is running)

## Development

### Prerequisites

- Rust 1.68 or newer
- Cargo package manager
- Windows 10+ development environment

### Project Setup

```bash
git clone https://github.com/ishaanko/zed-windows-auto-updater.git
cargo build
```

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
cargo build --release
```

## Troubleshooting

### Common Issues

| Issue | Solution |
|-------|----------|
| "Access denied" errors | Run the updater with administrator privileges |
| Failed downloads | Check your internet connection or proxy settings |
| Installation freezes | Ensure no other installers are running |
| Update verification fails | Try manually installing the update |

### Logs

Logs are stored in `%APPDATA%\zed-auto-updater\logs` for troubleshooting.

## Contributing

Contributions are welcome! Open PRs for features and bug-fixes!

Please ensure your code follows the project's style guidelines and includes appropriate tests.
