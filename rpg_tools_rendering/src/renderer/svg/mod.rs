use anyhow::Result;
use std::fs::File;

/// A valid [SVG](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics).
#[derive(Debug, PartialEq, Eq)]
pub struct Svg {
    lines: Vec<String>,
}

impl Svg {
    /// Returns the svg as a single string.
    pub fn export(&self) -> String {
        self.lines.join("\n")
    }

    /// Saves the svg to a file.
    pub fn save(&self, path: &str) -> Result<()> {
        let mut output = File::create(path)?;

        for line in &self.lines {
            writeln!(&mut output, "{}", line)?;
        }

        Ok(())
    }
}
