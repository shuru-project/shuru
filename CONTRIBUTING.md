# Contributing to Shuru

Thank you for your interest in contributing to Shuru! We welcome contributions that improve the project, whether it’s bug fixes, new features, or documentation.

## How to Contribute

### 1. Fork the Repository

Start by [forking the repository](https://github.com/shuru-project/shuru) and cloning it locally:

```bash
git clone https://github.com/your-username/shuru.git
cd shuru
```

### 2. Setting Up the Development Environment

To set up your development environment:

1. Install [Rust](https://www.rust-lang.org/).
2. Run `cargo build` to build the project or `cargo install --path .` to install it.
3. We encourage you to use the editor of your choice. Popular options include:
   - **VS Code** with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) plugin.
   - **IntelliJ IDEA** with the Rust plugin.
   - **RustRover**, a dedicated Rust IDE from JetBrains.
   - **Neovim** with `rust-tools.nvim` for LSP support.

### 3. Running the Examples

To test the functionality, run the examples provided in the `examples` folder. Each example demonstrates how Shuru can be used as a task runner or version manager.

You can run an example as follows:

```bash
cd examples/simple-node-app
shuru setup
shuru dev
```

### 4. Submitting Your Changes

Once your changes are ready, push them to your fork and create a pull request with a detailed description. Be sure to reference any relevant issues.

### 5. Code Style

Please ensure that your code follows the Rust style conventions. You can use the following commands to maintain code quality:

- To auto-format your code:

    ```bash
    cargo fmt
    ```

- To check for common mistakes and ensure best practices, use the **Clippy** linter:

    ```bash
    cargo clippy
    ```

### 6. Git Commit Style

When submitting pull requests, please follow the convention of prefixing commit messages with one of the following tags:

- `feat:` for new features (e.g., `feat: add a new cli command for autocompletions`).
- `fix:` for bug fixes (e.g., `fix: resolve issue with Node Version Manager`).
- `chore:` for minor tasks like refactoring or updating dependencies (e.g., `chore: update dependencies`).
- `docs:` for documentation updates (e.g., `docs: update contributing guidelines`).
- `refactor:` for code restructuring without changing functionality (e.g., `refactor: optimize setup flow`).

Keeping this structure makes it easier to understand the scope of changes.

### 7. Issues

If you encounter any bugs or have feature requests, please open an issue on GitHub and provide as much detail as possible.

---

We look forward to your contributions!
