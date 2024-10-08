<div align="center">

![Shuru Logo](shuru.svg)

# <span style="font-family: 'Arial', sans-serif;">Shuru</span>

A task runner and version manager for Node.js and Python, written in Rust! Shuru simplifies your development workflow by automating tasks and managing language versions.

[![Version](https://img.shields.io/badge/version-0.0.18-blue)](https://github.com/shuru-project/shuru/releases)
[![License](https://img.shields.io/badge/license-MIT-lightgrey)](https://opensource.org/licenses/MIT)
[![CI Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/shuru-project/shuru/actions)
[![Contributors](https://img.shields.io/badge/contributors-5-orange)](https://github.com/shuru-project/shuru/graphs/contributors)
[![Stars](https://img.shields.io/github/stars/shuru-project/shuru?style=social)](https://github.com/shuru-project/shuru/stargazers)
[![Forks](https://img.shields.io/github/forks/shuru-project/shuru?style=social)](https://github.com/shuru-project/shuru/network/members)
[![Open Issues](https://img.shields.io/github/issues/shuru-project/shuru)](https://github.com/shuru-project/shuru/issues)
[![Hacktoberfest](https://img.shields.io/badge/Hacktoberfest-2024-brightgreen)](https://github.com/shuru-project/shuru/discussions/10)


**Join us in [Hacktoberfest](https://github.com/shuru-project/shuru/discussions/10) and contribute to open source!**

</div>

## 📚 Table of Contents

- [🌟 Introduction](#-introduction)
- [🚀 Installation](#-installation)
- [📚 Usage](#-usage)
- [🛠️ Detailed Examples](#-detailed-examples)
  - [Node.js Project](#nodejs-project)
  - [Python Project](#python-project)
- [🤝 Community](#-community)
- [📄 License](#-license)

## 🌟 Introduction

Shuru enhances productivity by offering:

- **🔧 Task Automation**: Define and run tasks effortlessly.
- **🌐 Version Management**: Built-in Node.js and Python version management.
- **💻 Shell Completions**: Enjoy auto-completion in Bash, Zsh, and Fish.

## 🚀 Installation

### Linux and macOS

To install the `shuru` CLI, run:

```bash
sh -c "$(curl -fsSL https://raw.githubusercontent.com/shuru-project/shuru/main/install.sh)"
```

## 📚 Usage

1. **Create a `shuru.toml` File**: Define tasks and versions in the file at the project root.

   ### Example Configuration

   ```toml
   [versions]
   node = "v16.14.0"

   [tasks.setup]
   command = "npm install"

   [tasks.dev]
   command = "npm run dev"
   ```

2. **Run Tasks**: Execute defined tasks using:

   ```bash
   shuru <COMMAND>
   ```

   Example:

   ```bash
   shuru setup
   ```

## 🛠️ Detailed Examples

### Node.js Project

1. Set up a new project and create `shuru.toml` as above.
2. Install dependencies:

   ```bash
   shuru setup
   ```

3. Start development:

   ```bash
   shuru dev
   ```

### Python Project

1. Create a `shuru.toml` for your Python project:

   ```toml
   [versions]
   python = "3.9.5"

   [tasks.install]
   command = "pip install -r requirements.txt"

   [tasks.run]
   command = "python main.py"
   ```

2. Install dependencies:

   ```bash
   shuru install
   ```

3. Run your application:

   ```bash
   shuru run
   ```

## 🤝 Community

Join our community for support and discussions:  

[![Discord](https://img.shields.io/badge/Join%20Discord-7289DA?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/EtZn7EdDdS)

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🤗 Contributing

We welcome contributions! Please check our [Contributing Guidelines](CONTRIBUTING.md) for more information on how to get involved.
