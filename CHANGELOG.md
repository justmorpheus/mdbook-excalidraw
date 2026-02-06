# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-02-07

### Added
- Mermaid diagram rendering with mermaid.js
- Interactive zoom controls (zoom in/out/reset)
- Keyboard shortcuts (Ctrl/Cmd + +/-, 0)
- View Mermaid source toggle
- Dark theme support
- HTML entity escaping for XSS prevention
- Newline escaping in HTML attributes
- Support for multiple diagrams per page
- Transparent backgrounds
- CLI installation command
- Automatic book.toml configuration
- Asset copying (JS and CSS)

### Security
- HTML escaping for all user input
- XSS attack prevention
- Uses mermaid.run() for proper theme handling

[Unreleased]: https://github.com/peachycloudsecurity/mdbook-excalidraw/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/peachycloudsecurity/mdbook-excalidraw/releases/tag/v0.1.0
