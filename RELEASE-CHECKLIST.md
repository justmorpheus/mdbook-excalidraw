# Release Checklist

## Pre-Release

### Repository
- [x] Sponsor button (.github/FUNDING.yml)
- [x] Author: Divyanshu <help@peachycloudsecurity.com>
- [x] Repository: https://github.com/peachycloudsecurity/mdbook-excalidraw
- [x] All URLs updated
- [x] Contributors file

### Code Quality
- [ ] cargo test - all pass
- [ ] cargo clippy -- -D warnings
- [ ] cargo fmt --check
- [ ] cargo build --release
- [ ] Manual testing

### Documentation
- [x] README.md updated
- [x] CHANGELOG.md v0.1.0
- [x] PUBLISHING.md complete
- [x] Examples directory
- [x] Sponsor badges
- [x] Contact info

### GitHub
- [ ] Create repo: https://github.com/peachycloudsecurity/mdbook-excalidraw
- [ ] Push code
- [ ] Enable Sponsors
- [ ] Ko-fi integration
- [ ] Add CARGO_TOKEN secret
- [ ] Verify CI runs
- [ ] Create release

## Publishing

### GitHub Repository

```bash
cd /Users/divyanshushukla/Downloads/training-kubernetes-security-main/attacking-and-auditing-mdbook/mdbook-excalidraw

git init
git add .
git commit -m "Initial commit - v0.1.0"
git remote add origin https://github.com/peachycloudsecurity/mdbook-excalidraw.git
git branch -M main
git push -u origin main
```

### Create Release

```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

GitHub Actions will automatically:
- Run tests (Linux, macOS, Windows)
- Build binaries
- Create release
- Publish to crates.io (if CARGO_TOKEN set)

### Manual Publish

If automation fails:

```bash
cargo login <token>
cargo publish --dry-run
cargo publish
```

### Verify

- [ ] Check https://crates.io/crates/mdbook-excalidraw
- [ ] Test: cargo install mdbook-excalidraw
- [ ] Check GitHub Actions
- [ ] Verify release assets

## Post-Release

- [ ] Monitor crates.io stats
- [ ] Watch GitHub issues
- [ ] Check CI status

## Test Installation

```bash
cargo install mdbook-excalidraw

mdbook init test-book
cd test-book
mdbook-excalidraw install

cat >> src/chapter_1.md << 'EOF'

\`\`\`mermaid
graph TD
    A[Start] --> B[End]
\`\`\`
EOF

mdbook serve --open
```

## Status

| Item | Status |
|------|--------|
| Code | Done |
| Tests | Done |
| CI/CD | Done |
| Docs | Done |
| Sponsor | Done |
| GitHub | Pending |
| Crates.io | Pending |

## Support

Issues: help@peachycloudsecurity.com

See PUBLISHING.md for details.
