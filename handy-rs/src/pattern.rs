use jaro_winkler::jaro_winkler;
use levenshtein::levenshtein;
use regex::Regex;
use std::path::Path;

/// The margin of error for string similarity scores.
pub const ERROR_MARGIN: f64 = 0.001;

/// Converts a glob pattern to a regex pattern.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::pattern::glob_to_regex_pattern;
///
/// assert_eq!(glob_to_regex_pattern("fish*.txt"), "fish.*\\.txt");
/// ```
#[must_use]
pub fn glob_to_regex_pattern(pattern: &str) -> String {
    let mut regex_pattern = String::new();
    let mut escaping = false;

    for c in pattern.chars() {
        match c {
            '*' if !escaping => regex_pattern.push_str(".*"), // Match any sequence of characters
            '?' if !escaping => regex_pattern.push('.'),      // Match any single character
            '.' | '+' | '(' | ')' | '|' | '^' | '$' | '[' | ']' | '{' | '}' | '\\' if !escaping => {
                regex_pattern.push('\\'); // Escape regex special characters
                regex_pattern.push(c);
            }
            '\\' if !escaping => escaping = true, // Start escaping next character
            _ => {
                regex_pattern.push(c); // Literal character
                escaping = false;
            }
        }
    }
    regex_pattern
}

/// Checks if a string similarity score is close to the upper bound (1.0), which (according to the [`ERROR_MARGIN`]) indicates a perfect match.
///
/// ## Arguments
///
/// * `score` - The similarity score to check, can be from [`match_string`].
///
/// ## Returns
///
/// * `bool` - True if the score is close to 1.0, false otherwise.
#[must_use]
pub fn is_close_to_upper_bound(score: f64) -> bool {
    (score - 1.0).abs() < ERROR_MARGIN
}

/// Checks if a path's filename matches a glob pattern.
///
/// ## Examples
///
/// ```rust,no_run
/// use std::path::Path;
/// use handy::pattern::match_filename_with_glob_pattern;
///
/// assert!(match_filename_with_glob_pattern(Path::new("fish.txt"), "f*.txt"));
/// ```
///
/// ## Panics
///
/// This function panics if the internal glob pattern `.*` is invalid.
#[must_use]
pub fn match_filename_with_glob_pattern(path: &Path, pattern: &str) -> bool {
    let regex_pattern = glob_to_regex_pattern(pattern);
    let re = Regex::new(&regex_pattern).unwrap_or(Regex::new(".*").unwrap());

    if let Some(name) = path.file_name().map(|s| s.to_string_lossy().to_string()) {
        if re.is_match(&name) {
            return true;
        }
    }

    false
}

/// Returns a similarity score between two strings using a fuzzy matching algorithm.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::pattern::match_string;
///
/// let s1 = "Salvage Yard";
/// let s2 = "yard";
///
/// let score = match_string(s1, s2);
/// println!("Score: {}", score);
/// ```
///
/// ## Arguments
///
/// * `s1` - The first string.
/// * `s2` - The second string.
///
/// ## Returns
///
/// The similarity score between the two strings.
#[allow(clippy::cast_precision_loss)]
#[must_use]
pub fn match_string(s1: &str, s2: &str) -> f64 {
    let s1 = s1.to_lowercase();
    let s2 = s2.to_lowercase();

    if s1.is_empty() || s2.is_empty() {
        return if s1.is_empty() == s2.is_empty() {
            1.0
        } else {
            0.0
        };
    }

    if s1.contains(&s2) || s2.contains(&s1) {
        return 1.0;
    }

    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let shorter_len = len1.min(len2);

    if shorter_len == 0 {
        return 0.0;
    }

    let distance = levenshtein(&s1, &s2) as f64;
    let score = 1.0 - (distance / shorter_len as f64);

    score.clamp(0.0, 1.0)
}

/// Returns a similarity score between two strings using a fuzzy matching algorithm that relies on Jaro-Winkler instead Levenshtein. Use this over [`match_string`].
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::pattern::string_similarity;
///
/// let s1 = "Salvage Yard";
/// let s2 = "yad";
///
/// let score = string_similarity(s1, s2);
/// println!("Score: {}", score);
/// ```
///
/// ## Arguments
///
/// * `s1` - The first string.
/// * `s2` - The second string.
///
/// ## Returns
///
/// The similarity score between the two strings, the score is a [f64] between 0.0 and 1.0.
#[must_use]
pub fn string_similarity<S1, S2>(s1: S1, s2: S2) -> f64
where
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    string_similarity_impl(s1.as_ref(), s2.as_ref())
}

/// Returns a similarity score between two strings using a fuzzy matching algorithm that relies on Jaro-Winkler instead of Levenshtein.
fn string_similarity_impl(s1: &str, s2: &str) -> f64 {
    let s1 = s1.trim().to_lowercase();
    let s2 = s2.trim().to_lowercase();

    if s1.is_empty() || s2.is_empty() {
        return 0.0;
    }

    if s1.contains(&s2) || s2.contains(&s1) {
        return 1.0;
    }

    jaro_winkler(&s1, &s2)
}

/// Asserts that two strings have a similarity score close to the expected value.
#[macro_export]
macro_rules! assert_match_string {
    ($s1:expr, $s2:expr, $expected:expr) => {
        let actual = $crate::pattern::match_string($s1, $s2);
        assert!(
            (actual - $expected).abs() < $crate::pattern::ERROR_MARGIN,
            "Left: {}\nRight: {}",
            actual,
            $expected
        );
    };
}

/// Asserts that two strings have a similarity score close to the expected value.
#[macro_export]
macro_rules! assert_string_similarity {
    ($s1:expr, $s2:expr, $expected:expr) => {
        let actual = $crate::pattern::string_similarity($s1, $s2);
        assert!(
            (actual - $expected).abs() < $crate::pattern::ERROR_MARGIN,
            "Left: {}\nRight: {}",
            actual,
            $expected
        );
    };
}

#[cfg(test)]
mod tests {
    use super::{glob_to_regex_pattern, match_filename_with_glob_pattern};
    use crate::{assert_match_string, pattern::is_close_to_upper_bound};
    use std::path::Path;

    #[test]
    fn test_glob_to_regex() {
        assert_eq!(glob_to_regex_pattern("fish*.txt"), "fish.*\\.txt");
        assert_eq!(glob_to_regex_pattern("fish?txt"), "fish.txt");
        assert_eq!(glob_to_regex_pattern("fish+txt"), "fish\\+txt");
        assert_eq!(glob_to_regex_pattern("fish\\txt"), "fish\\\\txt");
        assert_eq!(glob_to_regex_pattern("fish\\(txt"), "fish\\\\\\(txt");
    }

    #[test]
    fn test_is_close_to_upper_bound() {
        assert!(is_close_to_upper_bound(1.0));
        assert!(is_close_to_upper_bound(0.9999));
    }

    #[test]
    #[should_panic(expected = "is_close_to_upper_bound(0.999)")]
    fn test_is_close_to_upper_bound_false() {
        assert!(is_close_to_upper_bound(0.999));
    }

    #[test]
    fn test_match_filename_with_glob_pattern() {
        assert!(match_filename_with_glob_pattern(
            Path::new("fish.txt"),
            "f*.txt"
        ));
        assert!(!match_filename_with_glob_pattern(
            Path::new("fish.txt"),
            "f*.jpg"
        ));
    }

    #[test]
    fn test_match_string() {
        assert_match_string!("kitten", "kissing", 0.333);
        assert_match_string!("Salvage Yard", "yard", 1.0);
        assert_match_string!("raiju", "yard", 0.0);
    }

    #[test]
    fn test_string_similarity() {
        assert_string_similarity!("kitten", "kissing", 0.714);
        assert_string_similarity!("Salvage Yard", "yard", 1.0);
        assert_string_similarity!("Salvage Yard", "yad", 0.472);
        assert_string_similarity!("raiju", "yard", 0.483);
    }
}
