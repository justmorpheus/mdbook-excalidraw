use anyhow::Result;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};

/// Escape HTML special characters to prevent XSS
fn escape_html(s: &str) -> String {
    let mut output = String::new();
    for c in s.chars() {
        match c {
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '&' => output.push_str("&amp;"),
            '\'' => output.push_str("&#39;"),
            _ => output.push(c),
        }
    }
    output
}

/// Process markdown content and convert mermaid code blocks to excalidraw format
pub fn add_excalidraw(content: &str) -> Result<String> {
    let mut mermaid_blocks = Vec::new();

    let parser = Parser::new(content);
    let mut in_mermaid_block = false;
    let mut current_mermaid = String::new();

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                if lang.as_ref() == "mermaid" {
                    in_mermaid_block = true;
                    current_mermaid.clear();
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                if in_mermaid_block {
                    in_mermaid_block = false;

                    // Generate unique ID for this diagram
                    let diagram_id = format!("excalidraw-{}", mermaid_blocks.len());

                    // Escape the mermaid content for security
                    let escaped_mermaid = escape_html(&current_mermaid);

                    // For the data attribute, also replace newlines with &#10; to keep it valid HTML
                    let escaped_for_attribute = escaped_mermaid.replace('\n', "&#10;");

                    // Create the excalidraw container with embedded mermaid data
                    let excalidraw_html = format!(
                        r#"<div class="excalidraw-wrapper" id="{id}">
  <div class="excalidraw-container" data-mermaid="{mermaid}">
    <div class="excalidraw-loading">Loading Excalidraw diagram...</div>
  </div>
  <details class="excalidraw-source">
    <summary>View Mermaid Source</summary>
    <pre><code class="language-mermaid">{source}</code></pre>
  </details>
</div>"#,
                        id = diagram_id,
                        mermaid = escaped_for_attribute,
                        source = escaped_mermaid
                    );

                    mermaid_blocks.push((range.start..range.end, excalidraw_html));
                    current_mermaid.clear();
                }
            }
            Event::Text(text) => {
                if in_mermaid_block {
                    current_mermaid.push_str(&text);
                }
            }
            Event::Code(text) => {
                if in_mermaid_block {
                    current_mermaid.push_str(&text);
                }
            }
            Event::SoftBreak => {
                if in_mermaid_block {
                    current_mermaid.push('\n');
                }
            }
            Event::HardBreak => {
                if in_mermaid_block {
                    current_mermaid.push('\n');
                }
            }
            Event::Html(html) => {
                if in_mermaid_block {
                    current_mermaid.push_str(&html);
                }
            }
            _ => {}
        }
    }

    // Replace mermaid blocks with excalidraw HTML (forward order, using original offsets)
    let mut result = String::with_capacity(content.len());
    let mut last_end = 0;
    for (span, block) in mermaid_blocks.iter() {
        result.push_str(&content[last_end..span.start]);
        result.push_str(block);
        last_end = span.end;
    }
    result.push_str(&content[last_end..]);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html() {
        assert_eq!(
            escape_html("<script>alert('XSS')</script>"),
            "&lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;"
        );
    }

    #[test]
    fn test_add_excalidraw_simple() {
        let content = r#"# Test

```mermaid
graph TD
    A-->B
```

Some text"#;

        let result = add_excalidraw(content).unwrap();
        assert!(result.contains("excalidraw-wrapper"));
        assert!(result.contains("data-mermaid"));
        assert!(result.contains("graph TD"));
    }

    #[test]
    fn test_add_excalidraw_with_special_chars() {
        let content = r#"```mermaid
graph TD
    A["<Component>"]-->B
```"#;

        let result = add_excalidraw(content).unwrap();
        assert!(result.contains("&lt;Component&gt;"));
    }

    #[test]
    fn test_multiple_mermaid_blocks() {
        let content = r#"# Test

```mermaid
graph TD
    A-->B
```

Some text

```mermaid
sequenceDiagram
    Alice->>Bob: Hello
```"#;

        let result = add_excalidraw(content).unwrap();
        assert!(result.contains("excalidraw-0"));
        assert!(result.contains("excalidraw-1"));
    }

    #[test]
    fn test_non_mermaid_blocks_unchanged() {
        let content = r#"```rust
fn main() {
    println!("Hello");
}
```"#;

        let result = add_excalidraw(content).unwrap();
        assert!(result.contains("```rust"));
        assert!(!result.contains("excalidraw"));
    }
}
