#![allow(clippy::cast_precision_loss)]

use num_traits::{AsPrimitive, Zero};

/// Formats bytes into a human readable string.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::human::human_bytes;
///
/// assert_eq!(human_bytes(123_456_789), "118 MiB");
/// ```
#[must_use]
pub fn human_bytes<U>(bytes: U) -> String
where
    U: Zero + AsPrimitive<f64> + PartialEq,
{
    const UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

    if bytes == U::zero() {
        return "0 B".to_string();
    }

    let mut bytes = bytes.as_();
    let mut index = 0;

    while bytes >= 1024.0 && index < UNITS.len() - 1 {
        bytes /= 1024.0;
        index += 1;
    }

    let n = if bytes < 10.0 {
        2
    } else {
        usize::from(bytes < 100.0)
    };
    format!("{bytes:.n$} {}", UNITS[index])
}

/// Formats bytes into a human readable string and its unit.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::human::human_bytes_as_parts;
///
/// assert_eq!(human_bytes_as_parts(123_456_789), (118.0, "MiB".to_string()));
/// ```
#[must_use]
pub fn human_bytes_as_parts<U>(bytes: U) -> (f64, String)
where
    U: Zero + AsPrimitive<f64> + PartialEq,
{
    const UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

    if bytes == U::zero() {
        return (0.0, "B".to_string());
    }

    let mut bytes = bytes.as_();
    let mut index = 0;

    while bytes >= 1024.0 && index < UNITS.len() - 1 {
        bytes /= 1024.0;
        index += 1;
    }

    (bytes, UNITS[index].to_string())
}

/// Formats bytes into a human readable string using SI units.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::human::human_bytes_si;
///
/// assert_eq!(human_bytes_si(123_456_789), "118 MB");
/// ```
#[must_use]
pub fn human_bytes_si<U>(bytes: U) -> String
where
    U: Zero + AsPrimitive<f64> + PartialEq,
{
    const UNITS: [&str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];

    if bytes == U::zero() {
        return "0 B".to_string();
    }

    let mut bytes = bytes.as_();
    let mut index = 0;

    while bytes >= 1000.0 && index < UNITS.len() - 1 {
        bytes /= 1000.0;
        index += 1;
    }

    let n = if bytes < 10.0 {
        2
    } else {
        usize::from(bytes < 100.0)
    };
    format!("{bytes:.n$} {}", UNITS[index])
}

/// Formats bytes into a human readable string and its unit using SI units.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::human::human_bytes_si_as_parts;
///
/// assert_eq!(human_bytes_si_as_parts(123_456_789), (118.0, "MB".to_string()));
/// ```
#[must_use]
pub fn human_bytes_si_as_parts<U>(bytes: U) -> (f64, String)
where
    U: Zero + AsPrimitive<f64> + PartialEq,
{
    const UNITS: [&str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];

    if bytes == U::zero() {
        return (0.0, "B".to_string());
    }

    let mut bytes = bytes.as_();
    let mut index = 0;

    while bytes >= 1000.0 && index < UNITS.len() - 1 {
        bytes /= 1000.0;
        index += 1;
    }

    (bytes, UNITS[index].to_string())
}

/// Formats a number into a human readable string.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::human::human_number;
///
/// assert_eq!(human_number(123_456_789), "123 M");
/// ```
#[must_use]
pub fn human_number<U>(number: U) -> String
where
    U: Zero + AsPrimitive<f64> + PartialEq,
{
    const UNITS: [&str; 7] = ["", "K", "M", "B", "T", "Qa", "Qi"];

    if number == U::zero() {
        return "0".to_string();
    }

    let mut number = number.as_();
    let mut index = 0;

    while number >= 1000.0 && index < UNITS.len() - 1 {
        number /= 1000.0;
        index += 1;
    }

    let n = if number < 10.0 {
        2
    } else {
        usize::from(number < 100.0)
    };
    format!("{number:.n$} {}", UNITS[index]).trim().to_string()
}

/// Formats a number into a human readable string and its unit.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::human::human_number_as_parts;
///
/// assert_eq!(human_number_as_parts(123_456_789), (123.0, "M".to_string()));
/// ```
pub fn human_number_as_parts<U>(number: U) -> (f64, String)
where
    U: Zero + AsPrimitive<f64> + PartialEq,
{
    const UNITS: [&str; 7] = ["", "K", "M", "B", "T", "Qa", "Qi"];

    if number == U::zero() {
        return (0.0, String::new());
    }

    let mut number = number.as_();
    let mut index = 0;

    while number >= 1000.0 && index < UNITS.len() - 1 {
        number /= 1000.0;
        index += 1;
    }

    (number, UNITS[index].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_bytes() {
        assert_eq!(human_bytes(0), "0 B");
        assert_eq!(human_bytes(635), "635 B");
        assert_eq!(human_bytes(12_345), "12.1 KiB");
        assert_eq!(human_bytes(1_234_567), "1.18 MiB");
        assert_eq!(human_bytes(123_456_789), "118 MiB");
        assert_eq!(human_bytes(12_345_678_901u64), "11.5 GiB");
        assert_eq!(human_bytes(123_456_789_012u64), "115 GiB");
        assert_eq!(human_bytes(123_456_789_012_345u64), "112 TiB");
        assert_eq!(human_bytes(123_456_789_012_345_678u64), "110 PiB");
    }

    #[test]
    fn test_human_bytes_as_parts() {
        assert_eq!(human_bytes_as_parts(0), (0.0, "B".to_string()));
        assert_eq!(human_bytes_as_parts(635), (635.0, "B".to_string()));
        assert_eq!(
            human_bytes_as_parts(12_345),
            (12.055_664_062_5, "KiB".to_string())
        );
        assert_eq!(
            human_bytes_as_parts(1_234_567),
            (1.177_374_839_782_714_8, "MiB".to_string())
        );
        assert_eq!(
            human_bytes_as_parts(123_456_789),
            (117.737_568_855_285_64, "MiB".to_string())
        );
        assert_eq!(
            human_bytes_as_parts(12_345_678_901u64),
            (11.497_809_459_455_311, "GiB".to_string())
        );
        assert_eq!(
            human_bytes_as_parts(123_456_789_012u64),
            (114.978_094_596_415_76, "GiB".to_string())
        );
        assert_eq!(
            human_bytes_as_parts(123_456_789_012_345u64),
            (112.283_295_504_626_04, "TiB".to_string())
        );
        assert_eq!(
            human_bytes_as_parts(123_456_789_012_345_678u64),
            (109.651_655_766_236_97, "PiB".to_string())
        );
    }

    #[test]
    fn test_human_bytes_si() {
        assert_eq!(human_bytes_si(0), "0 B");
        assert_eq!(human_bytes_si(635), "635 B");
        assert_eq!(human_bytes_si(12_345), "12.3 KB");
        assert_eq!(human_bytes_si(1_234_567), "1.23 MB");
        assert_eq!(human_bytes_si(123_456_789), "123 MB");
        assert_eq!(human_bytes_si(12_345_678_901u64), "12.3 GB");
        assert_eq!(human_bytes_si(123_456_789_012u64), "123 GB");
        assert_eq!(human_bytes_si(123_456_789_012_345u64), "123 TB");
        assert_eq!(human_bytes_si(123_456_789_012_345_678u64), "123 PB");
    }

    #[test]
    fn test_human_bytes_si_as_parts() {
        assert_eq!(human_bytes_si_as_parts(0), (0.0, "B".to_string()));
        assert_eq!(human_bytes_si_as_parts(635), (635.0, "B".to_string()));
        assert_eq!(human_bytes_si_as_parts(12_345), (12.345, "KB".to_string()));
        assert_eq!(
            human_bytes_si_as_parts(1_234_567),
            (1.234_567, "MB".to_string())
        );
        assert_eq!(
            human_bytes_si_as_parts(123_456_789),
            (123.456_789, "MB".to_string())
        );
        assert_eq!(
            human_bytes_si_as_parts(12_345_678_901u64),
            (12.345_678_901_000_001, "GB".to_string())
        );
        assert_eq!(
            human_bytes_si_as_parts(123_456_789_012u64),
            (123.456_789_011_999_99, "GB".to_string())
        );
        assert_eq!(
            human_bytes_si_as_parts(123_456_789_012_345u64),
            (123.456_789_012_345, "TB".to_string())
        );
        assert_eq!(
            human_bytes_si_as_parts(123_456_789_012_345_678u64),
            (123.456_789_012_345_7, "PB".to_string())
        );
    }

    #[test]
    fn test_human_number() {
        assert_eq!(human_number(0), "0");
        assert_eq!(human_number(635), "635");
        assert_eq!(human_number(12_345), "12.3 K");
        assert_eq!(human_number(1_234_567), "1.23 M");
        assert_eq!(human_number(123_456_789), "123 M");
        assert_eq!(human_number(12_345_678_901u64), "12.3 B");
        assert_eq!(human_number(123_456_789_012u64), "123 B");
        assert_eq!(human_number(123_456_789_012_345u64), "123 T");
        assert_eq!(human_number(123_456_789_012_345_678u64), "123 Qa");
    }

    #[test]
    fn test_human_number_as_parts() {
        assert_eq!(human_number_as_parts(0), (0.0, String::new()));
        assert_eq!(human_number_as_parts(635), (635.0, String::new()));
        assert_eq!(human_number_as_parts(12_345), (12.345, "K".to_string()));
        assert_eq!(
            human_number_as_parts(1_234_567),
            (1.234_567, "M".to_string())
        );
        assert_eq!(
            human_number_as_parts(123_456_789),
            (123.456_789, "M".to_string())
        );
        assert_eq!(
            human_number_as_parts(12_345_678_901u64),
            (12.345_678_901_000_001, "B".to_string())
        );
        assert_eq!(
            human_number_as_parts(123_456_789_012u64),
            (123.456_789_011_999_99, "B".to_string())
        );
        assert_eq!(
            human_number_as_parts(123_456_789_012_345u64),
            (123.456_789_012_345, "T".to_string())
        );
        assert_eq!(
            human_number_as_parts(123_456_789_012_345_678u64),
            (123.456_789_012_345_7, "Qa".to_string())
        );
    }
}
