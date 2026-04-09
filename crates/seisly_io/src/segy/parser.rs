use anyhow::Result;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct SegyMetadata {
    pub inline_min: i32,
    pub inline_max: i32,
    pub crossline_min: i32,
    pub crossline_max: i32,
    pub sample_count: usize,
    pub sample_interval: f32,
    pub format: u16, // 1: IBM float, 5: IEEE float
}

pub fn parse_metadata(path: &Path) -> Result<SegyMetadata> {
    let mut file = File::open(path)?;

    // Skip 3200-byte EBCDIC/ASCII header
    file.seek(SeekFrom::Start(3200))?;

    // Read 400-byte binary header
    let mut binary_header = [0u8; 400];
    file.read_exact(&mut binary_header)?;

    // Extract key fields (Big-Endian)
    let sample_interval = u16::from_be_bytes([binary_header[16], binary_header[17]]) as f32;
    let sample_count = u16::from_be_bytes([binary_header[20], binary_header[21]]) as usize;
    let format = u16::from_be_bytes([binary_header[24], binary_header[25]]);

    // Read the first trace header to get initial inline/crossline values.
    // First trace header starts at file offset 3600 (after text + binary headers).
    // Inline = bytes 188-191 BE, Crossline = bytes 192-195 BE.
    file.seek(SeekFrom::Start(3600))?;
    let mut first_trace_header = [0u8; 240];
    let iline: i32;
    let xline: i32;

    if file.read_exact(&mut first_trace_header).is_ok() {
        iline = i32::from_be_bytes([
            first_trace_header[188],
            first_trace_header[189],
            first_trace_header[190],
            first_trace_header[191],
        ]);
        xline = i32::from_be_bytes([
            first_trace_header[192],
            first_trace_header[193],
            first_trace_header[194],
            first_trace_header[195],
        ]);
    } else {
        // Fallback if we can't read the first trace header.
        iline = 1;
        xline = 1;
    }

    Ok(SegyMetadata {
        inline_min: iline,
        inline_max: iline,
        crossline_min: xline,
        crossline_max: xline,
        sample_count,
        sample_interval,
        format,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_binary_header() {
        let mut tmp = NamedTempFile::new().unwrap();

        // Write mock 3200-byte header
        let text_header = [0u8; 3200];
        tmp.write_all(&text_header).unwrap();

        // Write mock 400-byte binary header
        let mut binary_header = [0u8; 400];
        // Sample interval: 4000 (at bytes 16-17)
        binary_header[16] = 0x0F;
        binary_header[17] = 0xA0;
        // Sample count: 500 (at bytes 20-21)
        binary_header[20] = 0x01;
        binary_header[21] = 0xF4;
        // Format: 5 (at bytes 24-25)
        binary_header[25] = 0x05;

        tmp.write_all(&binary_header).unwrap();

        // Write mock first trace header with inline=100, xline=200
        let mut trace_header = [0u8; 240];
        let il = 100i32.to_be_bytes();
        let xl = 200i32.to_be_bytes();
        trace_header[188..192].copy_from_slice(&il);
        trace_header[192..196].copy_from_slice(&xl);
        tmp.write_all(&trace_header).unwrap();

        let metadata = parse_metadata(tmp.path()).unwrap();
        assert_eq!(metadata.sample_count, 500);
        assert_eq!(metadata.sample_interval, 4000.0);
        assert_eq!(metadata.format, 5);
        assert_eq!(metadata.inline_min, 100);
        assert_eq!(metadata.crossline_min, 200);
    }

    #[test]
    fn test_parse_metadata_no_trace_header() {
        let mut tmp = NamedTempFile::new().unwrap();

        // Write only the 3600-byte headers, no trace data
        let text_header = [0u8; 3200];
        tmp.write_all(&text_header).unwrap();

        let mut binary_header = [0u8; 400];
        binary_header[16] = 0x0F;
        binary_header[17] = 0xA0;
        binary_header[20] = 0x01;
        binary_header[21] = 0xF4;
        binary_header[25] = 0x05;
        tmp.write_all(&binary_header).unwrap();

        // Should fall back to (1, 1) since we can't read trace header
        let metadata = parse_metadata(tmp.path()).unwrap();
        assert_eq!(metadata.inline_min, 1);
        assert_eq!(metadata.crossline_min, 1);
    }
}
