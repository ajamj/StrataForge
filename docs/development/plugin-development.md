# Plugin Development Guide

## Overview

The myfield plugin system allows developers to extend functionality through Rust plugins with Python bindings. This guide covers plugin architecture, development, and distribution.

## Table of Contents

1. [Architecture](#architecture)
2. [Getting Started](#getting-started)
3. [Creating a Plugin](#creating-a-plugin)
4. [Plugin API Reference](#plugin-api-reference)
5. [Python Bindings](#python-bindings)
6. [Testing](#testing)
7. [Distribution](#distribution)
8. [Best Practices](#best-practices)

## Architecture

### Plugin System Components

```
Plugin Architecture
├── PluginManager      - Lifecycle management
├── PluginRegistry     - Discovery and registration
├── PluginTrait        - Base trait for all plugins
├── CommandExecutor    - Command dispatch
└── Python Bindings    - PyO3 integration
```

### Plugin Lifecycle

```
Discovery → Load → Initialize → Execute → Shutdown
```

### Command Pattern

Plugins expose commands that can be invoked:

```rust
trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn commands(&self) -> Vec<&str>;
    fn execute(&self, command: &str, args: Value) -> Result<Value>;
}
```

## Getting Started

### Prerequisites

```bash
# Rust toolchain
rustup install stable

# Create plugin project
cargo new my_plugin --lib
cd my_plugin

# Add dependencies
cargo add seisly_plugins
cargo add serde
cargo add serde_json
cargo add thiserror
```

### Plugin Template

```rust
// src/lib.rs
use seisly_plugins::{Plugin, PluginInfo, PluginResult};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyPluginError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
}

pub struct MyPlugin {
    // Plugin state
}

impl MyPlugin {
    pub fn new() -> Self {
        Self {
            // Initialize state
        }
    }
}

impl Plugin for MyPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "my_plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "My custom plugin".to_string(),
            author: "Your Name".to_string(),
        }
    }

    fn commands(&self) -> Vec<&str> {
        vec!["process", "analyze", "transform"]
    }

    fn execute(&self, command: &str, args: Value) -> PluginResult {
        match command {
            "process" => self.process(args),
            "analyze" => self.analyze(args),
            "transform" => self.transform(args),
            _ => Err(MyPluginError::InvalidArgument(
                format!("Unknown command: {}", command)
            ).into()),
        }
    }
}

impl MyPlugin {
    fn process(&self, args: Value) -> PluginResult {
        // Implementation
        Ok(Value::Null)
    }

    fn analyze(&self, args: Value) -> PluginResult {
        // Implementation
        Ok(Value::Null)
    }

    fn transform(&self, args: Value) -> PluginResult {
        // Implementation
        Ok(Value::Null)
    }
}

// Plugin registration
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(MyPlugin::new()))
}

#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut dyn Plugin) {
    unsafe {
        let _ = Box::from_raw(plugin);
    }
}
```

## Creating a Plugin

### Example: Custom Attribute Plugin

```rust
// src/lib.rs
use seisly_plugins::{Plugin, PluginInfo, PluginResult};
use seisly_attributes::amplitude::RmsAmplitude;
use serde_json::{Value, json};
use serde::Deserialize;
use thiserror::Error;
use ndarray::Array1;

#[derive(Error, Debug)]
pub enum AttributePluginError {
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
    #[error("Attribute computation failed: {0}")]
    ComputationFailed(String),
}

pub struct AttributePlugin;

impl AttributePlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for AttributePlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "custom_attributes".to_string(),
            version: "0.1.0".to_string(),
            description: "Custom seismic attribute plugin".to_string(),
            author: "Your Name".to_string(),
        }
    }

    fn commands(&self) -> Vec<&str> {
        vec!["compute_rms", "compute_energy", "batch_compute"]
    }

    fn execute(&self, command: &str, args: Value) -> PluginResult {
        match command {
            "compute_rms" => self.compute_rms(args),
            "compute_energy" => self.compute_energy(args),
            "batch_compute" => self.batch_compute(args),
            _ => Err(AttributePluginError::InvalidArguments(
                format!("Unknown command: {}", command)
            ).into()),
        }
    }
}

#[derive(Deserialize)]
struct TraceArgs {
    trace: Vec<f32>,
    window_size: usize,
}

impl AttributePlugin {
    fn compute_rms(&self, args: Value) -> PluginResult {
        let args: TraceArgs = serde_json::from_value(args)
            .map_err(|e| AttributePluginError::InvalidArguments(e.to_string()))?;

        let trace = Array1::from(args.trace);
        let attr = RmsAmplitude;
        let result = attr.compute(&trace, args.window_size)
            .map_err(|e| AttributePluginError::ComputationFailed(e.to_string()))?;

        Ok(json!({"rms": result}))
    }

    fn compute_energy(&self, args: Value) -> PluginResult {
        let args: TraceArgs = serde_json::from_value(args)?;
        
        let trace = Array1::from(args.trace);
        let energy: f32 = trace.iter().map(|x| x * x).sum();

        Ok(json!({"energy": energy}))
    }

    fn batch_compute(&self, args: Value) -> PluginResult {
        // Batch processing implementation
        Ok(json!({"results": []}))
    }
}

// Plugin registration
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(AttributePlugin::new()))
}

#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut dyn Plugin) {
    unsafe {
        let _ = Box::from_raw(plugin);
    }
}
```

### Example: Data Processing Plugin

```rust
// src/lib.rs
use seisly_plugins::{Plugin, PluginInfo, PluginResult};
use serde_json::{Value, json};
use serde::Deserialize;

pub struct ProcessingPlugin {
    config: ProcessingConfig,
}

#[derive(Deserialize, Clone)]
struct ProcessingConfig {
    normalize: bool,
    filter_type: String,
}

impl ProcessingPlugin {
    pub fn new(config: ProcessingConfig) -> Self {
        Self { config }
    }
}

impl Plugin for ProcessingPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "data_processor".to_string(),
            version: "0.1.0".to_string(),
            description: "Seismic data processing plugin".to_string(),
            author: "Your Name".to_string(),
        }
    }

    fn commands(&self) -> Vec<&str> {
        vec!["normalize", "filter", "aggregate"]
    }

    fn execute(&self, command: &str, args: Value) -> PluginResult {
        match command {
            "normalize" => self.normalize(args),
            "filter" => self.filter(args),
            "aggregate" => self.aggregate(args),
            _ => Err("Unknown command".into()),
        }
    }
}

impl ProcessingPlugin {
    fn normalize(&self, args: Value) -> PluginResult {
        let data: Vec<f32> = serde_json::from_value(args)?;
        
        let max = data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let min = data.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let range = max - min;

        let normalized: Vec<f32> = data.iter()
            .map(|&x| (x - min) / range)
            .collect();

        Ok(json!({"normalized": normalized}))
    }

    fn filter(&self, args: Value) -> PluginResult {
        // Filter implementation
        Ok(json!({"filtered": []}))
    }

    fn aggregate(&self, args: Value) -> PluginResult {
        // Aggregation implementation
        Ok(json!({"aggregated": {}}))
    }
}

// Plugin registration
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(ProcessingPlugin::new(ProcessingConfig {
        normalize: true,
        filter_type: "gaussian".to_string(),
    })))
}

#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut dyn Plugin) {
    unsafe {
        let _ = Box::from_raw(plugin);
    }
}
```

## Plugin API Reference

### Plugin Trait

```rust
pub trait Plugin: Send + Sync {
    /// Returns plugin metadata
    fn info(&self) -> PluginInfo;

    /// Returns list of available commands
    fn commands(&self) -> Vec<&str>;

    /// Executes a command with arguments
    fn execute(&self, command: &str, args: Value) -> PluginResult;

    /// Called when plugin is loaded (optional)
    fn initialize(&mut self) -> PluginResult {
        Ok(Value::Null)
    }

    /// Called when plugin is unloaded (optional)
    fn shutdown(&mut self) -> PluginResult {
        Ok(Value::Null)
    }

    /// Plugin configuration (optional)
    fn configure(&mut self, config: Value) -> PluginResult {
        Ok(Value::Null)
    }
}
```

### PluginInfo Struct

```rust
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}
```

### PluginManager

```rust
use seisly_plugins::PluginManager;

// Create manager
let manager = PluginManager::new();

// Load plugin from file
manager.load_plugin("path/to/plugin.so")?;

// Load plugin from directory
manager.load_directory("plugins/")?;

// List plugins
let plugins = manager.list_plugins();
for plugin in plugins {
    println!("{} v{}", plugin.name, plugin.version);
}

// Get plugin info
let info = manager.get_plugin_info("my_plugin")?;

// Execute command
let result = manager.execute("my_plugin", "command", args)?;

// Unload plugin
manager.unload_plugin("my_plugin")?;
```

## Python Bindings

### Setting Up PyO3

```toml
# Cargo.toml
[lib]
name = "my_plugin"
crate-type = ["cdylib", "rlib"]

[dependencies]
pyo3 = { version = "0.20", features = ["extension-module"] }
serde_json = "1.0"
```

### Python Wrapper

```rust
// src/python.rs
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass]
pub struct PyPlugin {
    inner: MyPlugin,
}

#[pymethods]
impl PyPlugin {
    #[new]
    fn new() -> Self {
        Self {
            inner: MyPlugin::new(),
        }
    }

    fn info(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let dict = PyDict::new(py);
            dict.set_item("name", self.inner.info().name)?;
            dict.set_item("version", self.inner.info().version)?;
            dict.set_item("description", self.inner.info().description)?;
            Ok(dict.into())
        })
    }

    fn commands(&self) -> PyResult<Vec<&str>> {
        Ok(self.inner.commands())
    }

    fn execute(&self, command: &str, args: &PyDict) -> PyResult<PyObject> {
        // Convert PyDict to serde_json::Value
        let args_json = python_to_json(args)?;
        
        let result = self.inner.execute(command, args_json)?;
        
        // Convert result back to Python object
        json_to_python(&result)
    }
}

// Module definition
#[pymodule]
fn my_plugin(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPlugin>()?;
    Ok(())
}
```

### Python Usage

```python
from my_plugin import PyPlugin

# Create plugin instance
plugin = PyPlugin()

# Get info
info = plugin.info()
print(f"Plugin: {info['name']} v{info['version']}")

# List commands
commands = plugin.commands()
print(f"Commands: {commands}")

# Execute command
result = plugin.execute("compute_rms", {
    "trace": [1.0, 2.0, 3.0, 4.0, 5.0],
    "window_size": 3
})
print(f"RMS: {result['rms']}")
```

### Integration with seisly

```python
from seisly import PluginManager

# Initialize manager
manager = PluginManager()

# Load plugin
manager.load("my_plugin")

# List plugins
plugins = manager.list_plugins()
for name, info in plugins.items():
    print(f"{name}: {info['version']}")

# Execute
result = manager.execute("my_plugin", "compute_rms", {
    "trace": [1.0, 2.0, 3.0],
    "window_size": 3
})
```

## Testing

### Unit Tests

```rust
// src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_plugin_info() {
        let plugin = MyPlugin::new();
        let info = plugin.info();
        
        assert_eq!(info.name, "my_plugin");
        assert_eq!(info.version, "0.1.0");
    }

    #[test]
    fn test_commands() {
        let plugin = MyPlugin::new();
        let commands = plugin.commands();
        
        assert!(commands.contains(&"process"));
        assert!(commands.contains(&"analyze"));
    }

    #[test]
    fn test_execute_process() {
        let plugin = MyPlugin::new();
        let args = json!({"data": [1.0, 2.0, 3.0]});
        
        let result = plugin.execute("process", args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_unknown_command() {
        let plugin = MyPlugin::new();
        let args = json!({});
        
        let result = plugin.execute("unknown", args);
        assert!(result.is_err());
    }
}
```

### Integration Tests

```rust
// tests/integration.rs
use seisly_plugins::PluginManager;
use serde_json::json;

#[test]
fn test_plugin_loading() {
    let manager = PluginManager::new();
    
    manager.load_plugin("target/debug/my_plugin.so").unwrap();
    
    let plugins = manager.list_plugins();
    assert!(!plugins.is_empty());
}

#[test]
fn test_plugin_execution() {
    let manager = PluginManager::new();
    manager.load_plugin("target/debug/my_plugin.so").unwrap();
    
    let args = json!({"trace": [1.0, 2.0, 3.0], "window_size": 3});
    let result = manager.execute("my_plugin", "compute_rms", args).unwrap();
    
    assert!(result["rms"].as_f64().is_some());
}
```

### Python Tests

```python
# tests/test_plugin.py
import pytest
from my_plugin import PyPlugin

def test_plugin_info():
    plugin = PyPlugin()
    info = plugin.info()
    
    assert info["name"] == "my_plugin"
    assert info["version"] == "0.1.0"

def test_execute_rms():
    plugin = PyPlugin()
    result = plugin.execute("compute_rms", {
        "trace": [1.0, 2.0, 3.0, 4.0, 5.0],
        "window_size": 3
    })
    
    assert "rms" in result
    assert result["rms"] > 0
```

## Distribution

### Plugin Package Structure

```
my_plugin/
├── Cargo.toml
├── src/
│   └── lib.rs
├── tests/
│   └── integration.rs
├── python/
│   └── my_plugin/
│       └── __init__.py
├── README.md
└── plugin.json
```

### Plugin Manifest

```json
{
  "name": "my_plugin",
  "version": "0.1.0",
  "description": "My custom plugin",
  "author": "Your Name",
  "license": "MIT",
  "repository": "https://github.com/username/my_plugin",
  "platforms": ["linux", "windows", "macos"],
  "python_package": "my_plugin",
  "commands": [
    {
      "name": "compute_rms",
      "description": "Compute RMS amplitude",
      "parameters": {
        "trace": {"type": "array", "required": true},
        "window_size": {"type": "integer", "required": true}
      }
    }
  ]
}
```

### Publishing

```bash
# Build release
cargo build --release

# Package plugin
cargo plugin package

# Publish to registry
cargo plugin publish
```

### Installation

```bash
# From registry
sf-plugins install my_plugin

# From file
sf-plugins install ./my_plugin-0.1.0.tar.gz

# From URL
sf-plugins install https://github.com/username/my_plugin/releases/download/v0.1.0/my_plugin.tar.gz
```

## Best Practices

### Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
```

### Logging

```rust
use log::{info, warn, error};

fn process(&self, args: Value) -> PluginResult {
    info!("Processing started");
    
    // Implementation
    
    if something_wrong {
        warn!("Potential issue detected");
    }
    
    info!("Processing completed");
    Ok(result)
}
```

### Performance

```rust
use rayon::prelude::*;

fn batch_process(&self, args: Value) -> PluginResult {
    let data: Vec<f32> = serde_json::from_value(args)?;
    
    // Parallel processing
    let results: Vec<f32> = data.par_iter()
        .map(|x| x * 2.0)
        .collect();
    
    Ok(json!({"results": results}))
}
```

### Documentation

```rust
/// Computes RMS amplitude for a seismic trace.
///
/// # Arguments
/// * `trace` - Seismic trace data
/// * `window_size` - Window size in samples
///
/// # Returns
/// RMS amplitude value
///
/// # Example
/// ```
/// let plugin = MyPlugin::new();
/// let result = plugin.compute_rms(trace, 50);
/// ```
pub fn compute_rms(&self, trace: &[f32], window_size: usize) -> f32 {
    // Implementation
}
```

### Version Compatibility

```rust
use seisly_plugins::PluginApiVersion;

impl Plugin for MyPlugin {
    fn api_version(&self) -> PluginApiVersion {
        PluginApiVersion::V1
    }
}
```

## Troubleshooting

### Plugin Not Loading

**Check:**
1. Plugin compiled for correct platform
2. Dependencies are available
3. Plugin implements required traits

```bash
# Check plugin exports
nm my_plugin.so | grep create_plugin
```

### Command Execution Fails

**Check:**
1. Command name is correct
2. Arguments match expected format
3. Error handling is proper

```rust
// Debug execution
fn execute(&self, command: &str, args: Value) -> PluginResult {
    eprintln!("Executing: {} with args: {:?}", command, args);
    // ...
}
```

### Python Bindings Fail

**Check:**
1. PyO3 features enabled
2. Python version matches
3. Module initialization correct

```bash
# Test Python import
python -c "import my_plugin"
```

## Examples

See the `examples/` directory for complete plugin examples:
- `examples/attribute_plugin/` - Custom attribute plugin
- `examples/processor_plugin/` - Data processing plugin
- `examples/ml_plugin/` - ML model plugin

## Resources

- [Plugin API Documentation](https://docs.myfield.dev/plugins)
- [PyO3 Guide](https://pyo3.rs/)
- [Plugin Examples](https://github.com/myfield/plugin-examples)
- [Plugin Registry](https://plugins.myfield.dev/)
