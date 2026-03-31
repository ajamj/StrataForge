# Plugin-Based Python AI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a high-performance, user-extensible Python Deep Learning engine embedded in the Rust Seisly application using PyO3 for zero-copy data sharing.

**Architecture:** Embedded Python interpreter in a new `seisly_plugin` manager, shared memory pointers for seismic volumes, and dynamic UI panels in `egui`.

**Tech Stack:** Rust, PyO3, NumPy, Apache Arrow (for data layout), egui, SQLite.

---

### Task 1: PyO3 Embedded Interpreter Initialization

**Files:**
- Create: `crates/seisly_plugin/src/interpreter.rs`
- Modify: `crates/seisly_plugin/src/lib.rs`
- Test: `crates/seisly_plugin/src/tests/interpreter_tests.rs`

- [ ] **Step 1: Write the failing test**
Verify that the Python interpreter can be initialized and can execute a simple string.

```rust
#[test]
fn test_python_init() {
    let interpreter = PythonInterpreter::new().unwrap();
    let result: String = interpreter.run_string("import sys; sys.version").unwrap();
    assert!(result.contains("3."));
}
```

- [ ] **Step 2: Run test to verify it fails**
Run: `cargo test -p seisly_plugin`
Expected: FAIL (types not defined)

- [ ] **Step 3: Implement `PythonInterpreter` struct**
Use `pyo3::prepare_freethreaded_python()` to initialize the runtime.

```rust
pub struct PythonInterpreter;

impl PythonInterpreter {
    pub fn new() -> Result<Self, PluginError> {
        pyo3::prepare_freethreaded_python();
        Ok(Self)
    }

    pub fn run_string(&self, code: &str) -> Result<String, PluginError> {
        Python::with_gil(|py| {
            let sys = py.import("sys")?;
            let version: String = sys.getattr("version")?.extract()?;
            Ok(version)
        })
    }
}
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test -p seisly_plugin`
Expected: PASS

- [ ] **Step 5: Commit**
```bash
git add crates/seisly_plugin/
git commit -m "feat(plugin): initialize embedded python interpreter"
```

---

### Task 2: Plugin Discovery and Manifest Parsing

**Files:**
- Create: `crates/seisly_plugin/src/manifest.rs`
- Test: `crates/seisly_plugin/src/tests/manifest_tests.rs`

- [ ] **Step 1: Define `PluginManifest` struct**
Support name, version, and type (Fault/Horizon).

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub plugin_type: String, // "fault" | "horizon"
    pub entry_point: String, // "main.py"
}
```

- [ ] **Step 2: Implement discovery logic**
Scan `plugins/` directory for subdirectories containing `manifest.yaml`.

- [ ] **Step 3: Write tests for discovery**
Create a temporary directory with mock manifests and verify they are correctly listed.

- [ ] **Step 4: Commit**
```bash
git add crates/seisly_plugin/
git commit -m "feat(plugin): implement plugin discovery and manifest parsing"
```

---

### Task 3: Zero-Copy Shared Memory Bridge

**Files:**
- Create: `crates/seisly_plugin/src/bridge.rs`
- Test: `crates/seisly_plugin/src/tests/bridge_tests.rs`

- [ ] **Step 1: Implement `SeismicView` for Python**
Convert a Rust slice `&[f32]` into a NumPy array using `pyo3-numpy`.

```rust
pub fn share_with_python(py: Python, data: &[f32], shape: Vec<usize>) -> PyResult<&PyArrayDyn<f32>> {
    let array = unsafe {
        PyArrayDyn::borrow_from_array(py, data, shape)
    };
    Ok(array)
}
```

- [ ] **Step 2: Verify zero-copy behavior**
Write a test that modifies the array in Python and checks if the Rust slice reflects the change (or vice versa).

- [ ] **Step 3: Commit**
```bash
git add crates/seisly_plugin/
git commit -m "feat(plugin): implement zero-copy memory bridge to numpy"
```

---

### Task 4: Plugin Manager UI (egui)

**Files:**
- Modify: `crates/seisly_app/src/windows/plugin_manager.rs`
- Modify: `crates/seisly_app/src/app.rs`

- [ ] **Step 1: Create Plugin List UI**
Display a table of discovered plugins with "Load" and "Unload" buttons.

- [ ] **Step 2: Implement Dynamic Parameter Panel**
Read a `schema` from the Python plugin and render egui sliders/inputs.

- [ ] **Step 3: Commit**
```bash
git add crates/seisly_app/
git commit -m "feat(ui): add plugin manager window and dynamic panels"
```

---

### Task 5: End-to-End Integration Test

- [ ] **Step 1: Create `MockHorizonTracker` Python plugin**
A simple Python script that "detects" a flat horizon at Z=500.

- [ ] **Step 2: Run full integration test**
Open app -> Load Mock Plugin -> Click Track -> Verify Picks in SQLite.

- [ ] **Step 3: Commit**
```bash
git add plugins/mock_plugin/
git commit -m "test(e2e): verify full plugin-to-interpretation workflow"
```
