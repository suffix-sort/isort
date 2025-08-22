# suffixsort

A high-performance Rust library for inverse lexicographic (suffix) sorting, providing both high-level processing utilities and low-level comparison functions.

## Features

- **Inverse Lexicographic Sorting**: Compare strings from the last character towards the first
- **Flexible Configuration**: Multiple sorting modes including dictionary order, case insensitivity, and reverse sorting
- **High Performance**: Parallel processing using Rayon for handling large datasets efficiently
- **Dual API**: Both high-level line processing and low-level comparator functions
- **Zero-Cost Abstractions**: Minimal performance overhead through Rust's zero-cost abstractions
- **Optimized Defaults**: Fastest performance with all options disabled by default

## Performance Characteristics

suffixsort is designed for maximum performance with all options disabled (default configuration). The library is extremely fast by default, processing millions of lines per second on modern hardware.

### Performance-Neutral Options
These options have minimal impact on performance:
- `reverse`: Simply inverts comparison results
- `right_align`: Adds padding during output formatting only
- `exclude_no_word`: Simple filtering during processing
- `word_only`: Affects output formatting only

### Performance-Impactful Options
These options may reduce performance when enabled:
- `normalize`: Unicode NFC normalization adds processing overhead during key extraction
- `stable`: Stable sorting algorithms are generally slower than unstable variants
- `ignore_case`: Case folding during comparison adds minor overhead
- `dictionary_order`: More complex key extraction logic
- `use_entire_line`: Simpler key extraction (uses whole line) but may use more memory

For maximum throughput with large datasets, use the default configuration (all options disabled).

## Project Structure

This library is part of a workspace that includes:
- `core/`: The suffixsort library crate (this crate)
- `cli/`: The ssort command-line interface that uses this library

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

- `ignore_case`: Case-insensitive comparison (minor performance impact)
- `use_entire_line`: Use entire line instead of first word for sorting (simpler but may use more memory)
- `dictionary_order`: Ignore non-alphabetic characters when finding first word (performance impact)
- `reverse`: Reverse the sort order (performance-neutral)
- `stable`: Use stable sorting algorithm (performance impact)
- `right_align`: Right-align output with padding (performance-neutral)
- `exclude_no_word`: Exclude lines without words (performance-neutral)
- `word_only`: Output only the word used for sorting (performance-neutral)
- `normalize`: Normalize Unicode to NFC form (performance impact)

## Performance

The library is designed for high performance with large datasets:

- Parallel processing using Rayon's work-stealing scheduler
- Zero-copy operations where possible
- Efficient character-by-character comparison
- Minimal memory allocation
- Optimized defaults for maximum throughput

## Examples

See the `cli` directory for a complete command-line implementation using this library.

## License

MIT OR Apache-2.0
