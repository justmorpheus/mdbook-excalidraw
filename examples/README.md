# Examples

## Quick Start

```bash
# Install
cargo install --path ..

# Create book
mdbook init my-book
cd my-book

# Install preprocessor
mdbook-excalidraw install

# Add diagram
cat >> src/chapter_1.md << 'EOF'

## Flowchart

\`\`\`mermaid
graph TB
    A[Issue] --> B{Type?}
    B -->|Code| C[Fix Code]
    B -->|Docker| D[Check Docker]
    B -->|K8s| E[Check K8s]

    C --> F[Deploy]
    D --> F
    E --> F
\`\`\`

## Sequence

\`\`\`mermaid
sequenceDiagram
    Client->>Server: Request
    Server->>DB: Query
    DB-->>Server: Data
    Server-->>Client: Response
\`\`\`

## Class Diagram

\`\`\`mermaid
classDiagram
    Animal <|-- Dog
    Animal <|-- Cat
    Animal: +name
    Animal: +age
    Dog: +breed
    Cat: +indoor
\`\`\`
EOF

# Serve
mdbook serve --open
```

## Features Demonstrated

- Flowcharts with styling
- Sequence diagrams
- Class diagrams
- Multiple diagrams per page
- Zoom controls
- View source toggle
