use crate::api::{Plugin, PluginCommand, PluginError, Result};
use crate::manifest::PluginManifest;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;
use std::path::PathBuf;

/// A plugin implemented in Python
pub struct PythonPlugin {
    manifest: PluginManifest,
    path: PathBuf,
}

impl PythonPlugin {
    pub fn new(manifest: PluginManifest, path: PathBuf) -> Self {
        Self { manifest, path }
    }
}

impl Plugin for PythonPlugin {
    fn name(&self) -> &str {
        &self.manifest.name
    }

    fn version(&self) -> &str {
        &self.manifest.version
    }

    fn description(&self) -> &str {
        self.manifest.description.as_deref().unwrap_or("No description")
    }

    fn commands(&self) -> Vec<PluginCommand> {
        vec![PluginCommand {
            name: "run".to_string(),
            description: "Run the Python plugin".to_string(),
        }]
    }

    fn execute(&self, _cmd: &str, _args: Value) -> Result<Value> {
        Python::with_gil(|py| -> PyResult<Value> {
            let sys = py.import_bound("sys")?;
            let path = sys.getattr("path")?;
            
            let plugin_dir = self.path.parent().ok_or_else(|| {
                pyo3::exceptions::PyRuntimeError::new_err("Invalid plugin path")
            })?;
            
            path.call_method1("append", (plugin_dir.to_str().unwrap(),))?;

            // Import the entry point module
            let module_name = self.manifest.entry_point.trim_end_matches(".py");
            let module = py.import_bound(module_name)?;
            
            // Call the 'execute' function in the module
            let execute_fn = module.getattr("execute")?;
            // Pass args as a Python dict (placeholder for now)
            let py_args = PyDict::new_bound(py);
            let _result = execute_fn.call1((py_args,))?;
            
            Ok(Value::String("Python execution successful".to_string()))
        }).map_err(|e| PluginError::ExecutionError(e.to_string()))
    }
}
