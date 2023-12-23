# ZipRustler

- **ZipRustler**is a Rust-based command-line tool for creating zip backups.
- This tool efficiently archives files, leveraging Rust's powerful features.

## Features

- **Efficient Directory Traversal**: Recursively scans the given directory.
- **Dynamic Backup Naming**: Auto-generates backup file names.
- **Progress Tracking**: Implements a progress bar using the `indicatif` crate.
- **User-Friendly**: Easy to use, with a default to the current directory.
- **Compression**: Uses zip compression to efficiently store files.

## Getting Started

### Prerequisites

- Rust environment setup.
- Dependencies: `zip`, `chrono`, `dirs`, `indicatif`.

### Installation

1. Clone this repository to your local machine.
2. Ensure Rust and Cargo are correctly installed.
3. Compile the project with `cargo build`.

### Usage

- Run the program with the command:

```shell
cargo run -- [optional_directory_path]
```

- If no directory path is provided, the current working directory will be used.

## Contributing

- Contributions, issues, and feature requests are welcome!
- Feel free to also check the issues page if you want to contribute.

## Support

- If you find this tool helpful and would like to show support/appreciation:

<p>
  <a href="https://www.Patreon.com/SystemSculpt">
    <img
      align="left"
      src="https://indigenousx.com.au/wp-content/uploads/2017/03/patreon-medium-button.png"
      height="50"
      width="210"
      alt="Support on Patreon"
  /></a>
  <a href="https://www.buymeacoffee.com/SystemSculpt">
    <img
      align="left"
      src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png"
      height="50"
      width="210"
      alt="Buy Me A Coffee"
  /></a>
</p>
<br /><br />
