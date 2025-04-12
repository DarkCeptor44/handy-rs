#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod errors;

pub use colored::Color;
use colored::{ColoredString, Colorize};
pub use errors::{Result, TableError};
use std::fmt::{Display, Write as _};
use unicode_width::UnicodeWidthStr;

// TODO implement halfwidth to center the cells

/// A trait that represents a row of data in a [Table]
///
/// Note: This will typically be implemented for a REFERENCE type (e.g., `impl Row for &MyData`)
pub trait Row {
    /// Returns the row as a vector of [cells](Cell)
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use tabela::{Cell, Color, Row};
    ///
    /// struct Person {
    ///     name: String,
    ///     age: u8,
    /// }
    ///
    /// impl Row for &Person {
    ///     fn as_row(&self) -> Vec<Cell> {
    ///         vec![
    ///             self.name.clone().into(),
    ///             Cell::new(self.age).with_color(Color::Cyan),
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// ## Returns
    ///
    /// A vector of [cells](Cell)
    fn as_row(&self) -> Vec<Cell>;
}

/// A struct that represents a cell in a [Table]
#[derive(Debug)]
pub struct Cell {
    pub value: String,
    pub color: Option<Color>,
    pub style: Option<CellStyle>,
}

impl Cell {
    /// Creates a new [Cell] with the given value
    ///
    /// ## Arguments
    ///
    /// * `value` - The value to add to the cell
    ///
    /// ## Returns
    ///
    /// A new [Cell] with the given value
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use tabela::Cell;
    ///
    /// let str_cell: Cell = "Hello".into();
    /// let int_cell = Cell::new(42);
    /// ```
    #[must_use]
    pub fn new<V>(value: V) -> Self
    where
        V: Display,
    {
        Cell {
            value: value.to_string(),
            color: None,
            style: None,
        }
    }

    /// Sets the color of the [Cell]
    ///
    /// ## Arguments
    ///
    /// * `color` - The color of the cell
    ///
    /// ## Returns
    ///
    /// A new [Cell] with the given color
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use tabela::{Cell, Color};
    ///
    /// let cell = Cell::new("This is a blue-colored string").with_color(Color::Blue);
    /// ```
    #[must_use]
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the style of the [Cell]
    ///
    /// ## Arguments
    ///
    /// * `style` - The style of the cell
    ///
    /// ## Returns
    ///
    /// A new [Cell] with the given style
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use tabela::{Cell, CellStyle};
    ///
    /// let cell = Cell::new("This is a bold string").with_style(CellStyle::Bold);
    /// ```
    #[must_use]
    pub fn with_style(mut self, style: CellStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.value.as_str();
        let colored_value: ColoredString;

        if let Some(color) = self.color {
            colored_value = value.color(color);
        } else {
            colored_value = value.normal();
        }

        match self.style {
            Some(CellStyle::Bold) => write!(f, "{}", colored_value.bold()),
            Some(CellStyle::Dimmed) => write!(f, "{}", colored_value.dimmed()),
            Some(CellStyle::Italic) => write!(f, "{}", colored_value.italic()),
            None => write!(f, "{colored_value}"),
        }
    }
}

impl From<String> for Cell {
    fn from(value: String) -> Self {
        Cell {
            value,
            color: None,
            style: None,
        }
    }
}

impl From<&str> for Cell {
    fn from(value: &str) -> Self {
        Cell {
            value: value.to_string(),
            color: None,
            style: None,
        }
    }
}

/// A enum that represents the style of a [Cell]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellStyle {
    Bold,
    Italic,
    Dimmed,
}

/// A struct that represents a table
#[derive(Debug)]
pub struct Table<'a, R>
where
    R: 'a,
{
    pub header: Vec<Cell>,
    pub rows: &'a [&'a R],
    pub separator: String,
}

impl<'a, R> Table<'a, R> {
    /// Creates a new [Table] with the given rows
    ///
    /// ## Arguments
    ///
    /// * `rows` - The rows to add to the table, they must implement the [Row] trait
    ///
    /// ## Returns
    ///
    /// A new [Table] with the given rows
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use tabela::{Cell, Color, Row, Table};
    ///
    /// struct Person {
    ///     name: String,
    ///     age: u8,
    /// }
    ///
    /// impl Row for &Person {
    ///     fn as_row(&self) -> Vec<Cell> {
    ///         vec![
    ///             self.name.clone().into(),
    ///             Cell::new(self.age).with_color(Color::Cyan),
    ///         ]
    ///     }
    /// }
    ///
    /// let data = [
    ///     Person {
    ///         name: "Johnny".into(),
    ///         age: 30,
    ///     },
    ///     Person {
    ///         name: "Jane".into(),
    ///         age: 25,
    ///     },
    /// ];
    /// let data_refs: Vec<&Person> = data.iter().collect();
    /// let table: Table<Person> = Table::new(&data_refs);
    /// ```
    #[must_use]
    pub fn new(rows: &'a [&'a R]) -> Self {
        Table {
            header: Vec::new(),
            rows,
            separator: String::from(" "),
        }
    }

    /// Adds a header to the table with [bold](CellStyle::Bold) style
    ///
    /// ## Arguments
    ///
    /// * `header` - The header to add to the table
    /// * `color` - The color of the header cells
    /// * `style` - The style of the header cells
    ///
    /// ## Returns
    ///
    /// A new [Table] with the given header
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use tabela::{Cell, CellStyle, Color, Row, Table};
    ///
    /// struct Person {
    ///     name: String,
    ///     age: u8,
    /// }
    ///
    /// impl Row for &Person {
    ///     fn as_row(&self) -> Vec<Cell> {
    ///         vec![
    ///             Cell::new(&self.name).with_color(Color::Green),
    ///             Cell::new(self.age).with_color(Color::Cyan),
    ///         ]
    ///     }
    /// }
    ///
    /// let data = [
    ///     Person {
    ///         name: "Johnny".into(),
    ///         age: 30,
    ///     },
    ///     Person {
    ///         name: "Jane".into(),
    ///         age: 25,
    ///     },
    /// ];
    /// let data_refs: Vec<&Person> = data.iter().collect();
    /// let table: Table<'_, Person> = Table::new(&data_refs).with_header(&["Name", "Age"], None, Some(CellStyle::Bold));
    /// ```
    #[must_use]
    pub fn with_header(
        mut self,
        header: &[&str],
        color: Option<Color>,
        style: Option<CellStyle>,
    ) -> Self {
        self.header = header
            .iter()
            .map(|&s| {
                let mut c = Cell::from(s);

                if let Some(col) = color {
                    c = c.with_color(col);
                }

                if let Some(st) = style {
                    c = c.with_style(st);
                }

                c
            })
            .collect();
        self
    }

    /// Sets the separator for the table cells.
    ///
    /// Default: `" "`
    ///
    /// ## Arguments
    ///
    /// * `separator` - The separator to use for the table
    ///
    /// ## Returns
    ///
    /// A new [Table] with the given separator
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use tabela::{Cell, Color, Row, Table};
    ///
    /// struct Person {
    ///     name: String,
    ///     age: u8,
    /// }
    ///
    /// impl Row for &Person {
    ///     fn as_row(&self) -> Vec<Cell> {
    ///         vec![
    ///             Cell::new(&self.name).with_color(Color::Green),
    ///             Cell::new(self.age).with_color(Color::Cyan),
    ///         ]
    ///     }
    /// }
    ///
    /// let data = [
    ///     Person {
    ///         name: "Johnny".into(),
    ///         age: 30,
    ///     },
    ///     Person {
    ///         name: "Jane".into(),
    ///         age: 25,
    ///     },
    /// ];
    /// let data_refs: Vec<&Person> = data.iter().collect();
    /// let table: Table<'_, Person> = Table::new(&data_refs).with_separator(" | ");
    /// ```
    #[must_use]
    pub fn with_separator<S>(mut self, separator: S) -> Self
    where
        S: AsRef<str>,
    {
        self.separator = separator.as_ref().to_string();
        self
    }
}

impl<'a, R> Table<'a, R>
where
    &'a R: Row,
{
    /// Formats the table into a string
    ///
    /// ## Returns
    ///
    /// A string representation of the table
    ///
    /// ## Errors
    ///
    /// - [`TableError::HeaderLengthMismatch`]: Header length does not match row length
    /// - [`TableError::RowLengthMismatch`]: Row length does not match first row length
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use tabela::{Cell, CellStyle, Color, Row, Table};
    ///
    /// struct Person {
    ///     name: String,
    ///     age: u8,
    /// }
    ///
    /// impl Row for &Person {
    ///     fn as_row(&self) -> Vec<Cell> {
    ///         vec![
    ///             Cell::new(&self.name).with_color(Color::Green),
    ///             Cell::new(self.age).with_color(Color::Cyan),
    ///         ]
    ///     }
    /// }
    ///
    /// let data = [
    ///     Person {
    ///         name: "Johnny".into(),
    ///         age: 30,
    ///     },
    ///     Person {
    ///         name: "Jane".into(),
    ///         age: 25,
    ///     },
    /// ];
    /// let data_refs: Vec<&Person> = data.iter().collect();
    /// let table: Table<'_, Person> = Table::new(&data_refs).with_header(&["Name", "Age"], None, Some(CellStyle::Bold));
    /// let formatted = table.format().unwrap();
    ///
    /// println!("{formatted}");
    /// ```
    pub fn format(&self) -> Result<String> {
        let mut col_widths: Vec<usize> = Vec::new();
        if !self.header.is_empty() {
            col_widths = self
                .header
                .iter()
                .map(|c| UnicodeWidthStr::width(c.value.as_str()))
                .collect();
        }

        if !self.rows.is_empty() {
            let first_row_len = self.rows[0].as_row().len();

            if !self.header.is_empty() && self.header.len() != first_row_len {
                return Err(TableError::HeaderLengthMismatch(
                    self.header.len(),
                    first_row_len,
                ));
            }

            if self.header.is_empty() && first_row_len > 0 {
                col_widths = vec![0; first_row_len];
            }

            for row in self.rows {
                let row_values = row.as_row();

                if row_values.len() != col_widths.len() && !self.header.is_empty() {
                    return Err(TableError::HeaderLengthMismatch(
                        row_values.len(),
                        col_widths.len(),
                    ));
                }
                if row_values.len() != first_row_len {
                    return Err(TableError::RowLengthMismatch(
                        row_values.len(),
                        first_row_len,
                    ));
                }

                for (i, value) in row_values.iter().enumerate() {
                    let cell_content_width = UnicodeWidthStr::width(value.value.as_str());
                    if i < col_widths.len() {
                        col_widths[i] = col_widths[i].max(cell_content_width);
                    } else {
                        col_widths.push(cell_content_width);
                    }
                }
            }
        }

        let mut output = String::new();
        if !self.header.is_empty() {
            for (i, header_cell) in self.header.iter().enumerate() {
                if i < col_widths.len() {
                    let header_display = format!("{header_cell}");
                    let header_content_width = UnicodeWidthStr::width(header_cell.value.as_str());
                    let required_width = col_widths[i];
                    let padding = required_width.saturating_sub(header_content_width);

                    write!(output, "{}{}", header_display, " ".repeat(padding)).unwrap();

                    if i < self.header.len() - 1 {
                        write!(output, "{}", self.separator).unwrap();
                    }
                } else {
                    write!(output, "{header_cell}").unwrap();

                    if i < self.header.len() - 1 {
                        write!(output, "{}", self.separator).unwrap();
                    }
                }
            }

            writeln!(output).unwrap();
        }

        for row in self.rows {
            let row_values = row.as_row();
            for (i, value_cell) in row_values.iter().enumerate() {
                if i >= col_widths.len() {
                    write!(output, "{value_cell}").unwrap();
                } else {
                    let value_display = format!("{value_cell}");
                    let value_content_width = UnicodeWidthStr::width(value_cell.value.as_str());
                    let required_width = col_widths[i];
                    let padding = required_width.saturating_sub(value_content_width);

                    write!(output, "{value_display}{}", " ".repeat(padding)).unwrap();
                }

                if i < row_values.len() - 1 {
                    write!(output, "{}", self.separator).unwrap();
                }
            }

            writeln!(output).unwrap();
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use handy::iter::IntoRefVec;

    #[test]
    fn test_table() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        impl Row for &Person {
            fn as_row(&self) -> Vec<Cell> {
                vec![self.name.clone().into(), Cell::new(self.age)]
            }
        }

        let data = [
            Person {
                name: "Johnny".into(),
                age: 30,
            },
            Person {
                name: "Jane".into(),
                age: 25,
            },
        ];
        let data_refs = data.as_ref_vec();
        let table = Table::new(&data_refs)
            .with_header(&["Name", "Age"], None, None)
            .with_separator("  ");
        let formatted = dbg!(table).format().unwrap();
        assert_eq!(formatted, "Name    Age\nJohnny  30 \nJane    25 \n");
    }

    #[test]
    fn test_table_colored() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        impl Row for &Person {
            fn as_row(&self) -> Vec<Cell> {
                vec![
                    self.name.clone().into(),
                    Cell::new(self.age).with_color(Color::Cyan),
                ]
            }
        }

        let data = [
            Person {
                name: "Johnny".into(),
                age: 30,
            },
            Person {
                name: "Jane".into(),
                age: 25,
            },
        ];
        let data_refs = data.as_ref_vec();
        let table = Table::new(&data_refs)
            .with_header(&["Name", "Age"], None, Some(CellStyle::Bold))
            .with_separator("  ");
        let formatted = dbg!(table).format().unwrap();
        assert_eq!(
            formatted,
            "\u{1b}[1mName\u{1b}[0m    \u{1b}[1mAge\u{1b}[0m\nJohnny  \u{1b}[36m30\u{1b}[0m \nJane    \u{1b}[36m25\u{1b}[0m \n"
        );
    }

    #[test]
    fn test_table_empty_header() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        impl Row for &Person {
            fn as_row(&self) -> Vec<Cell> {
                vec![self.name.clone().into(), Cell::new(self.age)]
            }
        }

        let data = [
            Person {
                name: "Johnny".into(),
                age: 30,
            },
            Person {
                name: "Jane".into(),
                age: 25,
            },
        ];
        let data_refs = data.as_ref_vec();
        let table = Table::new(&data_refs);
        let formatted = dbg!(table).format().unwrap();
        assert_eq!(formatted, "Johnny 30\nJane   25\n");
    }

    #[test]
    fn test_table_empty_rows() {
        #[derive(Debug)]
        struct Person;

        impl Row for &Person {
            fn as_row(&self) -> Vec<Cell> {
                vec![]
            }
        }

        let data = [];
        let data_refs = data.as_ref_vec();
        let table: Table<'_, Person> = Table::new(&data_refs)
            .with_header(&["Name", "Age"], None, None)
            .with_separator(" | ");
        let formatted = dbg!(table).format().unwrap();
        assert_eq!(formatted, "Name | Age\n");
    }

    #[test]
    #[should_panic(expected = "HeaderLengthMismatch(1, 2)")]
    fn test_table_wrong_header_length() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        impl Row for &Person {
            fn as_row(&self) -> Vec<Cell> {
                vec![self.name.clone().into(), Cell::new(self.age)]
            }
        }

        let data = [
            Person {
                name: "Johnny".into(),
                age: 30,
            },
            Person {
                name: "Jane".into(),
                age: 25,
            },
        ];
        let data_refs = data.as_ref_vec();
        let table = Table::new(&data_refs).with_header(&["Name"], None, None);
        dbg!(table).format().unwrap();
    }
}
