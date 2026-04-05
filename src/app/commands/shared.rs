use anyhow::Result;

use crate::core::processor::LineProcessor;

pub enum OutputMode {
    PreserveExisting,
    Append,
}

pub fn emit_processed_line(line: &str, processor: &LineProcessor, output_mode: OutputMode) -> Result<()> {
    if let Some(processed_line) = processor.process(line) {
        match output_mode {
            OutputMode::PreserveExisting => print!("{processed_line}"),
            OutputMode::Append => println!("{processed_line}"),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{emit_processed_line, OutputMode};
    use crate::core::processor::LineProcessor;

    #[test]
    fn processed_line_can_be_written_without_error() {
        let processor = LineProcessor::new(None, None, true).unwrap();

        assert!(emit_processed_line("plain log line", &processor, OutputMode::Append).is_ok());
        assert!(emit_processed_line("plain log line\n", &processor, OutputMode::PreserveExisting).is_ok());
    }
}
