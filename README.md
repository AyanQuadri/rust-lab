# Rust Lab 🦀

A collection of Rust learning exercises and experiments organized by topics.

## Getting Started

### Installation

1. Install Rust using Homebrew:
```bash
brew install rustup
```

2. Initialize Rust toolchain:
```bash
rustup-init
```
This installs Rust, Cargo, and other essential tools.

3. Create the basics project:
```bash
cargo new basics
cd basics
```

## Basics Module

The `basics` crate contains fundamental Rust programming examples:

### Available Programs

Run any program using: `cargo run --bin <program_name>`

| Program | Description | Command |
|---------|-------------|---------|
| `hello_world` | Classic first program | `cargo run --bin hello_world` |
| `variable_mutability` | Understanding mut keyword and immutability | `cargo run --bin variable_mutability` |
| `constant` | Constants vs variables | `cargo run --bin constant` |
| `shadowing` | Variable shadowing concepts | `cargo run --bin shadowing` |
| `scalar_datatype` | Primitive data types (integers, floats, bool, char) | `cargo run --bin scalar_datatype` |
| `compound_datatype` | Compound types (tuples, arrays) | `cargo run --bin compound_datatype` |
| `conditional` | if/else expressions | `cargo run --bin conditional` |
| `loop` | Basic loop constructs | `cargo run --bin loop` |
| `for_while` | for and while loops | `cargo run --bin for_while` |
| `loop_labels` | Breaking from nested loops | `cargo run --bin loop_labels` |
| `match_statement` | Pattern matching | `cargo run --bin match_statement` |
| `macros` | Introduction to macros | `cargo run --bin macros` |
| `practice_lab` | Practice exercises v1 | `cargo run --bin practice_lab` |
| `practice_lab_v2` | Practice exercises v2 | `cargo run --bin practice_lab_v2` |
