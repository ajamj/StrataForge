# Rebranding Design: StrataForge → Seisly

> **Date:** 2026-03-29  
> **Status:** Approved  
> **Version:** 1.0

---

## 1. Brand Identity

### New Name
**Seisly** (pronounced: /ˈsaɪzli/)

### Tagline
**"Rust-Powered Seismic Studio"**

### Rationale
- **Seis** - From "seismic", core business
- **ly** - Modern tech suffix (like "quickly", "simply")
- **Meaning** - "Seismic, simplified" or "Simply seismic"

### Crate Naming
All crates renamed from `sf_*` to `seisly_*`:

| Old Name | New Name |
|----------|----------|
| `sf_core` | `seisly_core` |
| `sf_io` | `seisly_io` |
| `sf_compute` | `seisly_compute` |
| `sf_render` | `seisly_render` |
| `sf_storage` | `seisly_storage` |
| `sf_app` | `seisly_app` |
| `sf_cli` | `seisly_cli` |
| `sf_ml` | `seisly_ml` |
| `sf_qi` | `seisly_qi` |
| `sf_4d` | `seisly_4d` |
| `sf_tracking` | `seisly_tracking` |
| `sf_production` | `seisly_production` |
| `sf_plugin` | `seisly_plugin` |
| `sf_attributes` | `seisly_attributes` |
| `sf_attributes_gpu` | `seisly_attributes_gpu` |
| `sf_fwi` | `seisly_fwi` |

---

## 2. Migration Plan

### Phase 1: Documentation (Immediate)
- [ ] Update README.md with new name
- [ ] Update all docs references
- [ ] Update Cargo.toml descriptions
- [ ] Update GitHub repo metadata

### Phase 2: Crate Rename (Core)
- [ ] Rename all crate directories
- [ ] Update workspace Cargo.toml
- [ ] Update all internal dependencies
- [ ] Update use statements in code

### Phase 3: Testing
- [ ] Build all crates
- [ ] Run all tests
- [ ] Verify GUI works
- [ ] Check CLI commands

### Phase 4: Deploy
- [ ] Update GitHub Pages
- [ ] Update documentation
- [ ] Push to GitHub
- [ ] Announce rebranding

---

## 3. Documentation & gh-pages

### Current Issue
gh-pages branch empty, showing only placeholder text.

### Solution
1. Trigger auto-generate-docs workflow
2. Deploy generated docs to gh-pages
3. Verify at https://ajamj.github.io/StrataForge

### Workflow
```yaml
.github/workflows/auto-generate-docs.yml
```

Triggers on:
- Push to master (crates/**, Cargo.toml)
- Manual dispatch

Generates:
- API docs (rustdoc)
- Features summary

---

## 4. File Changes Required

### High-Impact Files
1. `Cargo.toml` (workspace root)
2. `README.md`
3. `crates/*/Cargo.toml` (all 16 crates)
4. `crates/*/src/lib.rs` (all crates)
5. `docs/*.md` (all documentation)
6. `.github/workflows/*.yml`

### Code Changes
- All `use sf_*` → `use seisly_*`
- All `StrataForge` → `Seisly` (in comments, docs)
- All package names in Cargo.toml

---

## 5. Risk Mitigation

### Risks
1. **Broken imports** - Mitigated by systematic rename
2. **Lost git history** - Mitigated by using `git mv`
3. **Workflow failures** - Mitigated by testing after each phase
4. **User confusion** - Mitigated by clear communication

### Rollback Plan
If issues found:
```bash
git revert <rebranding-commit>
# Or restore from backup branch
git checkout backup-before-rebrand
```

---

## 6. Success Criteria

- [ ] All 16 crates renamed successfully
- [ ] All tests passing
- [ ] GUI builds and runs
- [ ] CLI commands work
- [ ] Documentation deployed
- [ ] No broken imports
- [ ] Clean git history

---

**Approved by:** User  
**Implementation Status:** In Progress
