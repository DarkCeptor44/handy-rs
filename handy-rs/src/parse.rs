use crate::errors::ParseError;
use std::str::FromStr;

/// Splits a string into a number and a suffix, e.g. `123abc` -> (123, "abc").
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::parse::split_at_non_digits;
///
/// assert_eq!(split_at_non_digits("123abc").unwrap(), (123, "abc".to_string()));
/// ```
///
/// ## Errors
///
/// - [`ParseError::InvalidNumber`](crate::errors::ParseError::InvalidNumber): If the prefix cannot be parsed as a number
pub fn split_at_non_digits<N>(s: &str) -> Result<(N, String), ParseError>
where
    N: FromStr,
{
    let split_index = s
        .find(|c: char| !c.is_ascii_digit() && c != '.')
        .unwrap_or(s.len());

    let (prefix_str, suffix_str) = s.split_at(split_index);

    let prefix: N = prefix_str
        .parse()
        .map_err(|_| ParseError::InvalidNumber(prefix_str.to_string()))?;

    Ok((prefix, suffix_str.to_string()))
}

#[cfg(test)]
mod tests {
    use super::split_at_non_digits;

    #[allow(clippy::approx_constant)]
    #[test]
    fn test_split_at_non_digits() {
        assert_eq!(
            split_at_non_digits("123abc").unwrap(),
            (123, "abc".to_string())
        );
        assert_eq!(
            split_at_non_digits("123ab4c").unwrap(),
            (123, "ab4c".to_string())
        );
        assert_eq!(
            split_at_non_digits("9.8 m/s").unwrap(),
            (9.8, " m/s".to_string())
        );
        assert_eq!(
            split_at_non_digits("3.14159").unwrap(),
            (3.14159, String::new())
        );
    }
}
