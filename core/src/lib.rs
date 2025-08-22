use rayon::prelude::*;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct SortConfig {
    pub ignore_case: bool,
    pub use_entire_line: bool,
    pub dictionary_order: bool,
    pub reverse: bool,
    pub stable: bool,
    pub right_align: bool,
    pub exclude_no_word: bool,
    pub word_only: bool,
}

#[derive(Debug)]
pub struct ProcessedLine {
    pub original: String,
    pub key: String,
    pub index: usize,
    pub visual_start: Option<usize>,
    pub word_length: Option<usize>,
}

#[derive(Debug)]
pub struct PaddingInfo {
    pub max_value: usize,
    pub use_end_pos: bool,
}

impl SortConfig {
    pub fn process_lines(&self, lines: Vec<String>) -> (Vec<ProcessedLine>, Option<PaddingInfo>) {
        // Process lines (extract keys, filter, etc.)
        let mut processed = if self.right_align && self.dictionary_order && !self.use_entire_line {
            self.process_lines_dict_align(&lines)
        } else {
            self.process_lines_standard(&lines)
        };

        // Compute padding information if needed
        let padding_info = if self.right_align {
            Some(self.compute_padding_info(&processed))
        } else {
            None
        };

        // Sort the processed lines
        self.sort_processed_lines(&mut processed);

        (processed, padding_info)
    }

    /// Creates a comparator closure that can be used with Rust's sort_by method.
    /// This allows advanced users to build custom sorting pipelines while using
    /// the same comparison logic as the ssort tool.
    ///
    /// # Example
    /// ```
    /// use suffixsort::SortConfig;
    /// use std::cmp::Ordering;
    ///
    /// let config = SortConfig {
    ///     ignore_case: true,
    ///     reverse: false,
    ///     ..SortConfig::default()
    /// };
    ///
    /// let comparer = config.get_comparer();
    /// let result = comparer("apple", "Banana");
    /// // Note: The exact result depends on the inverse lexicographic comparison
    /// ```
    pub fn get_comparer(&self) -> impl Fn(&str, &str) -> Ordering + '_ {
        let ignore_case = self.ignore_case;
        let reverse = self.reverse;

        move |a: &str, b: &str| {
            // Create case folding function based on flag
            let fold = |c: char| -> Box<dyn Iterator<Item = char>> {
                if ignore_case {
                    Box::new(c.to_lowercase())
                } else {
                    Box::new(std::iter::once(c))
                }
            };

            // Compare characters in reverse order (inverse lexicographic)
            let mut a_iter = a.chars().rev().flat_map(&fold);
            let mut b_iter = b.chars().rev().flat_map(&fold);

            let mut ordering = Ordering::Equal;
            loop {
                match (a_iter.next(), b_iter.next()) {
                    (Some(a_char), Some(b_char)) => {
                        let cmp = a_char.cmp(&b_char);
                        if cmp != Ordering::Equal {
                            ordering = cmp;
                            break;
                        }
                    }
                    (Some(_), None) => {
                        ordering = Ordering::Greater;
                        break;
                    }
                    (None, Some(_)) => {
                        ordering = Ordering::Less;
                        break;
                    }
                    (None, None) => break,
                }
            }

            // Apply reverse flag if needed
            if reverse {
                ordering.reverse()
            } else {
                ordering
            }
        }
    }

    fn process_lines_dict_align(&self, lines: &[String]) -> Vec<ProcessedLine> {
        lines
            .par_iter()
            .enumerate()
            .filter_map(|(index, line)| {
                let (key, visual_start, word_length) = {
                    let word_start = line
                        .char_indices()
                        .find(|(_, c)| c.is_alphabetic())
                        .map(|(idx, _)| idx);

                    match word_start {
                        Some(start) => {
                            let word_end = line[start..]
                                .char_indices()
                                .find(|(_, c)| !(c.is_alphabetic() || *c == '-'))
                                .map(|(idx, _)| start + idx)
                                .unwrap_or_else(|| line.len());

                            let word = line[start..word_end].to_string();
                            let word_len = word.chars().count();
                            (word, Some(start), Some(word_len))
                        }
                        None => (String::new(), None, None),
                    }
                };

                if self.exclude_no_word && key.is_empty() {
                    None
                } else {
                    Some(ProcessedLine {
                        original: line.clone(),
                        key,
                        index,
                        visual_start,
                        word_length,
                    })
                }
            })
            .collect()
    }

    fn process_lines_standard(&self, lines: &[String]) -> Vec<ProcessedLine> {
        lines
            .par_iter()
            .enumerate()
            .filter_map(|(index, line)| {
                let key = if self.use_entire_line {
                    line.clone()
                } else if self.dictionary_order {
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
                    let mut start = 0;
                    let mut end = 0;
                    let mut in_word = false;

                    for (idx, c) in line.char_indices() {
                        if c.is_whitespace() {
                            if in_word {
                                end = idx;
                                break;
                            }
                        } else if !in_word {
                            start = idx;
                            in_word = true;
                        }
                    }

                    if in_word && end == 0 {
                        line[start..].to_string()
                    } else if in_word {
                        line[start..end].to_string()
                    } else {
                        String::new()
                    }
                };

                if self.exclude_no_word && key.is_empty() {
                    None
                } else {
                    Some(ProcessedLine {
                        original: line.clone(),
                        key,
                        index,
                        visual_start: None,
                        word_length: None,
                    })
                }
            })
            .collect()
    }

    fn compute_padding_info(&self, processed: &[ProcessedLine]) -> PaddingInfo {
        if self.dictionary_order && !self.use_entire_line && !self.word_only {
            let max_end_pos = processed
                .par_iter()
                .filter_map(|p| p.visual_start.and_then(|s| p.word_length.map(|l| s + l)))
                .max()
                .unwrap_or(0);

            PaddingInfo {
                max_value: max_end_pos,
                use_end_pos: true,
            }
        } else {
            let max_key_len = processed
                .par_iter()
                .map(|p| p.key.chars().count())
                .max()
                .unwrap_or(0);

            PaddingInfo {
                max_value: max_key_len,
                use_end_pos: false,
            }
        }
    }

    fn sort_processed_lines(&self, processed: &mut [ProcessedLine]) {
        // Get the string comparer
        let string_comparer = self.get_comparer();

        // Create a comparator for ProcessedLine items
        let comparator = |a: &ProcessedLine, b: &ProcessedLine| {
            // Use the string comparer to compare the keys
            let key_cmp = string_comparer(&a.key, &b.key);

            // For equal keys, maintain original order (stable sort)
            if key_cmp == Ordering::Equal {
                a.index.cmp(&b.index)
            } else {
                key_cmp
            }
        };

        if self.stable {
            processed.par_sort_by(comparator);
        } else {
            processed.par_sort_unstable_by(comparator);
        }
    }
}

impl Default for SortConfig {
    fn default() -> Self {
        Self {
            ignore_case: false,
            use_entire_line: false,
            dictionary_order: false,
            reverse: false,
            stable: false,
            right_align: false,
            exclude_no_word: false,
            word_only: false,
        }
    }
}
