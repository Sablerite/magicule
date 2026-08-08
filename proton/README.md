# Proton

A Rust library for converting inches to meters with a command-line interface.

## Overview

Proton provides core functionality for converting measurements between inches and meters, featuring:

- Safe conversion with comprehensive error handling
- Library-focused design with thin CLI wrapper
- Unit tests, integration tests, and documentation tests
- Benchmarking support using Criterion
- Examples demonstrating usage

## Directory Structure

```
proton/
├── Cargo.toml          # Manifest with dependencies and metadata
├── Cargo.lock          # Dependency lock file (not committed for libraries)
├── src/
│   ├── lib.rs          # Library entry point
│   ├── conversion.rs   # Core conversion logic and error types
│   └── main.rs         # Thin binary that calls into the library
├── tests/
│   └── integration_test.rs  # Integration tests
├── benches/
│   └── benchmark_conversion.rs  # Criterion benchmarks
├── examples/
│   └── simple_convert.rs      # Runnable example
├── .gitignore          # Git ignore rules
├── rustfmt.toml        # Rust formatter configuration
�└── README.md           # This file
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
proton = { git = "https://github.com/sablerite/proton" }
```

Or use the published crate when available:

```toml
[dependencies]
proton = "0.1.0"
```

## Usage

### As a Library

```rust
use proton::convert_inches_to_meters;

fn main() {
    match convert_inches_to_meters(39.3701) {
        Ok(meters) => println!("39.3701 inches is {} meters", meters),
        Err(e) => eprintln!("Conversion failed: {}", e),
    }
}
```

### Command Line Interface

```bash
cargo run
```

The CLI will prompt you to enter inch values and convert them to meters. Enter 'q' to quit.

## Building

```bash
cargo build
```

For release builds:

```bash
cargo build --release
```

## Testing

Run all tests:

```bash
cargo test
```

Run only unit tests:

```bash
cargo test --lib
```

Run only integration tests:

```bash
cargo test --tests
```

## Documentation

View documentation locally:

```bash
cargo doc --open
```

The library includes documentation tests to ensure examples remain valid.

## Examples

Run the provided example:

```bash
cargo run --example simple_convert
```

## Benchmarks

Run performance benchmarks using Criterion:

```bash
cargo bench
```

This will execute the benchmarks in the `benches/` directory and generate reports in `target/criterion`.

## Features

- **Safe Conversion**: Comprehensive error handling for invalid inputs
- **Library-First Design**: Core functionality in `lib.rs` with thin `main.rs`
- **Well Tested**: Unit tests, integration tests, and doc tests
- **Benchmark Ready**: Criterion integration for performance testing
- **Well Documented**: Full API documentation with examples
- **Modern Rust**: Uses 2021 edition and current best practices

## Error Handling

The library defines specific error types for different failure conditions:

- `NegativeValue`: Input is less than zero
- `TooLarge`: Input exceeds maximum allowed value (1,000,000 inches)
- `NotFinite`: Input is NaN, infinity, or negative infinity

The CLI uses `anyhow` for application-level error propagation.

## License

This project is licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Feel free to submit issues or pull requests to improve the project.

## Acknowledgments

- Built with Rust �� 🦀
- Uses the `thiserror` crate for error definitions
- Uses the `anyhow` crate for application error handling
- Uses `criterion` for benchmarking