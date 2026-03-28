# GitHub Setup Guide

## Prerequisites

1. **GitHub Account** - Sign up at https://github.com
2. **Git installed** - https://git-scm.com/downloads

## Step 1: Create GitHub Repository

1. Go to https://github.com/new
2. Repository name: `strataforge` (atau nama lain sesuai keinginan)
3. Description: "Open-source subsurface interpretation and modeling platform"
4. **DO NOT** initialize with README, .gitignore, or license (kita sudah punya)
5. Click "Create repository"

## Step 2: Link Local Repository

```bash
# Navigate to project directory
cd D:\GRC-Ajam\myfield

# Remove existing remote (if any)
git remote remove origin

# Add your GitHub remote (replace YOUR_USERNAME dengan username GitHub Anda)
git remote add origin https://github.com/YOUR_USERNAME/strataforge.git

# Verify remote
git remote -v

# Push to GitHub
git push -u origin master
```

## Step 3: Enable GitHub Actions

1. Go to your repository on GitHub
2. Click "Actions" tab
3. Click "I understand my workflows, go ahead and enable them"
4. CI/CD pipeline akan otomatis run pada setiap push

## Step 4: Setup Codecov (Optional)

Untuk code coverage reporting:

1. Go to https://codecov.io
2. Sign in with GitHub
3. Add your repository
4. Copy token dari Codecov dashboard
5. Di GitHub repository, pergi ke Settings → Secrets and variables → Actions
6. Add new secret: `CODECOV_TOKEN` dengan value dari Codecov

## Step 5: Setup crates.io Publishing (Optional)

Untuk publish crates ke crates.io:

1. Create account di https://crates.io
2. Get API token: `cargo login`
3. Di GitHub repository, pergi ke Settings → Secrets and variables → Actions
4. Add new secret: `CARGO_REGISTRY_TOKEN` dengan token dari crates.io

## Step 6: Create First Release

### Option A: Manual Release

```bash
# Tag current commit
git tag -a v0.1.1 -m "Release v0.1.1 - Initial beta release"

# Push tag
git push origin v0.1.1
```

### Option B: Automated Release

1. Go to Actions tab di GitHub
2. Pilih workflow "Automated Releases"
3. Click "Run workflow"
4. Input version number (e.g., v0.1.1)
5. Click "Run workflow"

## CI/CD Pipeline Features

### Automated on Every Push:
- ✅ Build on Windows, Linux, macOS
- ✅ Run all tests
- ✅ Code formatting check
- ✅ Clippy linting
- ✅ Upload build artifacts

### On Release Tag:
- ✅ Create GitHub release with binaries
- ✅ Publish to crates.io (all workspace crates)
- ✅ Deploy documentation to GitHub Pages
- ✅ Generate and upload coverage report

## Troubleshooting

### Push Failed: Authentication Error

```bash
# Setup Git credentials
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"

# Use GitHub Personal Access Token instead of password
# Generate token di: https://github.com/settings/tokens
# Scopes: repo, workflow

# Update remote dengan token
git remote set-url origin https://YOUR_TOKEN@github.com/YOUR_USERNAME/strataforge.git
```

### Actions Not Running

- Pastikan Actions enabled di repository settings
- Check workflow permissions di Settings → Actions
- Verify workflow files ada di `.github/workflows/`

### Coverage Not Uploading

- Verify Codecov token setup
- Check workflow logs untuk error messages
- Pastikan coverage file generated (coverage/cobertura.xml)

## Next Steps

Setelah setup GitHub:

1. ✅ Enable branch protection untuk `master`
2. ✅ Setup CODEOWNERS file untuk code review
3. ✅ Add CONTRIBUTING.md untuk contributors
4. ✅ Setup project board untuk issue tracking
5. ✅ Add release checklist di RELEASES.md

---

**Repository is now ready for collaborative development! 🚀**
