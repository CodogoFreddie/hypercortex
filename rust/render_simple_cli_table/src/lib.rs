use ansi_term::Style;
use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::hash::Hash;

const GUTTER_WIDTH: usize = 2;

pub fn render_table<
    Header: Display + Eq + Hash,
    Value: Display + Default,
    Hasher: ::std::hash::BuildHasher,
>(
    headers: &[Header],
    header_style: &Style,
    rows: &[(ansi_term::Style, HashMap<Header, Value, Hasher>)],
) -> Result<(), std::fmt::Error> {
    let lines = if let Some((_, height)) = term_size::dimensions() {
        height - 5
    } else {
        40
    };

    let mut widths: HashMap<&Header, usize> = HashMap::new();

    // get widths
    for header in headers {
        widths.insert(header, format!("{}", header).len());
    }
    for (_, row) in rows.iter().take(lines) {
        for (header, cell) in row.iter() {
            widths.entry(header).and_modify(|width| {
                let l = format!("{}", cell).len();
                if l > *width {
                    *width = l
                }
            });
        }
    }

    //print header
    let mut header_string = String::from("");
    for header in headers {
        write!(
            &mut header_string,
            "{0:width$}",
            header,
            width = widths[header] + GUTTER_WIDTH
        )?;
    }
    println!("{}", header_style.paint(header_string));

    //print rows
    for (style, row) in rows.iter().take(lines) {
        let mut row_string = String::from("");
        for header in headers {
            write!(
                &mut row_string,
                "{:1$}",
                row.get(header).unwrap_or(&Value::default()),
                widths[header] + GUTTER_WIDTH
            )?;
        }
        println!("{}", style.paint(row_string));
    }

    Ok(())
}
