//! SEG-Y file reader with memory-mapped access
//!
//! Hybrid approach:
//! - segy-rs: Header parsing (EBCDIC decoding, binary header)
//! - memmap2: Zero-copy trace data access
//! - Custom: Optimized I/O layer

use memmap2::Mmap;
use segy_rs::{SegyFile, BinaryHeader, TraceHeader};
use std::fs::File;
use std::path::Path;
use thiserror::Error;

/// IO error type for SEG-Y operations
#[derive(Error, Debug)]
pub enum IoError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// SEG-Y volume reader
pub struct SegyReader {
    mmap: Mmap,
    segy: SegyFile,
}

impl SegyReader {
    /// Open a SEG-Y file with memory mapping
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, IoError> {
        let file = File::open(path.as_ref())?;
        let mmap = unsafe { Mmap::map(&file)? };
        
        // Parse SEG-Y from memory-mapped bytes
        let segy = SegyFile::from_bytes(&mmap)
            .map_err(|e| IoError::ParseError(format!("Failed to parse SEG-Y file: {}", e)))?;

        Ok(Self { mmap, segy })
    }

    /// Get textual header (EBCDIC/ASCII)
    pub fn textual_header(&self) -> &str {
        self.segy.textual_header()
    }

    /// Get binary header
    pub fn binary_header(&self) -> Result<&BinaryHeader, IoError> {
        Ok(self.segy.binary_header())
    }

    /// Get number of traces
    pub fn trace_count(&self) -> usize {
        self.segy.trace_count()
    }

    /// Read a trace by index
    pub fn read_trace(&self, index: usize) -> Result<Vec<f32>, IoError> {
        let trace = self.segy
            .trace(index)
            .map_err(|e| IoError::ParseError(format!("Failed to read trace {}: {}", index, e)))?;
        
        // Convert trace data to Vec<f32>
        let data = trace.iter()
            .map(|sample| sample as f32)
            .collect();
        
        Ok(data)
    }

    /// Get trace header at index
    pub fn trace_header(&self, index: usize) -> Result<&TraceHeader, IoError> {
        self.segy
            .trace_header(index)
            .map_err(|e| IoError::ParseError(format!("Failed to read trace header {}: {}", index, e)))
    }
}

/// Extended binary header with convenient accessors
pub struct ExtendedBinaryHeader {
    pub sample_rate: u32,
    pub trace_count: u32,
    pub samples_per_trace: u32,
    pub data_format: u16,
}

impl From<&BinaryHeader> for ExtendedBinaryHeader {
    fn from(header: &BinaryHeader) -> Self {
        Self {
            sample_rate: header.sample_rate(),
            trace_count: header.trace_count(),
            samples_per_trace: header.samples_per_trace(),
            data_format: header.data_format(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segy_reader_error_on_missing_file() {
        let result = SegyReader::open("non_existent.segy");
        assert!(result.is_err());
    }
}
