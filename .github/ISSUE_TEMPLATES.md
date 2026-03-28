# StrataForge GitHub Issues

Collection of GitHub issues for StrataForge project development tracking.

---

## 🚀 **Issue #1: Setup GitHub Repository & CI/CD**

**Labels:** `infrastructure`, `ci/cd`, `priority:critical`, `good first issue`  
**Assignee:** Unassigned  
**Milestone:** v0.1.1 Beta

### Description
Setup GitHub repository dengan complete CI/CD pipeline untuk automated testing, releases, dan deployment.

### Tasks
- [ ] Create GitHub repository (public)
- [ ] Configure repository settings
  - Enable Issues
  - Enable Projects
  - Enable Discussions
  - Enable Wiki
- [ ] Add repository topics: `rust`, `geoscience`, `seismic`, `open-source`
- [ ] Enable GitHub Actions
- [ ] Verify CI/CD workflows running
- [ ] Setup Codecov integration
- [ ] Configure crates.io publishing tokens
- [ ] Add repository to GitHub Projects board

### Acceptance Criteria
- ✅ Repository accessible at https://github.com/strataforge/strataforge
- ✅ CI pipeline runs successfully on push
- ✅ Tests pass on Windows, Linux, macOS
- ✅ Coverage report visible on Codecov
- ✅ Release workflow functional

### Technical Notes
- CI/CD workflows sudah ada di `.github/workflows/`
- Codecov token perlu ditambahkan ke repository secrets
- crates.io token untuk publishing workspace crates

### Resources
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Codecov Setup Guide](https://docs.codecov.com/docs)
- [crates.io Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)

---

## 🐛 **Issue #2: Complete Project Save/Load UI**

**Labels:** `feature`, `ui`, `priority:high`  
**Assignee:** Unassigned  
**Milestone:** v0.1.1 Beta

### Description
Implement complete project save/load functionality dengan file dialogs dan menu integration untuk enable users save and load their interpretation work.

### Background
Project management foundation sudah implemented di `crates/sf_app/src/project.rs` dengan:
- ProjectData serialization structure
- ProjectManager save/load methods
- Snapshot types untuk interpretation state

Yang masih missing adalah UI integration untuk trigger save/load operations.

### Tasks
- [ ] Add file dialog dependency
  ```toml
  [dependencies]
  rfd = "0.14"  # Native file dialogs
  ```
- [ ] Create "New Project" dialog
  - Project name input
  - Location picker
  - CRS selection (optional)
- [ ] Implement "Open Project" file picker
  - Filter `.sfp` files
  - Recent projects list
  - Validate project format
- [ ] Add "Save Project" functionality
  - Auto-save option
  - Save confirmation dialog
  - Error handling
- [ ] Create "Recent Projects" submenu
  - Store last 5-10 projects
  - Quick access from File menu
  - Remove invalid entries
- [ ] Add menu items to File menu
  - New Project (Ctrl+N)
  - Open Project (Ctrl+O)
  - Save Project (Ctrl+S)
  - Save As (Ctrl+Shift+S)
  - Recent Projects (submenu)
- [ ] Test end-to-end workflow
  - Create new project
  - Add interpretation data
  - Save project
  - Close and reopen
  - Verify data integrity

### Technical Implementation

**File Dialog Integration:**
```rust
use rfd::FileDialog;

// Save Project
if let Some(path) = FileDialog::new()
    .add_filter("StrataForge Project", &["sfp"])
    .set_file_name("project.sfp")
    .save_file()
{
    let project = ProjectData::from_state(
        &project_name,
        &self.interpretation,
        &self.wells,
    );
    ProjectManager::save(&project, &path)?;
}

// Open Project
if let Some(path) = FileDialog::new()
    .add_filter("StrataForge Project", &["sfp"])
    .pick_file()
{
    let project = ProjectManager::load(&path)?;
    // Restore state from project
}
```

### Acceptance Criteria
- ✅ Users dapat create new project dengan name dan location
- ✅ Users dapat open existing `.sfp` files
- ✅ Users dapat save current work
- ✅ Recent projects accessible dari menu
- ✅ Error handling untuk invalid files
- ✅ Data persistence verified

### Dependencies
- `rfd` crate for native file dialogs
- Existing `ProjectManager` implementation

---

## 📊 **Issue #3: Add Code Coverage Badge to README**

**Labels:** `documentation`, `ci/cd`, `good first issue`  
**Assignee:** Unassigned  
**Milestone:** v0.1.1 Beta

### Description
Add Codecov coverage badge dan status indicators ke README.md setelah CI/CD setup complete.

### Tasks
- [ ] Setup Codecov integration di GitHub Actions
- [ ] Get Codecov token
- [ ] Add token to repository secrets
- [ ] Verify coverage reports uploading
- [ ] Add badges to README.md:
  ```markdown
  [![Coverage Status](https://codecov.io/gh/strataforge/strataforge/branch/master/graph/badge.svg)](https://codecov.io/gh/strataforge/strataforge)
  [![CI/CD](https://github.com/strataforge/strataforge/actions/workflows/ci-cd.yml/badge.svg)](https://github.com/strataforge/strataforge/actions/workflows/ci-cd.yml)
  ```
- [ ] Test badge rendering

### Acceptance Criteria
- ✅ Coverage badge visible di README
- ✅ Badge links to Codecov dashboard
- ✅ Coverage data accurate
- ✅ CI/CD badge shows current status

---

## 🔧 **Issue #4: Implement Error Dialog System**

**Labels:** `feature`, `ui`, `error-handling`, `priority:medium`  
**Assignee:** Unassigned  
**Milestone:** v0.1.1 Beta

### Description
Create user-friendly error dialog system untuk replace console error messages dengan informative popups.

### Background
Currently, errors ditampilkan di console atau status bar. Users perlu better feedback ketika terjadi errors.

### Tasks
- [ ] Create ErrorDialog widget
  - Error title
  - Error message (support multiline)
  - Error details (collapsible)
  - Action buttons (OK, Retry, Cancel)
  - Icon based on severity
- [ ] Add error types classification
  - `ErrorSeverity::Warning` (yellow)
  - `ErrorSeverity::Error` (red)
  - `ErrorSeverity::Critical` (red + urgent)
- [ ] Integrate dengan existing error handling
  - File I/O errors
  - Parse errors
  - Validation errors
- [ ] Add error logging
  - Write to log file
  - Include timestamp
  - Stack trace for critical errors
- [ ] Create error codes system
  - `ERR-001`: File not found
  - `ERR-002`: Invalid format
  - `ERR-003`: Permission denied
  - etc.

### Design Mockup
```
┌─────────────────────────────────────┐
│  ⚠️  Error Loading Project           │
├─────────────────────────────────────┤
│                                     │
│  The project file could not be      │
│  loaded.                            │
│                                     │
│  ▼ Details                          │
│  Invalid JSON at line 42:           │
│  expected value at line 1 column 2  │
│                                     │
│  Error Code: ERR-002                │
│  Log: strataforge_2026-03-28.log   │
│                                     │
│         [Retry]      [OK] [Cancel]  │
└─────────────────────────────────────┘
```

### Acceptance Criteria
- ✅ Error dialogs muncul untuk common errors
- ✅ Clear, actionable error messages
- ✅ Error codes untuk troubleshooting
- ✅ Log file created untuk debugging
- ✅ Consistent styling across dialogs

---

## 🧪 **Issue #5: Increase Test Coverage to 80%**

**Labels:** `testing`, `quality`, `priority:medium`  
**Assignee:** Unassigned  
**Milestone:** v0.2.0

### Description
Increase code coverage dari current ~60% ke target 80% untuk ensure code quality and reliability.

### Current Status
```
Test Results: 59 passing
Coverage: ~60% (estimated)
Target: 80%
```

### Tasks
- [ ] Identify low-coverage modules
  - Run `cargo tarpaulin` untuk detailed report
  - Create coverage heatmap
- [ ] Write tests for sf_app widgets
  - FaultPropertiesPanel
  - HorizonPropertiesPanel
  - VelocityPanel
  - WellPanel
- [ ] Add integration tests
  - Project save/load workflow
  - LAS import/export workflow
  - Interpretation workflow
- [ ] Test error paths
  - Invalid file formats
  - Missing files
  - Permission errors
- [ ] Add property-based tests
  - Use `proptest` crate
  - Test edge cases
- [ ] Setup coverage threshold in CI
  - Fail PR jika coverage turun
  - Report coverage in PR comments

### Modules Priority
1. **Critical** (must test):
   - `sf_io::las::parser` - LAS parsing
   - `sf_io::las::writer` - LAS export
   - `sf_compute::synthetic` - Data generation
   - `sf_app::project` - Project management

2. **Important** (should test):
   - `sf_render::fault_renderer` - 3D rendering
   - `sf_compute::interpolation` - RBF modeling
   - `sf_app::widgets::viewport` - Main viewport

3. **Nice to Have** (optional):
   - UI styling code
   - Debug utilities
   - Example code

### Acceptance Criteria
- ✅ Overall coverage ≥ 80%
- ✅ All critical modules ≥ 90%
- ✅ No module < 50% coverage
- ✅ Coverage trend increasing
- ✅ CI fails jika coverage turun > 5%

### Tools
```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --workspace --out Html --output-dir coverage

# View report
open coverage/tarpaulin-report.html
```

---

## 🎨 **Issue #6: Implement Well Log Visualization**

**Labels:** `feature`, `ui`, `wells`, `priority:high`  
**Assignee:** Unassigned  
**Milestone:** v0.2.0

### Description
Create floating log viewer window untuk visualize well logs dengan professional display dan interactive controls.

### Background
Well data sudah bisa di-import dari LAS files, tapi belum ada UI untuk visualize log curves.

### Tasks
- [ ] Create LogViewerWindow widget
  - Draggable title bar
  - Resizable window
  - Close button
  - Stay-on-top option
- [ ] Implement curve display
  - Depth track (leftmost)
  - Multiple curve tracks (side-by-side)
  - Configurable curves per track
  - Curve styling:
    - Color picker
    - Line width slider
    - Fill options (left/right/none)
    - Scale (min/max)
- [ ] Add navigation controls
  - Zoom in/out (mouse wheel)
  - Pan (drag)
  - Fit to depth
  - Track lock (sync depth across wells)
- [ ] Implement curve statistics
  - Min/Max/Average
  - Standard deviation
  - Histogram view
- [ ] Add export functionality
  - Export to image (PNG, PDF)
  - Export to CSV
  - Copy to clipboard

### Design Mockup
```
┌─ Well-1 - Log Viewer ────────────────────────┐
│  Well-1                          [_][□][X]   │
├──────────────────────────────────────────────┤
│ Depth │   GR    │   DT    │  RHOB            │
│   (m) │  (GAPI) │ (us/m)  │ (g/cm³)          │
├───────┼─────────┼─────────┼──────────────────┤
│   0   │───┐    │         │                  │
│  500  │   │    │   ┌─────│──────┐           │
│ 1000  │───┘    │   │     │      │           │
│ 1500  │        │───┘     │──────┘           │
│ 2000  │        │         │                  │
│ 2500  │        │         │                  │
├───────┴─────────┴─────────┴──────────────────┤
│ Zoom: [━━━━━●━━━━━]  Fit  │  Export: [PNG]  │
└──────────────────────────────────────────────┘
```

### Acceptance Criteria
- ✅ Logs displayed dengan proper scaling
- ✅ Multiple curves visible simultaneously
- ✅ Users dapat customize curve appearance
- ✅ Zoom/pan smooth performance
- ✅ Export to image functional

### Dependencies
- `plotters` crate for curve rendering
- Well data dari LAS import

---

## 📝 **Issue #7: Create User Documentation Site**

**Labels:** `documentation`, `website`, `priority:low`  
**Assignee:** Unassigned  
**Milestone:** v0.2.0

### Description
Build comprehensive user documentation website dengan mdBook atau similar tool.

### Tasks
- [ ] Choose documentation platform
  - mdBook (Rust-native)
  - Docusaurus (React-based)
  - GitBook (hosted)
- [ ] Setup documentation structure
  - Getting Started
  - User Guide
  - API Reference
  - FAQ
  - Troubleshooting
- [ ] Write core content
  - Installation guide
  - Quick start tutorial
  - Feature walkthrough
  - Best practices
- [ ] Add screenshots dan diagrams
- [ ] Setup automated deployment
  - GitHub Pages
  - Netlify
  - Vercel
- [ ] Add search functionality
- [ ] Setup analytics

### Acceptance Criteria
- ✅ Documentation site live
- ✅ All major features documented
- ✅ Search functional
- ✅ Mobile-friendly
- ✅ Auto-deploy on merge to master

---

## 🎯 **Milestone: v0.1.1 Beta Release**

**Due Date:** April 4, 2026  
**Status:** In Progress (80% complete)

### Description
First public beta release dengan core interpretation features dan basic well integration.

### Features Included
- ✅ Seismic visualization
- ✅ Horizon interpretation
- ✅ Fault interpretation
- ✅ Velocity modeling
- ✅ Synthetic data generation
- ✅ LAS 2.0 import/export
- ✅ Modern UI dengan themes
- ⏳ Project save/load (in progress)
- ⏳ Error dialogs (planned)

### Release Checklist
- [ ] All critical bugs fixed
- [ ] Documentation complete
- [ ] CI/CD pipeline passing
- [ ] Release notes written
- [ ] Git tag created
- [ ] GitHub release published
- [ ] Binaries uploaded for all platforms
- [ ] Announcement blog post

### Known Issues
- Project save/load UI incomplete
- Error dialogs not implemented
- Well log viewer not included
- Some performance optimizations pending

---

## 📊 **How to Use These Issues**

1. **Copy issue templates** di atas
2. **Create di GitHub** (setelah repository setup)
3. **Assign labels dan milestones**
4. **Add to Project Board** untuk tracking
5. **Link PRs** ke issues terkait

**Total Issues Created:** 7 issues + 1 milestone  
**Priority Distribution:**
- 🔴 Critical: 1
- 🟠 High: 3
- 🟡 Medium: 2
- 🟢 Low: 1

Semua issues ready untuk development tracking! 🚀
