<div align="center">

![Shuru Logo](shuru.svg)

</div>

# Shuru
A Basic Command/Task Runner Written in Rust

## Installation

### Linux and macOS

Run the following command to install the `shuru` CLI on your system:

```bash
curl -s https://raw.githubusercontent.com/harshdoesdev/shuru/main/install.sh | sh
```

## Features
- Basic task runner
- Built-in Node Version Manager

## Usage

1. Create a `shuru.toml` file in the root of your project to define tasks.
2. Run tasks using the following command:

```bash
shuru <COMMAND>
```

Replace `<COMMAND>` with the name of the task you've defined in your `shuru.toml` file.

## Examples

You can explore the `examples` directory for more examples. Below is a simple example for a Node.js project:

```toml
[versions]
node = "v16.14.0"

[[task]]
name = "setup"
command = "npm i"

[[task]]
name = "dev"
command = "npm run dev"
default = true # This command can be run by just typing "shuru"

[[task]]
name = "build"
command = "npm run build"
```
