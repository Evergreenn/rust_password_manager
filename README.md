[![Rust](https://github.com/Evergreenn/rust_password_manager/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/Evergreenn/rust_password_manager/actions/workflows/rust.yml)

# Rust Password Manager (TUI App)

This is a terminal user interface (TUI) password manager written in Rust. It allows users to securely store and manage their passwords using an interactive command-line interface. The project utilizes various Rust libraries and dependencies to provide a reliable and efficient password management solution.

## Features

  - [ ] Securely store and manage passwords
  - [X] Generate strong and unique passwords
  - [X] Copy passwords to the clipboard
  - [ ] Search and retrieve passwords
  - [ ] Import and export passwords
  - [ ] Password strength analysis

## Prerequisites

Before running the Rust Password Manager, ensure that you have the following prerequisites installed on your system:

  - Rust (stable version recommended)
  - Cargo (Rust's package manager)

## Installation

1. Clone this repository to your local machine:

``` git clone https://github.com/Evergreenn/rust_password_manager.git ```

2. Change into the project directory:

``` cd rust-password-manager ```

3. Build the project using Cargo:

``` cargo build --release ```

4. Run the password manager:

``` cargo run --release ```

## Usage

The Rust Password Manager provides an interactive TUI for managing your passwords. Once you run the application, you can navigate through the interface using arrow keys and interact with the menus and options.

Here are some common actions:

- Use arrow keys to navigate through menus and options
- Press Enter to select an option or confirm an action
- Press Esc or q to exit or go back

For more detailed information on how to use the TUI app, please refer to the [user manual](user-manual.md).

## Dependencies

The Rust Password Manager project relies on the following dependencies:

- log = "0.4"
- ratatui = { version = "0.21.0", features = ["all-widgets"]}
- crossterm = "0.26.1"
- tokio = { version = "1", features = ["full"] }
- eyre = "0.6.8"
- tui-logger = { version = "0.9.2", features = ["ratatui-support"], default_features = false}
- rusqlite = { version = "0.29.0", features =["bundled", "chrono", "serde_json", "uuid"]}
- passwords = "3.1.13"
- chrono = "0.4.26"
- arboard = "3.2.0"
- uuid = {version = "1.3.3", features = ["serde", "v4"]}

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE) or [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0).
