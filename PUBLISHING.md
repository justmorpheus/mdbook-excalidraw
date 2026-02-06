# Publishing Guide

## Prerequisites

1. Crates.io account - https://crates.io
2. GitHub repository
3. Cargo token from https://crates.io/me

## Pre-Publishing

### Update Version

```toml
[package]
version = "0.1.0"
```

### Update CHANGELOG.md

Add release notes for the version.

### Run Tests

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

All must pass.

### Check Package

```bash
cargo package --list
```

Verify all files included.

### Test Install

```bash
cargo install --path . --force
mdbook-excalidraw --version
```

## Publish to Crates.io

### Login

```bash
cargo login <token>
```

### Dry Run

```bash
cargo publish --dry-run
```

### Publish

```bash
cargo publish
```

Note: Cannot delete or modify after publishing.

### Verify

Visit https://crates.io/crates/mdbook-excalidraw

Test:
```bash
cargo install mdbook-excalidraw
```

## GitHub Release

### Commit and Tag

```bash
git add .
git commit -m "Release v0.1.0"
git push origin main

git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### Automated

GitHub Actions (`.github/workflows/release.yml`) will:
- Create release
- Build binaries (Linux, macOS, Windows)
- Upload assets
- Publish to crates.io

### Manual

If automation fails:

1. Go to https://github.com/peachycloudsecurity/mdbook-excalidraw/releases/new
2. Choose tag: v0.1.0
3. Copy from CHANGELOG.md
4. Attach binaries from target/release/
5. Publish

## Post-Publishing

Monitor crates.io downloads and GitHub issues.

## Troubleshooting

### "crate name exists"

Name taken. Choose different name in Cargo.toml.

### "failed to verify"

```bash
cargo package --list
```

Add missing files to Cargo.toml include list.

### Permission denied

```bash
cargo logout
cargo login <new-token>
```

### CI failing

Check CARGO_TOKEN secret in repository settings.

## Version Numbering

Semantic Versioning:
- Major (1.0.0): Breaking changes
- Minor (0.1.0): New features
- Patch (0.0.1): Bug fixes

## Checklist

- [ ] Update Cargo.toml version
- [ ] Update CHANGELOG.md
- [ ] cargo test
- [ ] cargo clippy
- [ ] cargo fmt --check
- [ ] cargo package --dry-run
- [ ] Test install locally
- [ ] Commit and push
- [ ] Create tag
- [ ] Wait for CI
- [ ] Verify publication
