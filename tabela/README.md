# tabela

**tabela** (Portuguese for "table") is a Rust crate that provides a simple and easy-to-use way to display tabular data in the terminal, with the ability to add colors, styles and alignment to each cell.

I decided to write it because I found myself repeating the same table code over and over again in my projects, which consists of iterating through the data and figuring out the widths of each column based on the longest string in each column, including the header if provided, then writing the headers with correct width to a string, then iterating through the data again and writing each cell with the correct width like `format!("{:<width1$}  {:<width2$}", row.field1, row.field2)`.

## Concepts

* **Table**: A table is a collection of rows, it also stores the header (if provided) and the separator of the cells. **Note that for performance reasons the table stores the rows as `&[&R]` instead of `Vec<R>`**.
* **Row**: A row is a trait that represents a row of data in a table, it must implement the `as_row` method that returns a vector of cells. For example if you have a data of type `Vec<Person>` you'd have to implement the `Row` trait for `&Person`, refer to the example below.
* **Cell**: A cell is a struct that represents a cell in a table, it stores the string value of a type `V` that implements the `Display` trait, as well as the color (optional), style (optional) and alignment (left by default).
* **Color**: Re-export of [`colored::Color`](https://docs.rs/colored/latest/colored/enum.Color.html).
* **CellStyle**: A enum that represents the style of a cell, it can be `Bold`, `Italic` or `Dimmed`.
* **Alignment**: A enum that represents the alignment of a cell, it can be `Left`, `Center` or `Right`.
* **TableError**: A enum that represents the errors that can occur when formatting a table.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tabela = "^0.2"
```

Or install it with `cargo add tabela`.

## Usage

```rust
use tabela::{Alignment, Cell, Color, Row, Table};

// row type
struct Person {
    name: String,
    age: u8,
}

// row implementation
impl Row for &Person {
    fn as_row(&self) -> Vec<Cell> {
        vec![
            self.name.clone().into(),
            // you'd use `Cell::new(&self.name)` instead to avoid cloning a string and to add color/style/change alignment

            Cell::new(self.age).with_color(Color::Cyan).with_alignment(Alignment::Center),
            // adds the age field with cyan color and center alignment,
            // since `u8` already has a `Display` implementation you
            // don't need to do anything, if something doesn't have a `Display` impl
            // you can manually turn it to a `String` or `&str` and feed that into `Cell::new`
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
let data_refs: Vec<&Person> = data.iter().collect();  // rows have to be references, in the future maybe
                                                      // support for owned rows will be added

let table = Table::new(&data_refs)
    .with_header(&["Name", "Some Age"], None, Some(CellStyle::Bold), None)  // adds header with bold style
    .with_separator("  ");  // uses two spaces as separator (personal preference)

let formatted = table.format().unwrap();  // errors can only happen in `Table::format` if the
                                          // header length is different than the row length or
                                          // if the other rows' length is different than the first row length
                                          // so I wouldn't worry about using `unwrap` here

println!("{formatted}");

// output (without color/style characters):
//
// Name    Some Age
// Johnny     30
// Jane       25
// (extra '\n' at the end)
```

## Tests

Run the tests with `cargo test`.

## Benchmarks

TBA

## License

This crate is distributed under the terms of the [MIT license](LICENSE).
