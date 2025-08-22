use clap::Parser;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use suffixsort::{PaddingInfo, ProcessedLine, SortConfig};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "ssort: inverse lexicographic (suffix) sort by first word (default) or whole line",
    long_about = r#"
ssort: inverse lexicographic (suffix) sort by first word (default) or whole line

The inverse lexicographic sort, a.k.a. suffix sort, is a sort order
where strings are compared from the last character towards the first.
"#
)]
struct Args {
    /// input files (use '-' for stdin, default if no files provided)
    #[arg(value_name = "FILE")]
    files: Vec<String>,

    /// ignore case when sorting
    #[arg(short = 'i', long = "ignore-case", help_heading = "Sorting Options")]
    ignore_case: bool,

    /// use entire line for sorting instead of first word
    #[arg(short = 'l', long = "line", help_heading = "Sorting Options")]
    use_entire_line: bool,

    /// dictionary order: ignore non-alphabetic characters when finding first word
    #[arg(
        short = 'd',
        long = "dictionary-order",
        help_heading = "Sorting Options"
    )]
    dictionary_order: bool,

    /// reverse the sort order
    #[arg(short = 'r', long, help_heading = "Sorting Options")]
    reverse: bool,

    /// stable sort (maintains original order of equal elements)
    #[arg(short = 's', long, help_heading = "Sorting Options")]
    stable: bool,

    /// right-align output by adding leading spaces
    #[arg(short = 'a', long = "right-align", help_heading = "Output")]
    right_align: bool,

    /// exclude lines without words
    #[arg(short = 'x', long = "exclude-no-word", help_heading = "Output")]
    exclude_no_word: bool,

    /// output only the word used for sorting (excludes the remainder of lines)
    #[arg(short = 'w', long = "word-only", help_heading = "Output")]
    word_only: bool,

    /// normalize unicode to NFC form
    #[arg(short = 'n', long = "normalize", help_heading = "Sorting Options")]
    normalize: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Read input from files or stdin
    let lines = read_input(&args.files)?;

    // Create config for the library
    let config = SortConfig {
        ignore_case: args.ignore_case,
        use_entire_line: args.use_entire_line,
        dictionary_order: args.dictionary_order,
        reverse: args.reverse,
        stable: args.stable,
        right_align: args.right_align,
        exclude_no_word: args.exclude_no_word,
        word_only: args.word_only,
        normalize: args.normalize,
    };

    // Process and sort lines using the library
    let (processed, padding_info) = config.process_lines(lines);

    // Write results
    write_output(processed, padding_info, args.word_only, args.right_align)
}

fn read_input(files: &[String]) -> io::Result<Vec<String>> {
    if files.is_empty() {
        // Read from stdin
        io::stdin().lock().lines().collect()
    } else {
        // Read from files
        let mut lines = Vec::new();
        for filename in files {
            if filename == "-" {
                // Read from stdin
                lines.extend(io::stdin().lock().lines().collect::<Result<Vec<_>, _>>()?);
            } else {
                // Read from file
                let file = File::open(filename).map_err(|e| {
                    io::Error::new(io::ErrorKind::NotFound, format!("'{}': {}", filename, e))
                })?;
                let reader = BufReader::new(file);
                lines.extend(reader.lines().collect::<Result<Vec<_>, _>>()?);
            }
        }
        Ok(lines)
    }
}

fn write_output(
    processed: Vec<ProcessedLine>,
    padding_info: Option<PaddingInfo>,
    word_only: bool,
    right_align: bool,
) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if word_only {
        // Output only the word used for sorting
        if right_align {
            let max_key_len = processed
                .iter()
                .map(|p| p.key.chars().count())
                .max()
                .unwrap_or(0);

            for p in processed {
                let padding = " ".repeat(max_key_len.saturating_sub(p.key.chars().count()));
                writeln!(handle, "{}{}", padding, p.key)?;
            }
        } else {
            for p in processed {
                writeln!(handle, "{}", p.key)?;
            }
        }
    } else if let Some(padding_info) = padding_info {
        for p in processed {
            if padding_info.use_end_pos {
                // Dictionary order with right-align - use end position of first word
                if let (Some(visual_start), Some(word_length)) = (p.visual_start, p.word_length) {
                    let end_pos = visual_start + word_length;
                    let padding = " ".repeat(padding_info.max_value.saturating_sub(end_pos));
                    writeln!(handle, "{}{}", padding, p.original)?;
                } else {
                    // Line has no word, output without padding
                    writeln!(handle, "{}", p.original)?;
                }
            } else {
                // Other modes
                let padding =
                    " ".repeat(padding_info.max_value.saturating_sub(p.key.chars().count()));
                writeln!(handle, "{}{}", padding, p.original)?;
            }
        }
    } else {
        for p in processed {
            writeln!(handle, "{}", p.original)?;
        }
    }

    Ok(())
}
