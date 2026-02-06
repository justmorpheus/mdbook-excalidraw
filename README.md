# mdbook-excalidraw

[![Crates.io](https://img.shields.io/crates/v/mdbook-excalidraw.svg)](https://crates.io/crates/mdbook-excalidraw)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/peachycloudsecurity/mdbook-excalidraw/workflows/CI/badge.svg)](https://github.com/peachycloudsecurity/mdbook-excalidraw/actions)
[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%9D%A4-ff69b4)](https://github.com/sponsors/peachycloudsecurity)

mdBook preprocessor for rendering Mermaid diagrams with zoom controls.

## Features

- Render Mermaid diagrams using mermaid.js
- Interactive zoom controls (zoom in/out, reset)
- Keyboard shortcuts (Ctrl/Cmd + +/-, 0)
- View original Mermaid source
- Support for all Mermaid diagram types
- Dark theme support
- HTML escaping for security
- Transparent backgrounds
- Multiple diagrams per page

## Installation

### From crates.io

```bash
cargo install mdbook-excalidraw
```

### From source

```bash
git clone https://github.com/peachycloudsecurity/mdbook-excalidraw
cd mdbook-excalidraw
cargo install --path .
```

## Usage

### Install into mdBook project

```bash
cd your-mdbook-project
mdbook-excalidraw install
```

This copies assets to `theme/` and updates `book.toml`.

### Write Mermaid diagrams

```markdown
\`\`\`mermaid
graph TD
    A[Start] --> B{Working?}
    B -->|Yes| C[Done]
    B -->|No| D[Debug]
\`\`\`
```

### Build

```bash
mdbook build
```

## Configuration

### Automatic

```bash
mdbook-excalidraw install
```

### Manual

Add to `book.toml`:

```toml
[preprocessor.excalidraw]
command = "mdbook-excalidraw"

[output.html]
additional-js = ["mermaid.min.js", "mermaid-init.js", "theme/excalidraw.js"]
additional-css = ["theme/excalidraw.css"]
```

Note: Requires `mermaid.min.js` and `mermaid-init.js` for rendering.

## Zoom Controls

- Located top-right corner
- 60% opacity, 100% on hover
- Buttons: − (out), percentage, + (in), ⟲ (reset)
- Keyboard: Ctrl/Cmd + +/- for zoom, 0 for reset

## Supported Diagrams

All Mermaid types supported:
- Flowcharts
- Sequence diagrams
- Class diagrams
- State diagrams
- ER diagrams
- Gantt charts
- Pie charts
- And more

## Security

HTML escaping prevents XSS attacks. Newlines escaped as `&#10;` in attributes.

```rust
fn escape_html(s: &str) -> String {
    // Escapes: < > " & '
}
```

All rendering is client-side. No external requests.

## Development

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Lint

```bash
cargo clippy -- -D warnings
cargo fmt --check
```

## Troubleshooting

### Preprocessor not running

Check `book.toml`:

```toml
[preprocessor.excalidraw]
command = "mdbook-excalidraw"
```

### Assets not loading

Verify files exist in `theme/` and are listed in `book.toml`:

```toml
[output.html]
additional-js = ["mermaid.min.js", "mermaid-init.js", "theme/excalidraw.js"]
additional-css = ["theme/excalidraw.css"]
```

### Diagrams not rendering

1. Check browser console for errors
2. Verify mermaid.js loaded
3. Validate syntax at mermaid.live

## Contributing

Fork, create feature branch, add tests, run clippy and fmt, submit PR.

## License

GNU General Public License v3.0 - see LICENSE file.

## Author

Divyanshu - Peachy Cloud Security

Contact: help@peachycloudsecurity.com

## Support

GitHub Issues: https://github.com/peachycloudsecurity/mdbook-excalidraw/issues

### Sponsor

[![Sponsor on GitHub](https://img.shields.io/badge/Sponsor-GitHub-ea4aaa?logo=github-sponsors)](https://github.com/sponsors/peachycloudsecurity)
[![Ko-fi](https://img.shields.io/badge/Ko--fi-Support-ff5e5b?logo=ko-fi)](https://ko-fi.com/peachycloudsecurity)
