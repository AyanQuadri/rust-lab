# Rust Lab 🦀

A comprehensive collection of Rust learning exercises and experiments organized by topics. This repository progresses from fundamental concepts to advanced topics like ownership, collections, error handling, and package management.

## 📚 Table of Contents

- [Getting Started](#getting-started)
- [Basics Module](#basics-module)
- [Functions Module](#functions-module)
- [Ownership Module](#ownership-module)
- [Collections Module](#collections-module)
- [Errors Module](#errors-module)
- [Package Module](#package-module)

---

## Getting Started

### Installation

1. **Install Rust using Homebrew:**

```bash
brew install rustup
```

2. **Initialize Rust toolchain:**

```bash
rustup-init
```

This installs Rust, Cargo, and other essential tools.

3. **Navigate to any module:**

```bash
cd basics  # or functions, ownership, collections, errors, package
```

### Adding External Libraries

To add external crates (libraries) to your project:

1. **Add dependency to Cargo.toml:**

```toml
[dependencies]
rand = "0.8"
env_logger = "0.11"
```

2. **Or use cargo add (Rust 1.62+):**

```bash
cargo add rand
cargo add env_logger
```

3. **Import and use in your code:**

```rust
use rand::Rng;
use env_logger;
```

### Cleaning Cargo Cache

Over time, Cargo accumulates build artifacts and downloaded dependencies. To clean them:

**Option 1: Clean project-specific cache**

```bash
# From within a project directory
cargo clean
```

**Option 2: Clean global Cargo cache**

```bash
# Install cargo-cache tool
cargo install cargo-cache

# Clean all cache with auto-remove
cargo cache -a

# Or manually remove cache directories
rm -rf ~/.cargo/registry
rm -rf ~/.cargo/git
```

**Option 3: Clean specific module**

```bash
# Remove target directory in specific module
cd basics  # or any module
rm -rf target/
```

---

## Basics Module

The `basics` crate contains fundamental Rust programming examples covering variables, data types, control flow, and pattern matching.

### Available Programs

Run any program using: `cargo run --bin <program_name>`

| Program               | Description                                         | Command                               |
| --------------------- | --------------------------------------------------- | ------------------------------------- |
| `hello_world`         | Classic first program                               | `cargo run --bin hello_world`         |
| `variable_mutability` | Understanding mut keyword and immutability          | `cargo run --bin variable_mutability` |
| `constant`            | Constants vs variables                              | `cargo run --bin constant`            |
| `shadowing`           | Variable shadowing concepts                         | `cargo run --bin shadowing`           |
| `scalar_datatype`     | Primitive data types (integers, floats, bool, char) | `cargo run --bin scalar_datatype`     |
| `compound_datatype`   | Compound types (tuples, arrays)                     | `cargo run --bin compound_datatype`   |
| `conditional`         | if/else expressions                                 | `cargo run --bin conditional`         |
| `loop`                | Basic loop constructs                               | `cargo run --bin loop`                |
| `for_while`           | for and while loops                                 | `cargo run --bin for_while`           |
| `loop_labels`         | Breaking from nested loops                          | `cargo run --bin loop_labels`         |
| `match_statement`     | Pattern matching                                    | `cargo run --bin match_statement`     |
| `macros`              | Introduction to macros                              | `cargo run --bin macros`              |
| `practice_lab`        | Practice exercises v1                               | `cargo run --bin practice_lab`        |
| `practice_lab_v2`     | Practice exercises v2                               | `cargo run --bin practice_lab_v2`     |

---

## Functions Module

The `functions` crate explores function definitions, parameters, return values, and the distinction between statements and expressions.

### Available Programs

| Program                  | Description                             | Command                                  |
| ------------------------ | --------------------------------------- | ---------------------------------------- |
| `function`               | Basic function definitions              | `cargo run --bin function`               |
| `parameters_functions`   | Functions with parameters               | `cargo run --bin parameters_functions`   |
| `statements_expressions` | Understanding statements vs expressions | `cargo run --bin statements_expressions` |
| `return_value`           | Functions returning values              | `cargo run --bin return_value`           |
| `practice`               | Practice exercises                      | `cargo run --bin practice`               |

---

## Ownership Module

The `ownership` crate covers Rust's ownership system, borrowing, references, and slices—the core concepts that make Rust memory-safe.

### Available Programs

| Program                | Description                          | Command                                |
| ---------------------- | ------------------------------------ | -------------------------------------- |
| `scope_ownership`      | Variable scope and ownership basics  | `cargo run --bin scope_ownership`      |
| `string_type`          | String vs &str types                 | `cargo run --bin string_type`          |
| `data_interactions`    | Move semantics and data interactions | `cargo run --bin data_interactions`    |
| `cloning`              | Deep copying with clone              | `cargo run --bin cloning`              |
| `functions_ownership`  | Ownership with function calls        | `cargo run --bin functions_ownership`  |
| `references_borrowing` | Borrowing and references             | `cargo run --bin references_borrowing` |
| `mutability`           | Mutable references                   | `cargo run --bin mutability`           |
| `rules_of_references`  | Reference rules and restrictions     | `cargo run --bin rules_of_references`  |
| `slice_basics`         | Introduction to slices               | `cargo run --bin slice_basics`         |
| `slice_rules`          | Slice rules and best practices       | `cargo run --bin slice_rules`          |
| `time`                 | Time-related examples                | `cargo run --bin time`                 |
| `practice`             | Practice exercises                   | `cargo run --bin practice`             |

---

## Collections Module

The `collections` crate demonstrates Rust's standard collections: vectors, hashmaps, and their interactions with structs and enums.

### Available Programs

#### Vectors

| Program                | Description                  | Command                                |
| ---------------------- | ---------------------------- | -------------------------------------- |
| `vectors`              | Vector basics                | `cargo run --bin vectors`              |
| `common_vector_method` | Common vector methods        | `cargo run --bin common_vector_method` |
| `iteration_vector`     | Iterating over vectors       | `cargo run --bin iteration_vector`     |
| `ownership_vectors`    | Ownership rules with vectors | `cargo run --bin ownership_vectors`    |

#### HashMaps

| Program              | Description                    | Command                              |
| -------------------- | ------------------------------ | ------------------------------------ |
| `hashmaps_basics`    | HashMap fundamentals           | `cargo run --bin hashmaps_basics`    |
| `hashmaps_more`      | Advanced HashMap operations    | `cargo run --bin hashmaps_more`      |
| `iteration_hashmaps` | Iterating over hashmaps        | `cargo run --bin iteration_hashmaps` |
| `ownership_hashmaps` | Ownership with hashmaps        | `cargo run --bin ownership_hashmaps` |
| `vector_to_hashmap`  | Converting vectors to hashmaps | `cargo run --bin vector_to_hashmap`  |

#### Structs & Enums

| Program              | Description                | Command                              |
| -------------------- | -------------------------- | ------------------------------------ |
| `struct`             | Struct basics              | `cargo run --bin struct`             |
| `struct_printing`    | Debug printing structs     | `cargo run --bin struct_printing`    |
| `struct_ownership`   | Ownership in structs       | `cargo run --bin struct_ownership`   |
| `struct_non_copy`    | Non-copyable struct types  | `cargo run --bin struct_non_copy`    |
| `method`             | Methods and impl blocks    | `cargo run --bin method`             |
| `enums`              | Enum fundamentals          | `cargo run --bin enums`              |
| `enums_with_data`    | Enums containing data      | `cargo run --bin enums_with_data`    |
| `enums_v3`           | Advanced enum patterns     | `cargo run --bin enums_v3`           |
| `enum_with_function` | Enums with methods         | `cargo run --bin enum_with_function` |
| `enum_in_struct`     | Using enums in structs     | `cargo run --bin enum_in_struct`     |
| `options_enum`       | Option<T> enum             | `cargo run --bin options_enum`       |
| `file_error`         | File operations and errors | `cargo run --bin file_error`         |
| `practice`           | Practice exercises         | `cargo run --bin practice`           |

---

## Errors Module

The `errors` crate covers Rust's error handling mechanisms, including Result, Option, panic, and custom error types.

### Available Programs

| Program                | Description                       | Command                                |
| ---------------------- | --------------------------------- | -------------------------------------- |
| `unrecoverable_errors` | panic! and unrecoverable errors   | `cargo run --bin unrecoverable_errors` |
| `recoverable_errors`   | Result<T, E> and error handling   | `cargo run --bin recoverable_errors`   |
| `operator_errors`      | Error propagation with ? operator | `cargo run --bin operator_errors`      |
| `unwrap_or_else`       | Error recovery methods            | `cargo run --bin unwrap_or_else`       |
| `custom_error`         | Creating custom error types       | `cargo run --bin custom_error`         |
| `logging_libraries`    | Using env_logger for logging      | `cargo run --bin logging_libraries`    |
| `practice`             | Practice exercises                | `cargo run --bin practice`             |

### External Dependencies

This module uses:

- **`env_logger`**: Environment-based logging configuration

---

## Package Module

The `package` crate demonstrates Rust's module system, workspaces, and external dependencies.

### Structure

```
package/
├── src/
│   ├── bin/
│   │   ├── math/           # Math module examples
│   │   └── practice.rs     # Uses rand crate
│   └── main.rs
└── text_library/           # Custom library crate
    └── src/
        ├── lib.rs
        └── maths.rs
```

### Available Programs

| Program    | Description              | Command                    |
| ---------- | ------------------------ | -------------------------- |
| `math`     | Math module examples     | `cargo run --bin math`     |
| `practice` | Practice with rand crate | `cargo run --bin practice` |

### External Dependencies

This module uses:

- **`rand`**: Random number generation

### Running the Main Package

```bash
cd package
cargo run
```

---

## 🎯 Learning Path

1. **Start with Basics** - Learn syntax, data types, and control flow
2. **Functions** - Understand function concepts and expressions
3. **Ownership** - Master Rust's unique ownership system
4. **Collections** - Work with vectors, hashmaps, structs, and enums
5. **Errors** - Handle errors gracefully
6. **Package** - Learn module organization and external dependencies

---

## 📝 Notes

- Each module is a separate Cargo workspace
- Binary examples are located in `src/bin/` directories
- Run examples from within their respective module directories
- The `target/` directories contain compiled artifacts (not tracked in git)

---

## 🚀 Quick Reference

### Build a module

```bash
cargo build
```

### Run a specific binary

```bash
cargo run --bin <binary_name>
```

### Check code without building

```bash
cargo check
```

### Run with optimizations

```bash
cargo run --release --bin <binary_name>
```

---

## 📖 Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

---

**Happy Learning! 🦀**
