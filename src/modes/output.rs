use anyhow::Result;

use crate::formatter::lineformatter::LineFormatter;

pub enum NewlineMode {
    PreserveExisting,
    Append,
}

pub fn write_processed_line(line: &str, formatter: &LineFormatter, newline_mode: NewlineMode) -> Result<()> {
    if let Some(processed_line) = formatter.process_line(line) {
        match newline_mode {
            NewlineMode::PreserveExisting => print!("{processed_line}"),
            NewlineMode::Append => println!("{processed_line}"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{write_processed_line, NewlineMode};
    use crate::formatter::lineformatter::LineFormatter;

    #[test]
    fn processed_line_can_be_written_without_error() {
        let formatter = LineFormatter::new(None, None, true).unwrap();

        assert!(write_processed_line("plain log line", &formatter, NewlineMode::Append).is_ok());
        assert!(write_processed_line("plain log line\n", &formatter, NewlineMode::PreserveExisting).is_ok());
    }
}
