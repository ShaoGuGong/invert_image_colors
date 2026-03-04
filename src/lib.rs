use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

#[derive(Debug, Clone, Copy)]
pub struct PpmHeader {
    width: i32,
    high: i32,
    max_val: i32,
}

impl PpmHeader {
    #[inline]
    pub fn new(width: i32, high: i32, max_val: i32) -> Self {
        Self {
            width,
            high,
            max_val,
        }
    }
}

impl fmt::Display for PpmHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P6\n{} {}\n{}\n", self.width, self.high, self.max_val)
    }
}

pub fn read_ppm(input_file: File) -> io::Result<(PpmHeader, Vec<u8>)> {
    let mut reader = BufReader::new(input_file);
    let mut header_vals = Vec::new();
    let mut line = String::new();

    while header_vals.len() < 4 {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "File to short"));
        }
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        for part in trimmed.split_whitespace() {
            header_vals.push(part.to_string());
        }
    }

    if header_vals[0] != "P6" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid File Type. PPM type is not P6",
        ));
    }

    let width: i32 = header_vals[1]
        .parse()
        .map_err(|_| io::ErrorKind::InvalidData)?;
    let high: i32 = header_vals[2]
        .parse()
        .map_err(|_| io::ErrorKind::InvalidData)?;
    let max_val: i32 = header_vals[3]
        .parse()
        .map_err(|_| io::ErrorKind::InvalidData)?;
    let mut pixels = vec![0_u8; (width * high * 3) as usize];
    reader.read_exact(&mut pixels)?;

    Ok((PpmHeader::new(width, high, max_val), pixels))
}

pub fn write_ppm(output_path: &str, header: &PpmHeader, pixels: &[u8]) -> io::Result<()> {
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    write!(writer, "{header}")?;
    writer.write_all(pixels)?;
    writer.flush()?;
    Ok(())
}

#[inline]
pub fn invert_colors(pixels: &mut Vec<u8>) {
    pixels.iter_mut().for_each(|p| {
        *p = !*p;
    });
}
