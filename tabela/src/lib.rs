//! # tabela
//!
//! **tabela** (Portuguese for "table") is a Rust crate that provides a simple and easy-to-use way to display tabular data in the terminal, with the ability to add colors, styles and alignment to each cell.
//!
//! I decided to write it because I found myself repeating the same table code over and over again in my projects, which consists of iterating through the data and figuring out the widths of each column based on the longest string in each column, including the header if provided, then writing the headers with correct width to a string, then iterating through the data again and writing each cell with the correct width like `format!("{:<width1$}  {:<width2$}", row.field1, row.field2)`.
//!
//! ## Concepts
//!
//! * **Table**: A table is a collection of rows, it also stores the header (if provided) and the separator of the cells. **Note that for performance reasons the table stores the rows as `&[&R]` instead of `Vec<R>`**.
//! * **Row**: A row is a trait that represents a row of data in a table, it must implement the `as_row` method that returns a vector of cells. For example if you have a data of type `Vec<Person>` you'd have to implement the `Row` trait for `&Person`, refer to the example below.
//! * **Cell**: A cell is a struct that represents a cell in a table, it stores the string value of a type `V` that implements the `Display` trait, as well as the color (optional), style (optional) and alignment (left by default).
//! * **Color**: Re-export of [`colored::Color`](https://docs.rs/colored/latest/colored/enum.Color.html).
//! * **CellStyle**: A enum that represents the style of a cell, it can be `Bold`, `Italic` or `Dimmed`.
//! * **Alignment**: A enum that represents the alignment of a cell, it can be `Left`, `Center` or `Right`.
//! * **TableError**: A enum that represents the errors that can occur when formatting a table.
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tabela = "^0.2"
//! ```
//!
//! Or install it with `cargo add tabela`.
//!
//! ## Usage
//!
//! ```rust
//! use tabela::{Alignment, Cell, Color, Row, Table};
//!
//! // row type
//! struct Person {
//!     name: String,
//!     age: u8,
//! }
//!
//! // row implementation
//! impl Row for &Person {
//!     fn as_row(&self) -> Vec<Cell> {
//!         vec![
//!             self.name.clone().into(),
//!             // you'd use `Cell::new(&self.name)` instead to avoid cloning a string and to add color/style/change alignment
//!
//!             Cell::new(self.age).with_color(Color::Cyan).with_alignment(Alignment::Center),
//!             // adds the age field with cyan color and center alignment,
//!             // since `u8` already has a `Display` implementation you
//!             // don't need to do anything, if something doesn't have a `Display` impl
//!             // you can manually turn it to a `String` or `&str` and feed that into `Cell::new`
//!         ]
//!     }
//! }
//!
//! let data = [
//!     Person {
//!         name: "Johnny".into(),
//!         age: 30,
//!     },
//!     Person {
//!         name: "Jane".into(),
//!         age: 25,
//!     },
//! ];
//! let data_refs: Vec<&Person> = data.iter().collect();  // rows have to be references, in the future maybe
//!                                                       // support for owned rows will be added
//!
//! let table = Table::new(&data_refs)
//!     .with_header(&["Name", "Some Age"], None, Some(CellStyle::Bold), None)  // adds header with bold style
//!     .with_separator("  ");  // uses two spaces as separator (personal preference)
//!
//! let formatted = table.format().unwrap();  // errors can only happen in `Table::format` if the
//!                                           // header length is different than the row length or
//!                                           // if the other rows' length is different than the first row length
//!                                           // so I wouldn't worry about using `unwrap` here
//!
//! println!("{formatted}");
//!
//! // output (without color/style characters):
//! //
//! // Name    Some Age
//! // Johnny     30
//! // Jane       25
//! // (extra '\n' at the end)
//! ```

#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_debug_implementations)]

mod errors;

pub use colored::Color;
use colored::{ColoredString, Colorize};
pub use errors::{Result, TableError};
use std::fmt::{Display, Write as _};
use unicode_width::UnicodeWidthStr;

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
    pub alignment: Alignment,
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
            alignment: Alignment::Left,
        }
    }

    /// Sets the alignment of the [Cell].
    ///
    /// Default: [`Alignment::Left`], which means space is added to the end of the cell value
    ///
    /// ## Arguments
    ///
    /// * `alignment` - The alignment of the cell
    ///
    /// ## Returns
    ///
    /// A new [Cell] with the given alignment
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use tabela::{Cell, Alignment};
    ///
    /// let cell = Cell::new("This is a centered string").with_alignment(Alignment::Center);
    /// ```
    #[must_use]
    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
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
            alignment: Alignment::Left,
        }
    }
}

impl From<&str> for Cell {
    fn from(value: &str) -> Self {
        Cell {
            value: value.to_string(),
            color: None,
            style: None,
            alignment: Alignment::Left,
        }
    }
}

/// A enum that represents the style of a [Cell]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellStyle {
    /// Makes the cell bold
    Bold,

    /// Makes the cell italic
    Italic,

    /// Makes the cell dimmed
    Dimmed,
}

/// A enum that represents the alignment of a [Cell]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    /// Aligns the cell to the left
    Left,

    /// Aligns the cell to the center
    Center,

    /// Aligns the cell to the right
    Right,
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

    /// Adds a header to the [Table] with optional color, style and alignment
    ///
    /// ## Arguments
    ///
    /// * `header` - The header to add to the table
    /// * `color` - The color of the header cells
    /// * `style` - The style of the header cells
    /// * `alignment` - The alignment of the header cells
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
    /// let table: Table<'_, Person> = Table::new(&data_refs).with_header(&["Name", "Age"], None, Some(CellStyle::Bold), None);
    /// ```
    #[must_use]
    pub fn with_header(
        mut self,
        header: &[&str],
        color: Option<Color>,
        style: Option<CellStyle>,
        alignment: Option<Alignment>,
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

                if let Some(al) = alignment {
                    c = c.with_alignment(al);
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
    /// let table: Table<'_, Person> = Table::new(&data_refs).with_header(&["Name", "Age"], None, Some(CellStyle::Bold), None);
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

                    format_cell(&mut output, header_cell.alignment, &header_display, padding);

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

                    format_cell(&mut output, value_cell.alignment, &value_display, padding);
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

/// Formats a [Cell] to a string.
///
/// ## Arguments
///
/// * `output` - The string to write to.
/// * `alignment` - The alignment of the cell.
/// * `value` - The value of the cell.
/// * `padding` - The padding to add to the cell.
fn format_cell(output: &mut String, alignment: Alignment, value: &str, padding: usize) {
    match alignment {
        Alignment::Left => {
            write!(output, "{value}{}", " ".repeat(padding)).unwrap();
        }
        Alignment::Center => {
            let left_padding = padding / 2;
            let right_padding = padding - left_padding;

            write!(
                output,
                "{}{value}{}",
                " ".repeat(left_padding),
                " ".repeat(right_padding)
            )
            .unwrap();
        }
        Alignment::Right => {
            write!(output, "{}{value}", " ".repeat(padding)).unwrap();
        }
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
            .with_header(&["Name", "Age"], None, None, None)
            .with_separator("  ");
        let formatted = dbg!(table).format().unwrap();
        assert_eq!(formatted, "Name    Age\nJohnny  30 \nJane    25 \n");

        // Output:
        //
        // Name    Age
        // Johnny  30
        // Jane    25
    }

    #[test]
    fn test_table_centered_header() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        impl Row for &Person {
            fn as_row(&self) -> Vec<Cell> {
                vec![
                    Cell::new(&self.name).with_alignment(Alignment::Right),
                    Cell::new(self.age).with_alignment(Alignment::Center),
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
            .with_header(&["Name", "Some Age"], None, None, Some(Alignment::Center))
            .with_separator("  ");
        let formatted = dbg!(table).format().unwrap();
        assert_eq!(
            formatted,
            " Name   Some Age\nJohnny     30   \n  Jane     25   \n"
        );

        // Output:
        //
        //  Name   Some Age
        // Johnny     30
        //   Jane     25
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
            .with_header(&["Name", "Age"], None, Some(CellStyle::Bold), None)
            .with_separator("  ");
        let formatted = dbg!(table).format().unwrap();
        assert_eq!(
            formatted,
            "\u{1b}[1mName\u{1b}[0m    \u{1b}[1mAge\u{1b}[0m\nJohnny  \u{1b}[36m30\u{1b}[0m \nJane    \u{1b}[36m25\u{1b}[0m \n"
        );

        // Output:
        //
        // Name    Age
        // Johnny  30
        // Jane    25
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

        // Output:
        //
        // Johnny 30
        // Jane   25
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
            .with_header(&["Name", "Age"], None, None, None)
            .with_separator(" | ");
        let formatted = dbg!(table).format().unwrap();
        assert_eq!(formatted, "Name | Age\n");

        // Output:
        //
        // Name | Age
    }

    #[test]
    fn test_table_with_alignment() {
        #[derive(Debug)]
        struct Person<'a> {
            name: &'a str,
            age: u8,
            number: u32,
        }

        impl Row for &Person<'_> {
            fn as_row(&self) -> Vec<Cell> {
                vec![
                    Cell::new(self.name),
                    Cell::new(self.age).with_alignment(Alignment::Center),
                    Cell::new(self.number).with_alignment(Alignment::Right),
                ]
            }
        }

        let data = [
            Person {
                name: "Johnny",
                age: 30,
                number: 1,
            },
            Person {
                name: "Jane",
                age: 25,
                number: 2,
            },
        ];
        let data_refs = data.as_ref_vec();
        let table = Table::new(&data_refs)
            .with_header(
                &["Person's name", "Person's Age", "Number"],
                None,
                None,
                None,
            )
            .with_separator(" | ");
        let formatted = dbg!(table).format().unwrap();
        assert_eq!(formatted, "Person's name | Person's Age | Number\nJohnny        |      30      |      1\nJane          |      25      |      2\n");

        // Output:
        //
        // Person's name | Person's Age | Number
        // Johnny        |      30      |      1
        // Jane          |      25      |      2
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
        let table = Table::new(&data_refs).with_header(&["Name"], None, None, None);
        dbg!(table).format().unwrap();
    }
}
