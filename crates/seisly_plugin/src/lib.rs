//! Seisly Plugin System
//!
//! Extensible plugin architecture for custom workflows.

pub mod api;
pub mod manager;
pub mod manifest;

#[cfg(feature = "python")]
pub mod python;
#[cfg(feature = "python")]
pub mod interpreter;
#[cfg(feature = "python")]
pub mod bridge;
#[cfg(feature = "python")]
pub mod python_plugin;
#[cfg(feature = "python")]
pub mod ipc;

pub use api::{Plugin, PluginContext, PluginCommand, PluginError, Result};
pub use manager::PluginManager;
pub use manifest::PluginManifest;
