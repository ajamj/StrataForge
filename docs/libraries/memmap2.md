# memmap2

`memmap2` provides cross-platform memory-mapped file I/O for StrataForge. It is used to handle large seismic data files that are too big to load into memory.

## Overview

In StrataForge, we use `memmap2` to map SEG-Y files directly into the virtual memory of our process. This allows for extremely fast, random-access to seismic traces without the overhead of explicit file seeks or repeated `read()` calls.

### Why memmap2?
- **Speed:** Offloads I/O management to the operating system's kernel, which uses advanced caching and paging algorithms.
- **Large Files:** Allows us to work with files far larger than our physical RAM.
- **Random Access:** Simplifies the logic for fetching individual seismic traces from across the file.

## Usage in Project

The `sf_io` crate uses `memmap2` for high-performance seismic data handling.

### Memory Mapping a SEG-Y File
Located in `crates/sf_io/src/segy/mmap.rs`:

```rust
use memmap2::Mmap;
use std::fs::File;
use anyhow::Result;

pub struct MmappedSegy {
    mmap: Mmap,
    // ... metadata fields
}

impl MmappedSegy {
    pub fn new(path: &Path) -> Result<Self> {
        let file = File::open(path)?;
        // Create the memory map
        let mmap = unsafe { Mmap::map(&file)? };

        // Access the bytes directly as if they were in a byte slice
        let sample_interval = u16::from_be_bytes([mmap[3216], mmap[3217]]);
        
        Ok(Self { mmap, ... })
    }

    pub fn read_trace_data(&self, offset: usize, count: usize) -> Vec<f32> {
        // Use slice indexing directly on the map
        let bytes = &self.mmap[offset..offset + (count * 4)];
        // ... process bytes to f32 values
    }
}
```

## Key Concepts

- **Memory Mapping:** A technique where a file is mapped to a portion of the process's address space. Accessing that memory triggers the OS to load the corresponding part of the file from disk (on-demand).
- **Mmap:** The primary struct representing a read-only memory map of a file.
- **MmapMut:** A mutable memory map, useful for writing directly back to the file (not currently used for SEG-Y input).
- **Unsafe Code:** Creating a memory map is `unsafe` because the file on disk could be modified or truncated by another process, potentially leading to memory unsafety. In StrataForge, we use this carefully for performance-critical seismic I/O.

## Resources

- [memmap2 Documentation (docs.rs)](https://docs.rs/memmap2)
- [Wikipedia: Memory-mapped file](https://en.wikipedia.org/wiki/Memory-mapped_file)
