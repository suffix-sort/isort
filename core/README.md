# suffixsort

A high-performance Rust library for inverse lexicographic (suffix) sorting, providing both high-level processing utilities and low-level comparison functions.

## Features

- **Inverse Lexicographic Sorting**: Compare strings from the last character towards the first
- **Flexible Configuration**: Multiple sorting modes including dictionary order, case insensitivity, and reverse sorting
- **High Performance**: Parallel processing using Rayon for handling large datasets efficiently
- **Dual API**: Both high-level line processing and low-level comparator functions
- **Zero-Cost Abstractions**: Minimal performance overhead through Rust's zero-cost abstractions

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
suffixsort = ">=0.1"
```

## Usage

### High-Level API

The main entry point is the `SortConfig` struct which allows you to configure and execute the sorting process:

```rust
use suffixsort::{SortConfig, ProcessedLine};

let config = SortConfig {
    ignore_case: true,
    reverse: false,
    dictionary_order: true,
    // ... other configuration options
    ..Default::default()
};

let lines = vec![
    "Apple".to_string(),
    "banana".to_string(),
    "Cherry".to_string(),
];

let (processed, padding_info) = config.process_lines(lines);

for line in processed {
    println!("{}", line.original);
}
```

### Low-Level API

For advanced use cases, you can use the comparator function directly:

```rust
use suffixsort::SortConfig;
use std::cmp::Ordering;

let config = SortConfig {
    ignore_case: true,
    reverse: false,
    ..Default::default()
};

let comparer = config.get_comparer();
let mut words = vec!["Banana", "apple", "Cherry"];

// Use with standard sort
words.sort_by(|a, b| comparer(a, b));

// Or with parallel sort (requires Rayon)
use rayon::prelude::*;
words.par_sort_by(|a, b| comparer(a, b));
```

## Configuration Options

The `SortConfig` struct provides these options:

- `ignore_case`: Case-insensitive comparison
- `use_entire_line`: Use entire line instead of first word for sorting
- `dictionary_order`: Ignore non-alphabetic characters when finding first word
- `reverse`: Reverse the sort order
- `stable`: Use stable sorting algorithm
- `right_align`: Right-align output with padding
- `exclude_no_word`: Exclude lines without words
- `word_only`: Output only the word used for sorting

## Performance

The library is designed for high performance with large datasets:

- Parallel processing using Rayon's work-stealing scheduler
- Zero-copy operations where possible
- Efficient character-by-character comparison
- Minimal memory allocation

## Examples

See the `ssort` binary crate for a complete CLI implementation using this library.

## License

MIT OR Apache-2.0
