# Seisly Rebranding Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Completely rebrand the project from "Seisly" to "Seisly", including all code, configuration, documentation, and UI elements.

**Architecture:** Systematic global search and replace using case-sensitive patterns, followed by targeted updates to specific components like UI titles and Python packages.

**Tech Stack:** Rust, Python, Proto, Markdown, YAML, SQL.

---

### Task 1: Global Search and Replace (Part 1: Case-Sensitive Names)

**Files:**
- Modify: All files containing "Seisly" or "Seisly"

- [ ] **Step 1: Replace "Seisly" with "Seisly"**
- [ ] **Step 2: Replace "Seisly" with "seisly"**
- [ ] **Step 3: Commit**

```bash
git commit -m "rebrand: global replace Seisly with Seisly"
```

### Task 2: Global Search and Replace (Part 2: Prefixes and Acronyms)

**Files:**
- Modify: All files containing "seisly_" or "SF" (where SF means Seisly)

- [ ] **Step 1: Replace "seisly_" with "seisly_"**
- [ ] **Step 2: Replace "SF" with "SEISLY" (carefully, verify context)**
- [ ] **Step 3: Commit**

```bash
git commit -m "rebrand: update seisly_ prefix and SF acronym to seisly_"
```

### Task 3: Update UI and Application Logic

**Files:**
- Modify: `crates/seisly_app/src/main.rs`
- Modify: `crates/seisly_app/src/app.rs`
- Modify: `crates/seisly_app/src/project.rs`

- [ ] **Step 1: Update application titles and struct names in `main.rs` and `app.rs`**
- [ ] **Step 2: Update project file extension from `sfp` to `seisly` (or similar if requested, keeping `sfp` might be fine but `seisly` is better)**
- [ ] **Step 3: Commit**

```bash
git commit -m "rebrand: update UI titles and application structs"
```

### Task 4: Update Python Bindings and Examples

**Files:**
- Modify: `python/seisly/__init__.py`
- Modify: `python/examples/basic_usage.py`
- Modify: `pyproject.toml`

- [ ] **Step 1: Update python module name and references**
- [ ] **Step 2: Update `pyproject.toml` metadata**
- [ ] **Step 3: Commit**

```bash
git commit -m "rebrand: update python bindings and examples"
```

### Task 5: Update Documentation and Metadata

**Files:**
- Modify: `README.md`
- Modify: `ROADMAP.md`
- Modify: `CHANGELOG.md`
- Modify: `RELEASE_NOTES_v0.3.0.md`
- Modify: `docs/**/*.md`
- Modify: `SECURITY.md`
- Modify: `GITHUB_PROJECTS_SETUP.md`

- [ ] **Step 1: Update all documentation files with new branding**
- [ ] **Step 2: Update URLs and repository links**
- [ ] **Step 3: Commit**

```bash
git commit -m "rebrand: update documentation and metadata"
```

### Task 6: Verification and Build

**Files:**
- Test: All crates
- Test: Python bindings

- [ ] **Step 1: Run `cargo check` to verify Rust code**
- [ ] **Step 2: Run `cargo test` to verify integration tests**
- [ ] **Step 3: Verify python bindings can be imported**
- [ ] **Step 4: Commit**

```bash
git commit -m "rebrand: final verification and build fixes"
```
