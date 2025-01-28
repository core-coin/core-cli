# Core CLI

The **Core CLI** is a command-line interface for interacting with the Core Blockchain. It allows you to manage accounts, send transactions, deploy smart contracts, and perform other operations directly from your terminal. Designed to operate even in decentralized environments, it can connect to nodes offline using Lunaº Mesh technology.

## Install Rust (Optional)

Before installing the Core CLI, you can optionally install Rust if you plan to build from source or use custom Rust-based features.

1. Install Rust using `rustup` (recommended installer)

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Follow the on-screen instructions to complete the installation.

3. Verify the installation

   ```bash
   rustc --version
   ```

If you don’t need to build from source, you can skip this step and directly install the Core CLI.

## Installing Core CLI

Install the Core CLI by running this command in your terminal:

```bash
bash -c "$(curl -fsSL https://raw.githubusercontent.com/core-coin/core-cli/refs/heads/master/install)"
```

### What this does

- Downloads the Core CLI binaries.
- Sets up environment variables for `core-cli`.
- Provides additional setup instructions if required.

## Verify Installation

After installation, check if the Core CLI was installed successfully:

```bash
core-cli --version
```

You should see the installed version printed to the terminal.

## Building Core CLI from Source (Optional)

If you'd like to build Core CLI from the source code:

1. Clone the repository:

   ```bash
   git clone https://github.com/core-coin/core-cli.git
   cd core-cli
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Add the binary to your PATH:

   ```bash
   export PATH="$PWD/target/release:$PATH"
   ```

4. Verify the installation:

   ```bash
   core-cli --version
   ```

## Getting Started with Core CLI

Once installed, you can start using the Core CLI to interact with the Core Blockchain:

- **Check account balance:**

  ```bash
  core-cli account balance <address>
  ```

- **Send transactions:**

  ```bash
  core-cli transaction send <params>
  ```

- **Interact with smart contracts:**

  ```bash
  core-cli contract call <params>
  ```

- **Offline operations via Lunaº Mesh:** Use Core CLI with nodes that operate without an internet connection.

To see all available commands, run:

```bash
core-cli --help
```

## Repository and Contributions

Source code is available on GitHub:
[Core CLI GitHub Repository](https://github.com/core-coin/core-cli)

Feel free to contribute by reporting issues or submitting pull requests.

## License

The Core CLI is licensed under the [CORE License](LICENSE).
