use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};

pub fn write(outfile: &str, buffer: &[u8]) -> Result<bool> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    if let Err(e) = writer.write_all(&buffer) {
        if e.kind() == ErrorKind::BrokenPipe {
            // false means "stop the program cleanly"
            return Ok(false);
        }
        return Err(e);
    }

    // true means "keep going"
    Ok(true)
}
