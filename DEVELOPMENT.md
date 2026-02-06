# Development Guide: mdbook-excalidraw

## Project Overview

mdbook-excalidraw is a preprocessor for mdBook that converts Mermaid diagrams to interactive Excalidraw format, inspired by mdbook-mermaid and powered by @excalidraw/mermaid-to-excalidraw.

## Architecture

### Component Breakdown

```
┌─────────────────────────────────────────────────────────┐
│                    mdBook Build Process                  │
├─────────────────────────────────────────────────────────┤
│  1. Read markdown files                                  │
│  2. Run preprocessors (mdbook-excalidraw)               │
│  3. Generate HTML                                        │
│  4. Inject JS/CSS assets                                │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│              mdbook-excalidraw Preprocessor              │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐    ┌──────────────┐                  │
│  │  src/lib.rs  │───▶│ Markdown     │                  │
│  │              │    │ Parser       │                  │
│  │ Core Logic   │    │ (pulldown-   │                  │
│  │              │    │  cmark)      │                  │
│  └──────────────┘    └──────────────┘                  │
│         │                    │                          │
│         ▼                    ▼                          │
│  ┌──────────────────────────────┐                      │
│  │  Find Mermaid Code Blocks    │                      │
│  └──────────────────────────────┘                      │
│         │                                               │
│         ▼                                               │
│  ┌──────────────────────────────┐                      │
│  │  HTML Escape (XSS Prevention)│                      │
│  └──────────────────────────────┘                      │
│         │                                               │
│         ▼                                               │
│  ┌──────────────────────────────┐                      │
│  │  Wrap in Excalidraw Container│                      │
│  └──────────────────────────────┘                      │
│         │                                               │
│         ▼                                               │
│  ┌──────────────────────────────┐                      │
│  │  Return Modified Markdown    │                      │
│  └──────────────────────────────┘                      │
│                                                          │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│              Client-Side Rendering (Browser)             │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐    ┌──────────────┐                  │
│  │excalidraw.js │───▶│Load CDN libs │                  │
│  │              │    │@excalidraw/  │                  │
│  │Initialization│    │mermaid-to-   │                  │
│  │              │    │excalidraw    │                  │
│  └──────────────┘    └──────────────┘                  │
│         │                    │                          │
│         ▼                    ▼                          │
│  ┌──────────────────────────────┐                      │
│  │  Parse Mermaid from HTML     │                      │
│  └──────────────────────────────┘                      │
│         │                                               │
│         ▼                                               │
│  ┌──────────────────────────────┐                      │
│  │  Convert to Excalidraw       │                      │
│  │  (parseMermaidToExcalidraw)  │                      │
│  └──────────────────────────────┘                      │
│         │                                               │
│         ▼                                               │
│  ┌──────────────────────────────┐                      │
│  │  Render SVG Preview          │                      │
│  └──────────────────────────────┘                      │
│         │                                               │
│         ▼                                               │
│  ┌──────────────────────────────┐                      │
│  │  Add Edit Button & Controls  │                      │
│  └──────────────────────────────┘                      │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### File Structure

```
mdbook-excalidraw/
├── Cargo.toml                          # Rust package manifest
├── LICENSE                             # MIT license
├── README.md                           # User-facing documentation
├── USAGE.md                            # Detailed usage guide
├── DEVELOPMENT.md                      # This file
├── .gitignore                          # Git ignore patterns
│
├── src/
│   ├── lib.rs                          # Core preprocessing logic
│   │   ├── escape_html()               # XSS prevention
│   │   └── add_excalidraw()            # Main transform function
│   │
│   └── bin/
│       ├── mdbook-excalidraw.rs        # CLI binary
│       │   ├── ExcalidrawPreprocessor  # mdBook integration
│       │   ├── handle_supports()       # Renderer support check
│       │   ├── handle_install()        # Asset installation
│       │   └── handle_preprocessing()  # Main preprocessor entry
│       │
│       └── assets/
│           ├── excalidraw-init.js      # Client-side converter
│           │   ├── loadScript()        # Dynamic CDN loading
│           │   ├── initExcalidraw()    # Initialize diagrams
│           │   ├── createExcalidrawSVG()  # SVG renderer
│           │   └── openExcalidrawEditor() # Modal editor
│           │
│           └── excalidraw-style.css    # Styling
│               ├── .excalidraw-wrapper
│               ├── .excalidraw-container
│               ├── .excalidraw-edit-button
│               └── .excalidraw-modal
│
└── target/
    └── release/
        └── mdbook-excalidraw           # Compiled binary
```

## Code Flow

### 1. Build-Time Processing (Rust)

```rust
// src/lib.rs

pub fn add_excalidraw(content: &str) -> Result<String> {
    // 1. Parse markdown with pulldown-cmark
    let parser = Parser::new(content);

    // 2. Iterate through events
    for (event, range) in parser.into_offset_iter() {
        match event {
            // 3. Find mermaid code blocks
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang)))
                if lang.as_ref() == "mermaid" => {
                // Start collecting mermaid content
            }

            // 4. Collect text content
            Event::Text(text) if in_mermaid_block => {
                current_mermaid.push_str(&text);
            }

            // 5. End of block - transform
            Event::End(TagEnd::CodeBlock) if in_mermaid_block => {
                // a. Escape HTML for security
                let escaped = escape_html(&current_mermaid);

                // b. Create excalidraw container
                let html = format!(r#"
                    <div class="excalidraw-wrapper" id="...">
                        <div data-mermaid="{escaped}">
                            Loading...
                        </div>
                    </div>
                "#);

                // c. Store for replacement
                mermaid_blocks.push((range, html));
            }
        }
    }

    // 6. Replace all blocks in reverse order
    for (span, block) in mermaid_blocks.iter().rev() {
        result = replace_at_range(result, span, block);
    }

    Ok(result)
}
```

### 2. Runtime Processing (JavaScript)

```javascript
// src/bin/assets/excalidraw-init.js

async function initExcalidraw() {
    // 1. Find all excalidraw containers
    const containers = document.querySelectorAll('.excalidraw-container');

    for (const container of containers) {
        // 2. Get mermaid data from HTML attribute
        const mermaidData = container.getAttribute('data-mermaid');

        // 3. Decode HTML entities
        const decoded = decodeHTMLEntities(mermaidData);

        // 4. Load mermaid-to-excalidraw library from CDN
        const { parseMermaidToExcalidraw } = await import(CDN_URL);

        // 5. Convert mermaid to excalidraw
        const { elements, files } = await parseMermaidToExcalidraw(decoded);

        // 6. Render SVG preview
        const svg = createExcalidrawSVG(elements);
        container.appendChild(svg);

        // 7. Add edit button
        const button = createEditButton(elements, files);
        container.appendChild(button);
    }
}
```

## Security Considerations

### HTML Escaping

All user-provided mermaid content is escaped to prevent XSS attacks:

```rust
fn escape_html(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '<' => "&lt;",
            '>' => "&gt;",
            '"' => "&quot;",
            '&' => "&amp;",
            '\'' => "&#39;",
            _ => c,
        })
        .collect()
}
```

### CDN Security

- Uses official @excalidraw packages from unpkg.com
- All processing happens client-side (no data sent to servers)
- Content Security Policy compatible

### Input Validation

- Only processes `mermaid` fenced code blocks
- Ignores other languages (`rust`, `bash`, etc.)
- Validates mermaid syntax on client-side

## Testing

### Unit Tests

```rust
// src/lib.rs

#[test]
fn test_escape_html() {
    assert_eq!(
        escape_html("<script>alert('XSS')</script>"),
        "&lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;"
    );
}

#[test]
fn test_add_excalidraw_simple() {
    let input = r#"```mermaid
graph TD
    A-->B
```"#;

    let result = add_excalidraw(input).unwrap();

    assert!(result.contains("excalidraw-wrapper"));
    assert!(result.contains("data-mermaid"));
}
```

### Integration Testing

Create test book:

```bash
# 1. Create test book
mdbook init test-book
cd test-book

# 2. Install preprocessor
cargo run --bin mdbook-excalidraw -- install

# 3. Add test diagrams
echo '```mermaid
graph TD
    A-->B
```' >> src/chapter_1.md

# 4. Build
mdbook build

# 5. Check output
cat book/chapter_1.html | grep excalidraw
```

## Building and Installation

### Development Build

```bash
cargo build
./target/debug/mdbook-excalidraw --help
```

### Release Build

```bash
cargo build --release
sudo cp target/release/mdbook-excalidraw /usr/local/bin/
```

### Local Installation

```bash
cargo install --path .
```

## Debugging

### Enable Logging

```bash
# Set log level
export RUST_LOG=debug

# Run preprocessor
mdbook build

# Output includes:
# [DEBUG] Processing chapter: Introduction
# [INFO] Running excalidraw preprocessor
```

### Debug JavaScript

Open browser console (F12) and check for:

```
mdbook-excalidraw: Initializing...
mdbook-excalidraw: Found 3 diagrams
mdbook-excalidraw: Converting diagram 1...
mdbook-excalidraw: Success!
```

### Common Issues

**Issue**: Preprocessor not running

**Debug**:
```bash
# Check if configured
grep excalidraw book.toml

# Check if binary is in PATH
which mdbook-excalidraw

# Test supports command
mdbook-excalidraw supports html
echo $?  # Should be 0
```

**Issue**: Diagrams not rendering

**Debug**:
```bash
# Check if assets installed
ls theme/excalidraw.*

# Check if JavaScript loaded
# Browser console → Network tab → Look for excalidraw.js

# Check for JavaScript errors
# Browser console → Console tab
```

## Contributing

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- Add tests for new features
- Update documentation

### Pull Request Process

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing`
3. Make changes and test
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Commit: `git commit -m "Add amazing feature"`
7. Push: `git push origin feature/amazing`
8. Open PR

### Adding New Features

Example: Add support for custom themes

1. **Update Rust code**:
```rust
// src/lib.rs
pub struct ExcalidrawConfig {
    pub theme: Option<String>,
}

pub fn add_excalidraw_with_config(
    content: &str,
    config: &ExcalidrawConfig
) -> Result<String> {
    // Use config.theme
}
```

2. **Update JavaScript**:
```javascript
// src/bin/assets/excalidraw-init.js
const theme = container.getAttribute('data-theme') || 'default';
```

3. **Add tests**:
```rust
#[test]
fn test_custom_theme() {
    let config = ExcalidrawConfig {
        theme: Some("dark".to_string()),
    };
    let result = add_excalidraw_with_config(input, &config).unwrap();
    assert!(result.contains("data-theme=\"dark\""));
}
```

4. **Update documentation**:
- README.md - Add configuration example
- USAGE.md - Add usage instructions
- DEVELOPMENT.md - Add architecture notes

## Performance

### Build-Time

- **Preprocessing**: ~1-2ms per diagram
- **Binary size**: ~5MB (includes mdbook dependencies)
- **Memory usage**: Minimal (processes one chapter at a time)

### Runtime

- **First load**: ~500ms (CDN library download)
- **Subsequent loads**: Instant (cached)
- **SVG rendering**: ~10-50ms per diagram
- **Memory**: ~2-5MB per diagram

### Optimization Tips

1. **Reduce build time**:
   - Use `cargo build --release` for production
   - Cache dependencies in CI/CD

2. **Reduce client-side load time**:
   - Self-host CDN libraries for faster loading
   - Lazy-load diagrams (only render when visible)

## Roadmap

### Version 0.2.0

- [ ] Add configuration options for themes
- [ ] Support offline mode (bundle libraries)
- [ ] Add diagram caching

### Version 0.3.0

- [ ] Interactive editing in modal
- [ ] Export to Excalidraw format
- [ ] Advanced styling options

### Version 1.0.0

- [ ] Full Excalidraw editor integration
- [ ] Save/load diagram state
- [ ] Collaborative editing support

## License

MIT License - See LICENSE file

## Credits

- Based on mdbook-mermaid architecture
- Uses @excalidraw/mermaid-to-excalidraw library
- Inspired by Excalidraw and Mermaid projects

## Support

- GitHub Issues: Report bugs
- Discussions: Ask questions
- Documentation: README.md and USAGE.md
