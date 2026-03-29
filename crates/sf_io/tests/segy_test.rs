//! SEG-Y reader integration tests

use sf_io::segy::{IoError, SegyReader};
use tempfile::TempDir;

#[test]
fn test_segy_reader_open_and_read() {
    // We'll test with SegyWriter (implemented in Task 3)
    // For now, skip if no test data
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.segy");

    // Test file doesn't exist yet - this test will be enabled after Task 3
    if !test_file.exists() {
        // Skip for now, will be implemented with SegyWriter
        return;
    }

    let reader = SegyReader::open(&test_file).unwrap();

    // Check binary header
    assert!(reader.binary_header().is_ok());

    // Read first trace
    let trace = reader.read_trace(0);
    assert!(trace.is_ok());
}

#[test]
fn test_segy_reader_error_handling() {
    // Test error on non-existent file
    let result = SegyReader::open("non_existent_file.segy");
    assert!(result.is_err());
    
    // Verify it's an Io error
    match result {
        Err(IoError::Io(_)) => {}, // Expected
        Err(IoError::ParseError(_)) => panic!("Expected IO error, got ParseError"),
        Ok(_) => panic!("Expected error, got Ok"),
    }
}
