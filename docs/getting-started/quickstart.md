# Seisly - Quick Start Guide

**Version:** 0.1.0  
**Last Updated:** 2026-03-28

---

## 🚀 What is Seisly?

Seisly is a modern, open-source 3D seismic interpretation application designed for geoscientists in oil & gas exploration.

### **Key Features:**
- ✅ Seismic volume visualization (inline, crossline, time slice)
- ✅ Horizon interpretation (manual picking, auto-tracking)
- ✅ Fault interpretation (sketch mode, RBF surface modeling)
- ✅ 3D visualization with transparency
- ✅ Velocity modeling & time-depth conversion
- ✅ Synthetic data generation for testing
- ✅ Light/Dark theme support
- ✅ Well data integration (LAS import coming soon)

---

## 📥 Installation

### **Prerequisites:**
- Rust toolchain (install from https://rustup.rs)
- Git
- Minimum 4GB RAM, 8GB recommended

### **Build from Source:**

```bash
# Clone repository
git clone https://github.com/ajamj/seisly.git
cd seisly

# Build release version
cargo build --release

# Run application
cargo run --release --bin sf-app
```

---

## 🎯 Quick Start Tutorial

### **Step 1: Launch Application**

```bash
cargo run --release --bin sf-app
```

Application window will open with:
- **Top Ribbon:** Tools and menu
- **Left Panel:** Project data tree
- **Center:** 3D Viewport
- **Right Panel:** Properties and analysis
- **Bottom Panel:** Well logs (collapsible)
- **Status Bar:** Coordinates and status

### **Step 2: Generate Synthetic Data (Optional)**

For testing without real data, generate synthetic data:

```rust
// In Rust REPL or test script
use seisly_compute::synthetic::*;

// Generate synthetic seismic
let seismic = SyntheticSeismic::new(500, 500, 512);
let data = seismic.generate();

// Generate synthetic well logs
let well = SyntheticWellLog::new("Demo Well", 500000.0, 1000000.0, 50.0, 3000.0);
let (_depths, gr) = well.generate_gr();
```

### **Step 3: Interpret Horizon**

1. **Select Picking Mode:**
   - Click toolbar: `Seed` | `Manual` | `Auto` | `Sketch`

2. **Pick Horizon Points:**
   - Click on seismic volume to add picks
   - Horizon mesh will auto-generate

3. **View Properties:**
   - Select horizon in left panel
   - Edit color, visibility in right panel

### **Step 4: Sketch Fault**

1. **Enable Sketch Mode:**
   - Click `Sketch Fault` in toolbar

2. **Draw Fault Stick:**
   - Click-drag in viewport to draw
   - Fault surface auto-models with RBF

3. **Adjust Properties:**
   - Select fault in left panel
   - Change color, transparency

### **Step 5: Velocity & Depth Conversion**

1. **Open Velocity Panel:**
   - Right panel → "Velocity Model"

2. **Set Parameters:**
   - V0: 2000 m/s (default)
   - k: 0.5 1/s (default)

3. **Enable Depth Mode:**
   - Toggle "Depth" checkbox
   - Viewport displays depth domain

---

## 🎨 UI Guide

### **Theme Toggle:**
- Click ☀ (sun) for Light theme
- Click ☾ (moon) for Dark theme

### **Viewport Navigation:**
- **Rotate:** Right-click + drag
- **Pan:** Middle-click + drag
- **Zoom:** Scroll wheel
- **Reset View:** Double-click right mouse

### **Picking Modes:**

| Mode | Icon | Shortcut | Description |
|------|------|----------|-------------|
| None | ⊘ | N | No picking |
| Seed | 🌱 | S | Auto-seed picking |
| Manual | ✏️ | M | Manual point picking |
| Auto | 🔄 | A | Auto-tracking |
| Sketch | ⚡ | K | Sketch fault mode |

---

## 📊 Data Formats

### **Supported Import:**
- **Seismic:** In-memory provider (SEG-Y coming soon)
- **Wells:** LAS 2.0 (implementation in progress)

### **Supported Export:**
- **Horizon Picks:** XYZ, JSON
- **Mesh Surfaces:** XYZ

### **Export Horizon:**
1. Select horizon in left panel
2. Right panel → "Export XYZ" or "Export JSON"
3. File saved to project directory

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+S` | Save project |
| `Ctrl+O` | Open project |
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |
| `N` | None picking mode |
| `S` | Seed picking mode |
| `M` | Manual picking mode |
| `A` | Auto-tracking mode |
| `K` | Sketch fault mode |
| `D` | Toggle depth mode |
| `F1` | Show help |

---

## 🐛 Troubleshooting

### **Application Won't Start:**
```bash
# Clean build
cargo clean
cargo build --release

# Check Rust version
rustc --version  # Should be 1.70+
```

### **Performance Issues:**
- Reduce seismic volume size
- Disable unused horizons/faults
- Use lower resolution mesh

### **Import Errors:**
- Verify LAS file format (2.0 supported)
- Check file encoding (ASCII required)
- Ensure no BOM markers

---

## 📚 Additional Resources

- **Design Spec:** `docs/superpowers/specs/2026-03-28-well-integration-design.md`
- **Implementation Plan:** `docs/superpowers/plans/2026-03-28-well-integration.md`
- **UI Redesign Spec:** `docs/ui-redesign-spec.md`
- **Synthetic Data Docs:** `crates/seisly_compute/src/synthetic.rs` (module docs)

---

## 🤝 Contributing

Contributions welcome! Please:
1. Fork repository
2. Create feature branch
3. Run tests: `cargo test --workspace`
4. Submit PR

---

## 📞 Support

- **Issues:** GitHub Issues
- **Discussions:** GitHub Discussions
- **Email:** support@Seisly.example.com

---

**Happy Interpreting! 🛢️🌍**
