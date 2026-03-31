//! Safe I/O utilities for Seisly
//!
//! This module provides safe wrappers around low-level I/O operations
//! to protect against crashes from file system failures, network disconnections,
//! and other I/O errors.

pub mod safe_mmap;

pub use safe_mmap::{SafeMmap, SafeMmapArc, SafeMmapExt};
