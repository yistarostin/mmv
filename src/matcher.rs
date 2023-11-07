pub struct Matcher {
    pub pattern: String,
}

impl Matcher {
    /// Creates `Matcher` instance
    ///
    /// # Arguments
    ///
    /// * pattern: `String` -- string containing stars, which will be used to filter filenames fitting the pattern
    ///
    pub fn new(pattern: String) -> Self {
        Matcher { pattern }
    }

    pub fn fill_in_template<'a>(
        &'a self,
        substituted_parts: Vec<&str>,
        filename: &'a str,
    ) -> String {
        let mut result_string = filename.to_string();
        substituted_parts
            .iter()
            .rev()
            .enumerate()
            .for_each(|(index, substring)| {
                result_string = result_string.replace(
                    &format!("#{w}", w = substituted_parts.len() - index),
                    substring,
                );
            });
        result_string
    }

    /// Check if `file` matches `self.pattern`, meaning if it is possible to replace all stars in `self.pattern` with arbitrary to get `filename` string
    ///
    /// # Arguments
    ///
    /// * filename: `&str` -- filename to check
    ///
    /// # Returns
    ///
    /// `None` in case filename does not fit `self.pattern`, Vec<String> of substituted substrings otherwise

    pub fn pattern_matcher<'a>(&'a self, filename: &'a str) -> Option<Vec<String>> {
        if self.pattern.len() > filename.len() {
            return None;
        }
        let mut substituted_parts: Vec<String> = vec![];
        let indexes_of_stars: Vec<usize> = self
            .pattern
            .char_indices()
            .filter(|(_, ch)| ch == &'*')
            .map(|(ind, _)| ind)
            .collect();
        let first_star = *indexes_of_stars.first().unwrap_or(&filename.len());
        if first_star == filename.len() {
            return if filename == self.pattern {
                Some(vec![])
            } else {
                None
            };
        }
        let substring_to_match = &self.pattern[0..first_star];
        if !filename.is_char_boundary(first_star) {
            return None;
        }
        let file_substring = &filename[0..first_star];
        if substring_to_match != file_substring {
            return None;
        }
        let stars_iterator = self
            .pattern
            .clone()
            .char_indices()
            .filter(|(_, ch)| ch == &'*')
            .collect::<Vec<(usize, char)>>()
            .into_iter()
            .peekable();
        let mut pattern_iterator = stars_iterator;
        let mut filename_iterator = filename
            .clone()
            .char_indices()
            .skip_while(|(ind, _)| ind < &first_star)
            .collect::<Vec<(usize, char)>>()
            .into_iter()
            .peekable();
        while pattern_iterator.peek().is_some() || filename_iterator.peek().is_some() {
            if indexes_of_stars.contains(&(pattern_iterator.peek().unwrap().0 + 1))
                || (pattern_iterator.peek().unwrap().0 == self.pattern.len() - 1)
            {
                let remaining_pattern: String = pattern_iterator
                    .clone()
                    .map(|(_, character)| character)
                    .collect();
                if remaining_pattern == "*".repeat(remaining_pattern.len()) {
                    substituted_parts.push(
                        filename_iterator
                            .clone()
                            .map(|(_, character)| character)
                            .collect::<String>()
                            .to_owned(),
                    );
                    break;
                }
            }
            let substring_to_match = &self.pattern[pattern_iterator.peek().unwrap().0 + 1
                ..pattern_iterator
                    .clone()
                    .nth(1)
                    .unwrap_or((self.pattern.len(), '*'))
                    .0];
            let mut start_of_equal_substring = filename_iterator.clone();
            while start_of_equal_substring.peek().is_some() {
                let file_substring: String = start_of_equal_substring
                    .clone()
                    .take(substring_to_match.len())
                    .map(|(_, character)| character)
                    .collect();
                if substring_to_match == file_substring {
                    break;
                }
                start_of_equal_substring.next();
            }
            if start_of_equal_substring.peek().is_none() {
                return None;
            }
            let file_substring: String = start_of_equal_substring
                .clone()
                .take(substring_to_match.len())
                .map(|(_, character)| character)
                .collect();
            if substring_to_match != file_substring {
                break;
            }
            let substituted_substring: String = filename_iterator
                .clone()
                .filter(|(ind, _)| ind < &start_of_equal_substring.peek().unwrap().0)
                .map(|(_, character)| character)
                .collect();

            substituted_parts.push(substituted_substring);

            filename_iterator = start_of_equal_substring.clone();
            for _ in 0..substring_to_match.len() {
                filename_iterator.next();
            }
            pattern_iterator.next();
        }
        Some(substituted_parts)
    }
}

#[test]
fn test_substitution() {
    let matcher = Matcher::new("path/to/some_*_filename.*".to_string());
    let template = "path2/to/changed_#1_filename.#2";
    let filenames = vec![
        "path/to/some_A_filename.bin",
        "path/to/some_B_filename.bin",
        "path/to/some_A_filename.jpg",
        "path/to/some_B_filename.jpg",
    ];
    for file in filenames {
        let changes = matcher.pattern_matcher(file);
        let file_letter = &file[13..14];
        let file_extension = &file[file.len() - 3..];
        assert!(changes.is_some());
        assert!(dbg!(changes.clone()).unwrap() == vec![file_letter, file_extension]);
        let new_name = matcher.fill_in_template(
            changes.unwrap().iter().map(|s| s.as_str()).collect(),
            template,
        );
        assert_eq!(
            dbg!(new_name),
            template
                .replace("#1", file_letter)
                .replace("#2", file_extension)
        );
    }
}
#[test]
fn test_star_as_dirname() {
    let matcher = Matcher::new("path/*".to_string());
    let filename = "path/to/some/file";
    let changes = matcher.pattern_matcher(filename);
    assert!(changes.is_some());
}

#[test]
fn test_no_star() {
    let matcher = Matcher::new("abc".to_string());
    let filename = "abc";
    let changes = matcher.pattern_matcher(filename);
    assert_eq!(changes, Some(vec![]))
}
