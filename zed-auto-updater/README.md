# Zed Auto Updater

This project is an auto updater for the Zed application on Windows. It is designed to check for updates, download the latest version, and install it seamlessly without requiring the user to manually rebuild the application.

## Features

- Automatically checks for updates from the GitHub repository.
- Downloads the latest release of the Zed application.
- Installs updates while verifying the integrity of the downloaded files.
- Provides a user-friendly interface to display update progress.

## Project Structure

```
zed-auto-updater
├── src
│   ├── main.rs          # Entry point of the application
│   ├── updater          # Module for updater functionality
│   │   ├── mod.rs
│   │   ├── github.rs    # Interacts with GitHub API
│   │   ├── installer.rs  # Handles installation of updates
│   │   └── version.rs    # Versioning logic
│   ├── ui               # Module for user interface components
│   │   ├── mod.rs
│   │   └── progress.rs   # Manages progress display
│   └── utils            # Module for utility functions
│       ├── mod.rs
│       └── file_helper.rs # Helper functions for file operations
├── resources
│   └── app.manifest     # Application metadata
├── Cargo.toml           # Rust project configuration
├── build.rs             # Build script for customization
└── README.md            # Documentation for the project
```

## Setup Instructions

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/zed-auto-updater.git
   cd zed-auto-updater
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the application:
   ```
   cargo run --release
   ```

## Usage

Upon running the application, it will automatically check for updates. If a new version is available, it will download and install it, providing progress feedback throughout the process.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.