#!/usr/bin/env python3
"""Basic usage example for Seisly Python bindings"""

from seisly import PluginManager

def main():
    print("Seisly Python Example")
    print("=" * 40)
    
    # Create plugin manager
    manager = PluginManager()
    
    # List available plugins
    plugins = manager.list_plugins()
    print(f"Available plugins: {plugins}")
    print(f"Plugin count: {manager.plugin_count()}")
    
    # Note: To use plugins, you need to register them first
    # Example with test plugin would go here
    
    print("\nExample completed successfully!")

if __name__ == "__main__":
    main()
