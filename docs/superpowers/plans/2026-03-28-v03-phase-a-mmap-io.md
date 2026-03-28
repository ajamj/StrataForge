# StrataForge v0.3 Phase A: High-Performance Seismic I/O Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transition from in-memory seismic data to high-performance memory-mapped access for SEG-Y files, enabling near-instant random access to traces.

**Architecture:** Use `memmap2` to map SEG-Y files. Implement a `TraceIndex` that maps (Inline, Crossline) to file offsets, accounting for the 240-byte trace headers. Refactor `SeismicVolume` to support an abstraction for data access (In-memory vs Mmap) via a common trait.

**Tech Stack:** Rust, `memmap2`.

---

### Task 1: Dependency Setup

**Files:**
- Modify: `crates/sf_io/Cargo.toml`

- [ ] **Step 1: Add memmap2 to sf_io**

```toml
[dependencies]
memmap2 = "0.9"
# ... existing dependencies ...
```

- [ ] **Step 2: Commit**

```bash
git add crates/sf_io/Cargo.toml
git commit -m "chore: add memmap2 dependency to sf_io"
```

---

### Task 2: Implementing SEG-Y Header Parsing (Real)

**Files:**
- Modify: `crates/sf_io/src/segy/parser.rs`

- [ ] **Step 1: Implement basic header reading logic**
Read the 3200-byte EBCDIC header (with ASCII conversion) and 400-byte binary header to determine data format (IBM vs IEEE float) and volume dimensions.

- [ ] **Step 2: Add unit tests with a mock SEG-Y buffer**

- [ ] **Step 3: Commit**

```bash
git add crates/sf_io/src/segy/parser.rs
git commit -m "feat: implement basic SEG-Y header parsing"
```

---

### Task 3: Memory-Mapped Trace Access & Format Conversion

**Files:**
- Create: `crates/sf_io/src/segy/mmap.rs`
- Modify: `crates/sf_io/src/segy/mod.rs`

- [ ] **Step 1: Implement MmappedSegy struct with byte-swapping logic**

```rust
use memmap2::Mmap;
use std::fs::File;

pub struct MmappedSegy {
    mmap: Mmap,
    trace_count: usize,
    samples_per_trace: usize,
    // ... metadata ...
}

impl MmappedSegy {
    /// Read trace data at trace_index, performing Big-Endian to Little-Endian conversion.
    /// Note: Returns Vec<f32> as zero-copy ref is not possible due to endianness.
    pub fn read_trace_data(&self, trace_index: usize) -> Vec<f32> {
        // Offset = 3600 (headers) + (trace_index * (240 + samples_per_trace * 4)) + 240
        vec![] // placeholder
    }
}
```

- [ ] **Step 2: Implement trace indexing**
Scan trace headers to build a lookup table: `(inline, xline) -> trace_index`. Account for the 240-byte trace header offset per trace.

- [ ] **Step 3: Commit**

```bash
git add crates/sf_io/src/segy/mmap.rs crates/sf_io/src/segy/mod.rs
git commit -m "feat: implement memory-mapped SEG-Y access and indexing"
```

---

### Task 4: Refactoring SeismicVolume for Mmap Backend

**Files:**
- Modify: `crates/sf_compute/src/seismic.rs`

- [ ] **Step 1: Update SeismicVolume to use a TraceProvider trait**

```rust
pub trait TraceProvider {
    fn get_trace(&self, inline: i32, xline: i32) -> Option<Vec<f32>>;
    // ... dimensions ...
}

pub struct SeismicVolume {
    pub provider: Box<dyn TraceProvider + Send + Sync>,
    // ...
}
```

- [ ] **Step 2: Update slicing logic to work with the provider**
Note: `get_crossline` for the `Mmap` backend will be slower than `get_inline` due to non-contiguous reads.

- [ ] **Step 3: Verify with integration test using a sample file**

- [ ] **Step 4: Commit**

```bash
git add crates/sf_compute/src/seismic.rs
git commit -m "feat: refactor SeismicVolume to support provider-based mmap backend"
```
