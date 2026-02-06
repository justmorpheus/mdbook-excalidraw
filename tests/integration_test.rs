use mdbook_excalidraw::add_excalidraw;

#[test]
fn test_simple_mermaid_conversion() {
    let input = r#"# Test

```mermaid
graph TD
    A-->B
```

Some text"#;

    let result = add_excalidraw(input).unwrap();

    assert!(result.contains("excalidraw-wrapper"));
    assert!(result.contains("data-mermaid"));
    assert!(result.contains("graph TD"));
    assert!(result.contains("A--&gt;B"));
}

#[test]
fn test_html_escaping() {
    let input = r#"```mermaid
graph TD
    A["<script>alert('xss')</script>"]-->B
```"#;

    let result = add_excalidraw(input).unwrap();

    assert!(result.contains("&lt;script&gt;"));
    assert!(result.contains("&lt;/script&gt;"));
    assert!(!result.contains("<script>"));
}

#[test]
fn test_newline_escaping() {
    let input = r#"```mermaid
graph TD
    A-->B
    B-->C
```"#;

    let result = add_excalidraw(input).unwrap();

    // Newlines should be escaped as &#10; in data-mermaid attribute
    assert!(result.contains("&#10;"));
}

#[test]
fn test_multiple_diagrams() {
    let input = r#"# Test

```mermaid
graph TD
    A-->B
```

Some text

```mermaid
sequenceDiagram
    Alice->>Bob: Hello
```"#;

    let result = add_excalidraw(input).unwrap();

    assert!(result.contains("excalidraw-0"));
    assert!(result.contains("excalidraw-1"));
    assert!(result.contains("graph TD"));
    assert!(result.contains("sequenceDiagram"));
}

#[test]
fn test_non_mermaid_blocks_unchanged() {
    let input = r#"```rust
fn main() {
    println!("Hello");
}
```"#;

    let result = add_excalidraw(input).unwrap();

    assert!(result.contains("```rust"));
    assert!(!result.contains("excalidraw"));
}

#[test]
fn test_view_source_section() {
    let input = r#"```mermaid
graph TD
    A-->B
```"#;

    let result = add_excalidraw(input).unwrap();

    assert!(result.contains("excalidraw-source"));
    assert!(result.contains("View Mermaid Source"));
}

#[test]
fn test_empty_mermaid_block() {
    let input = r#"```mermaid
```"#;

    let result = add_excalidraw(input).unwrap();

    assert!(result.contains("excalidraw-wrapper"));
}

#[test]
fn test_special_characters() {
    let input = r#"```mermaid
graph TD
    A["Test & <thing>"]-->B
```"#;

    let result = add_excalidraw(input).unwrap();

    assert!(result.contains("&amp;"));
    assert!(result.contains("&lt;"));
    assert!(result.contains("&gt;"));
}

#[test]
fn test_quotes_in_diagram() {
    let input = r#"```mermaid
graph TD
    A["He said 'hello'"]-->B
```"#;

    let result = add_excalidraw(input).unwrap();

    assert!(result.contains("&#39;"));
}
