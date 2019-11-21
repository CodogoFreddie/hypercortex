use ansi_term::Style;
use hypertask_engine::prelude::*;
use std::collections::HashMap;
use std::fmt::{Display, Write};
use std::hash::Hash;

const GUTTER_WIDTH: usize = 2;

pub fn render_table<Header: Display + Eq + Hash>(
    headers: &[Header],
    header_style: &Style,
    rows: &Vec<(ansi_term::Style, HashMap<Header, String>)>,
) -> HyperTaskResult<()> {
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
                let l = cell.len();
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
        )
        .map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Render, HyperTaskErrorAction::Write).from(e)
        })?;
    }
    println!("{}", header_style.paint(header_string));

    //print rows

    for (style, row) in rows.iter().take(lines) {
        let mut row_string = String::from("");
        for header in headers {
            write!(
                &mut row_string,
                "{:1$}",
                row.get(header).unwrap_or(&String::from("")),
                widths[header] + GUTTER_WIDTH
            )
            .map_err(|e| {
                HyperTaskError::new(HyperTaskErrorDomain::Render, HyperTaskErrorAction::Write)
                    .from(e)
            })?;
        }
        println!("{}", style.paint(row_string));
    }

    Ok(())
}
