use num_traits::{AsPrimitive, Zero};
use std::sync::OnceLock;

static NUM_HUMANIZER: OnceLock<Humanizer> = OnceLock::new();
static BINARY_HUMANIZER: OnceLock<Humanizer> = OnceLock::new();
static SI_HUMANIZER: OnceLock<Humanizer> = OnceLock::new();

fn num_humanizer() -> &'static Humanizer {
    NUM_HUMANIZER.get_or_init(|| {
        Humanizer::new(&["", "K", "M", "B", "T", "Qa", "Qd"])
            .with_division_factor(1000.0)
            .with_space_before_unit(true)
    })
}

fn binary_humanizer() -> &'static Humanizer {
    BINARY_HUMANIZER.get_or_init(|| {
        Humanizer::new(&["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"])
            .with_division_factor(1024.0)
            .with_space_before_unit(true)
    })
}

fn si_humanizer() -> &'static Humanizer {
    SI_HUMANIZER.get_or_init(|| {
        Humanizer::new(&["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"])
            .with_division_factor(1000.0)
            .with_space_before_unit(true)
    })
}

/// A struct that can be used to humanize numbers with custom units.
#[derive(Clone, Debug)]
pub struct Humanizer {
    units: Vec<String>,
    space_before_unit: bool,
    division_factor: f64,
}

impl Humanizer {
    /// Creates a new humanizer with the given units.
    ///
    /// Note: the first unit is the default one so it's usually empty unless it's bytes.
    ///
    /// ## Arguments
    ///
    /// * `units` - The units to use when humanizing numbers.
    ///
    /// ## Returns
    ///
    /// A new humanizer with the given units.
    ///
    /// ## Panics
    ///
    /// Panics if `units` is empty.
    pub fn new(units: &[&str]) -> Self {
        assert!(!units.is_empty(), "Units slice must not be empty");

        Self {
            units: units.iter().map(std::string::ToString::to_string).collect(),
            space_before_unit: true,
            division_factor: 1000.0,
        }
    }

    /// Sets whether or not to add a space before the unit (default: `true`).
    /// Example: `true` -> "1 MB", `false` -> "1MB".
    #[must_use]
    pub fn with_space_before_unit(mut self, space_before_unit: bool) -> Self {
        self.space_before_unit = space_before_unit;
        self
    }

    /// Sets the division factor between units (default: `1000.0`).
    /// Example: Use `1024.0` for binary prefixes (KiB, MiB, etc.).
    ///
    /// ## Panics
    ///
    /// Panics if the division factor is less than or equal to 0.
    #[must_use]
    pub fn with_division_factor<F>(mut self, factor: F) -> Self
    where
        F: Into<f64>,
    {
        self.division_factor = factor.into();
        assert!(
            self.division_factor >= 0.0,
            "Division factor must be greater than 0"
        );
        self
    }

    /// Calculates the number and index of the unit to use when humanizing a number.
    ///
    /// ## Returns
    ///
    /// * `f64` - The scaled number.
    /// * `usize` - The index of the unit.
    fn calculate_parts<U>(&self, value: U) -> (f64, usize)
    where
        U: Zero + AsPrimitive<f64> + PartialEq + Copy,
    {
        if value == U::zero() {
            return (0.0, 0);
        }

        let mut num_value = value.as_();
        let mut index = 0;
        let max_index = self.units.len() - 1;

        while num_value.abs() >= self.division_factor && index < max_index {
            num_value /= self.division_factor;
            index += 1;
        }

        (num_value, index)
    }

    /// Formats a number into a human readable string using the humanizer's units.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use handy::human::Humanizer;
    ///
    /// let humanizer = Humanizer::new(&["", "k", "m", "b", "t"]).with_space_before_unit(false);
    /// assert_eq!(humanizer.format(123_456_789), "123m");
    /// ```
    ///
    /// ## Arguments
    ///
    /// * `value` - The value to format.
    ///
    /// ## Returns
    ///
    /// A human readable string using the humanizer's units.
    pub fn format<U>(&self, value: U) -> String
    where
        U: Zero + AsPrimitive<f64> + PartialEq,
    {
        let (num_value, index) = self.calculate_parts(value);
        let unit = &self.units[index];
        let space = if self.space_before_unit && !unit.is_empty() {
            " "
        } else {
            ""
        };

        if index == 0 && num_value == 0.0 {
            return format!("0{space}{unit}");
        }

        let abs_val = num_value.abs();
        let precision = if abs_val < 10.0 {
            2
        } else {
            usize::from(abs_val < 100.0)
        };

        format!("{num_value:.precision$}{space}{unit}")
    }

    /// Formats a number into a human readable string using the humanizer's units but returns the value and the unit.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use handy::human::Humanizer;
    ///
    /// let humanizer = Humanizer::new(&["", "k", "m", "b", "t"]);
    /// assert_eq!(humanizer.format_as_parts(123_456_789), (123.456789, "m"));
    /// ```
    ///
    /// ## Arguments
    ///
    /// * `value` - The value to format.
    ///
    /// ## Returns
    ///
    /// * `f64` - The value.
    /// * `&str` - The unit.
    pub fn format_as_parts<U>(&self, value: U) -> (f64, &str)
    where
        U: Zero + AsPrimitive<f64> + PartialEq + Copy,
    {
        let (num_value, index) = self.calculate_parts(value);
        (num_value, &self.units[index])
    }
}

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
    binary_humanizer().format(bytes)
}

/// Formats bytes into a human readable string and its unit.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::human::human_bytes_as_parts;
///
/// assert_eq!(human_bytes_as_parts(123_456_789), (118.0, "MiB"));
/// ```
#[must_use]
pub fn human_bytes_as_parts<U>(bytes: U) -> (f64, &'static str)
where
    U: Zero + AsPrimitive<f64> + PartialEq,
{
    binary_humanizer().format_as_parts(bytes)
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
    si_humanizer().format(bytes)
}

/// Formats bytes into a human readable string and its unit using SI units.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::human::human_bytes_si_as_parts;
///
/// assert_eq!(human_bytes_si_as_parts(123_456_789), (118.0, "MB"));
/// ```
#[must_use]
pub fn human_bytes_si_as_parts<U>(bytes: U) -> (f64, &'static str)
where
    U: Zero + AsPrimitive<f64> + PartialEq,
{
    si_humanizer().format_as_parts(bytes)
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
    num_humanizer().format(number)
}

/// Formats a number into a human readable string and its unit.
///
/// ## Examples
///
/// ```rust,no_run
/// use handy::human::human_number_as_parts;
///
/// assert_eq!(human_number_as_parts(123_456_789), (123.0, "M"));
/// ```
pub fn human_number_as_parts<U>(number: U) -> (f64, &'static str)
where
    U: Zero + AsPrimitive<f64> + PartialEq,
{
    num_humanizer().format_as_parts(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_humanizer() {
        let humanizer = Humanizer::new(&["", "k", "m", "b", "t"]).with_space_before_unit(false);

        assert_eq!(humanizer.format(0), "0");
        assert_eq!(humanizer.format(889), "889");
        assert_eq!(humanizer.format(123_456_789), "123m");
        assert_eq!(humanizer.format(1_234_567_890), "1.23b");
        assert_eq!(humanizer.format(12_345_678_901u64), "12.3b");
        assert_eq!(humanizer.format(123_456_789_012u64), "123b");
        assert_eq!(humanizer.format(123_456_789_012_345u64), "123t");

        let humanizer2 = humanizer
            .with_space_before_unit(true)
            .with_division_factor(500);
        assert_eq!(humanizer2.format(0), "0");
        assert_eq!(humanizer2.format(889), "1.78 k");
        assert_eq!(humanizer2.format(123_456_789), "494 m");
        assert_eq!(humanizer2.format(1_234_567_890), "9.88 b");
        assert_eq!(humanizer2.format(12_345_678_901u64), "98.8 b");
        assert_eq!(humanizer2.format(123_456_789_012u64), "1.98 t");
        assert_eq!(humanizer2.format(123_456_789_012_345u64), "1975 t");

        let humanizer3 = Humanizer::new(&["", "k", "m", "b", "t", "qa"]);
        assert_eq!(humanizer3.format_as_parts(0), (0.0, ""));
        assert_eq!(humanizer3.format_as_parts(635), (635.0, ""));
        assert_eq!(humanizer3.format_as_parts(12_345), (12.345, "k"));
        assert_eq!(humanizer3.format_as_parts(1_234_567), (1.234_567, "m"));
        assert_eq!(humanizer3.format_as_parts(123_456_789), (123.456_789, "m"));
        assert_eq!(
            humanizer3.format_as_parts(12_345_678_901u64),
            (12.345_678_901_000_001, "b")
        );
        assert_eq!(
            humanizer3.format_as_parts(123_456_789_012u64),
            (123.456_789_011_999_99, "b")
        );
        assert_eq!(
            humanizer3.format_as_parts(123_456_789_012_345u64),
            (123.456_789_012_345, "t")
        );
        assert_eq!(
            humanizer3.format_as_parts(123_456_789_012_345_678u64),
            (123.456_789_012_345_7, "qa")
        );
    }

    #[test]
    #[should_panic(expected = "Units slice must not be empty")]
    fn test_humanizer_new_empty_units() {
        let _ = Humanizer::new(&[]);
    }

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
        assert_eq!(human_bytes_as_parts(0), (0.0, "B"));
        assert_eq!(human_bytes_as_parts(635), (635.0, "B"));
        assert_eq!(human_bytes_as_parts(12_345), (12.055_664_062_5, "KiB"));
        assert_eq!(
            human_bytes_as_parts(1_234_567),
            (1.177_374_839_782_714_8, "MiB")
        );
        assert_eq!(
            human_bytes_as_parts(123_456_789),
            (117.737_568_855_285_64, "MiB")
        );
        assert_eq!(
            human_bytes_as_parts(12_345_678_901u64),
            (11.497_809_459_455_311, "GiB")
        );
        assert_eq!(
            human_bytes_as_parts(123_456_789_012u64),
            (114.978_094_596_415_76, "GiB")
        );
        assert_eq!(
            human_bytes_as_parts(123_456_789_012_345u64),
            (112.283_295_504_626_04, "TiB")
        );
        assert_eq!(
            human_bytes_as_parts(123_456_789_012_345_678u64),
            (109.651_655_766_236_97, "PiB")
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
        assert_eq!(human_bytes_si_as_parts(0), (0.0, "B"));
        assert_eq!(human_bytes_si_as_parts(635), (635.0, "B"));
        assert_eq!(human_bytes_si_as_parts(12_345), (12.345, "KB"));
        assert_eq!(human_bytes_si_as_parts(1_234_567), (1.234_567, "MB"));
        assert_eq!(human_bytes_si_as_parts(123_456_789), (123.456_789, "MB"));
        assert_eq!(
            human_bytes_si_as_parts(12_345_678_901u64),
            (12.345_678_901_000_001, "GB")
        );
        assert_eq!(
            human_bytes_si_as_parts(123_456_789_012u64),
            (123.456_789_011_999_99, "GB")
        );
        assert_eq!(
            human_bytes_si_as_parts(123_456_789_012_345u64),
            (123.456_789_012_345, "TB")
        );
        assert_eq!(
            human_bytes_si_as_parts(123_456_789_012_345_678u64),
            (123.456_789_012_345_7, "PB")
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
        assert_eq!(human_number_as_parts(0), (0.0, ""));
        assert_eq!(human_number_as_parts(635), (635.0, ""));
        assert_eq!(human_number_as_parts(12_345), (12.345, "K"));
        assert_eq!(human_number_as_parts(1_234_567), (1.234_567, "M"));
        assert_eq!(human_number_as_parts(123_456_789), (123.456_789, "M"));
        assert_eq!(
            human_number_as_parts(12_345_678_901u64),
            (12.345_678_901_000_001, "B")
        );
        assert_eq!(
            human_number_as_parts(123_456_789_012u64),
            (123.456_789_011_999_99, "B")
        );
        assert_eq!(
            human_number_as_parts(123_456_789_012_345u64),
            (123.456_789_012_345, "T")
        );
        assert_eq!(
            human_number_as_parts(123_456_789_012_345_678u64),
            (123.456_789_012_345_7, "Qa")
        );
    }
}
