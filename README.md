# suffixsort - Inverse Lexicographic Sorter

A high-performance Rust workspace containing both a command-line tool and library for inverse lexicographic (suffix) sorting.

## Workspace Structure

This workspace contains two crates:

### 1. suffixsort (Library Crate)
A high-performance library for inverse lexicographic sorting that provides:
- Core sorting algorithms
- Flexible configuration options
- Parallel processing capabilities
- Both high-level and low-level APIs

**Location**: `core/`

### 2. ssort (Binary Crate)
A command-line interface that uses the suffixsort library to provide:
- File and stdin input handling
- Multiple sorting options and output formatting
- High-performance processing of large files

**Location**: `cli/`

## Getting Started

### Prerequisites
- Rust toolchain (latest stable version recommended)
- Cargo (comes with Rust)

### Building

```bash
# Build both crates
cargo build

# Build in release mode (optimized)
cargo build --release
```

### Using the CLI Tool

```bash
# Install the ssort tool globally
cargo install --path cli

# Basic usage
ssort input.txt

# With various options
ssort -air file1.txt file2.txt

# Read from stdin
cat large_file.txt | ssort -i
```

### Using the Library

Add to your `Cargo.toml`:
```toml
[dependencies]
suffixsort = ">=0.1"
```

Example usage:
```rust
use suffixsort::SortConfig;

let config = SortConfig {
    ignore_case: true,
    reverse: false,
    ..Default::default()
};

let lines = vec!["Banana".to_string(), "apple".to_string(), "Cherry".to_string()];
let (sorted, _) = config.process_lines(lines);

for item in sorted {
    println!("{}", item.original);
}
```

## Performance

This workspace is optimized for processing large datasets:
- Parallel processing using Rayon
- Efficient memory management
- Zero-cost abstractions
- Streaming input/output support

## License

Dual-licensed under:
- MIT License (see LICENSE-MIT)
- Apache License 2.0 (see LICENSE-APACHE)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Run tests with `cargo test`
6. Submit a pull request

For more detailed information about each crate, see their individual README files.
