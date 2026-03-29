//! StrataForge Plugin System
//!
//! Extensible plugin architecture for custom workflows.

pub mod api;
pub mod manager;

pub use api::{Plugin, PluginContext, PluginCommand, PluginError, Result};
pub use manager::PluginManager;
