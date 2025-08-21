use clap::Parser;
use rayon::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

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
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Read input from files or stdin
    let lines = if args.files.is_empty() {
        // Read from stdin
        io::stdin().lock().lines().collect::<Result<Vec<_>, _>>()?
    } else {
        // Read from files
        let mut lines = Vec::new();
        for filename in &args.files {
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
        lines
    };

    // We need to store extra information for right-alignment with dictionary order
    let mut processed: Vec<_> =
        if args.right_align && args.dictionary_order && !args.use_entire_line {
            lines
                .into_par_iter()
                .enumerate()
                .filter_map(|(index, line)| {
                    // For dictionary order with right-align, we need both the visual start and word length
                    let (key, visual_start, word_length) = {
                        // Find the start of the first word
                        let word_start = line
                            .char_indices()
                            .find(|(_, c)| c.is_alphabetic())
                            .map(|(idx, _)| idx);

                        match word_start {
                            Some(start) => {
                                // Find end of word
                                let word_end = line[start..]
                                    .char_indices()
                                    .find(|(_, c)| !(c.is_alphabetic() || *c == '-'))
                                    .map(|(idx, _)| start + idx)
                                    .unwrap_or_else(|| line.len());

                                let word = line[start..word_end].to_string();
                                let word_len = word.chars().count();
                                (word, Some(start), Some(word_len))
                            }
                            None => {
                                // No alphabetic characters found
                                (String::new(), None, None)
                            }
                        }
                    };

                    // Exclude lines without words if requested
                    if args.exclude_no_word && key.is_empty() {
                        None
                    } else {
                        Some((line, key, index, visual_start, word_length))
                    }
                })
                .collect()
        } else {
            lines
                .into_par_iter()
                .enumerate()
                .filter_map(|(index, line)| {
                    let key = if args.use_entire_line {
                        // Use entire line as key
                        line.clone()
                    } else if args.dictionary_order {
                        // Dictionary order word extraction
                        let word_start = line
                            .char_indices()
                            .find(|(_, c)| c.is_alphabetic())
                            .map(|(idx, _)| idx)
                            .unwrap_or(usize::MAX);

                        if word_start == usize::MAX {
                            String::new()
                        } else {
                            let word_end = line[word_start..]
                                .char_indices()
                                .find(|(_, c)| !(c.is_alphabetic() || *c == '-'))
                                .map(|(idx, _)| word_start + idx)
                                .unwrap_or_else(|| line.len());

                            line[word_start..word_end].to_string()
                        }
                    } else {
                        // Standard word extraction
                        let mut start = 0;
                        let mut end = 0;
                        let mut in_word = false;

                        for (idx, c) in line.char_indices() {
                            if c.is_whitespace() {
                                if in_word {
                                    // Found end of first word
                                    end = idx;
                                    break;
                                }
                            } else if !in_word {
                                // Found start of first word
                                start = idx;
                                in_word = true;
                            }
                        }

                        // Handle words at end of line
                        if in_word && end == 0 {
                            line[start..].to_string()
                        } else if in_word {
                            line[start..end].to_string()
                        } else {
                            String::new()
                        }
                    };

                    // Exclude lines without words if requested
                    if args.exclude_no_word && key.is_empty() {
                        None
                    } else {
                        Some((line, key, index, None, None))
                    }
                })
                .collect()
        };

    // Compute padding information
    let padding_info = if args.right_align {
        if args.dictionary_order && !args.use_entire_line && !args.word_only {
            // For dictionary order with right-align, we need the end position of the first word
            // Only consider lines that actually have a word
            let max_end_pos = processed
                .par_iter()
                .filter_map(|(_, _, _, visual_start, word_length)| {
                    visual_start.and_then(|s| word_length.map(|l| s + l))
                })
                .max()
                .unwrap_or(0);

            Some((max_end_pos, true))
        } else {
            // For other modes, just use key length
            let max_key_len = processed
                .par_iter()
                .map(|(_, key, _, _, _)| key.chars().count())
                .max()
                .unwrap_or(0);

            Some((max_key_len, false))
        }
    } else {
        None
    };

    // Create comparison closure
    let ignore_case = args.ignore_case;
    let comparator =
        |a: &(String, String, usize, Option<usize>, Option<usize>),
         b: &(String, String, usize, Option<usize>, Option<usize>)| {
            // Create case folding function based on flag
            let fold = |c: char| -> Box<dyn Iterator<Item = char>> {
                if ignore_case {
                    Box::new(c.to_lowercase()) as Box<dyn Iterator<Item = char>>
                } else {
                    Box::new(std::iter::once(c)) as Box<dyn Iterator<Item = char>>
                }
            };

            // Compare characters in reverse order
            let mut a_iter = a.1.chars().rev().flat_map(fold);
            let mut b_iter = b.1.chars().rev().flat_map(fold);

            let mut word_cmp = std::cmp::Ordering::Equal;
            loop {
                match (a_iter.next(), b_iter.next()) {
                    (Some(a_char), Some(b_char)) => {
                        let cmp = a_char.cmp(&b_char);
                        if cmp != std::cmp::Ordering::Equal {
                            word_cmp = cmp;
                            break;
                        }
                    }
                    (Some(_), None) => {
                        word_cmp = std::cmp::Ordering::Greater;
                        break;
                    }
                    (None, Some(_)) => {
                        word_cmp = std::cmp::Ordering::Less;
                        break;
                    }
                    (None, None) => break,
                }
            }

            // Apply reverse flag if needed
            if args.reverse {
                word_cmp = word_cmp.reverse();
            }

            // Secondary comparison for tie-breaking
            if word_cmp == std::cmp::Ordering::Equal {
                // For stable sort, preserve original order
                a.2.cmp(&b.2)
            } else {
                word_cmp
            }
        };

    // Choose between stable and unstable sort
    if args.stable {
        processed.par_sort_by(comparator);
    } else {
        processed.par_sort_unstable_by(comparator);
    }

    // Write results
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if args.word_only {
        // Output only the word used for sorting
        if args.right_align {
            let max_key_len = processed
                .par_iter()
                .map(|(_, key, _, _, _)| key.chars().count())
                .max()
                .unwrap_or(0);

            for (_, key, _, _, _) in processed {
                let padding = " ".repeat(max_key_len.saturating_sub(key.chars().count()));
                writeln!(handle, "{}{}", padding, key)?;
            }
        } else {
            for (_, key, _, _, _) in processed {
                writeln!(handle, "{}", key)?;
            }
        }
    } else if let Some((max_value, use_end_pos)) = padding_info {
        for (line, key, _, visual_start, word_length) in processed {
            if use_end_pos {
                // Dictionary order with right-align - use end position of first word
                if let (Some(visual_start), Some(word_length)) = (visual_start, word_length) {
                    let end_pos = visual_start + word_length;
                    let padding = " ".repeat(max_value.saturating_sub(end_pos));
                    writeln!(handle, "{}{}", padding, line)?;
                } else {
                    // Line has no word, output without padding
                    writeln!(handle, "{}", line)?;
                }
            } else {
                // Other modes
                let padding = " ".repeat(max_value.saturating_sub(key.chars().count()));
                writeln!(handle, "{}{}", padding, line)?;
            }
        }
    } else {
        for (line, _, _, _, _) in processed {
            writeln!(handle, "{}", line)?;
        }
    }

    Ok(())
}
