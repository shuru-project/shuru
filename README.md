<div align="center">

![Shuru Logo](pages/shuru.svg)

# <span style="font-family: 'Arial', sans-serif;">Shuru</span>

A task runner and version manager for Node.js, written in Rust! Shuru simplifies your development workflow by automating tasks and managing language versions.

[![Version](https://img.shields.io/badge/version-0.0.27-blue)](https://github.com/shuru-project/shuru/releases)
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
- [🤝 Community](#-community)
- [📄 License](#-license)
- [🤗 Contributing](#-contributing)

## 🌟 Introduction

Shuru enhances productivity by offering:

- **🔧 Task Automation**: Define and run tasks effortlessly.
- **🌐 Version Management**: Built-in Node.js version management.
- **💻 Shell Completions**: Enjoy auto-completion in Bash, Zsh, and Fish.

## 🚀 Installation

### Linux and macOS

To install the `shuru` CLI, run:

```bash
sh -c "$(curl -fsSL https://shuru.run/install.sh)"
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

## 🤝 Community

Join our community for support and discussions:  

[![Discord](https://img.shields.io/badge/Join%20Discord-7289DA?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/EtZn7EdDdS)

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🤗 Contributing

We welcome contributions! Please check our [Contributing Guidelines](CONTRIBUTING.md) for more information on how to get involved.
