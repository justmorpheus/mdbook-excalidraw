use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use mdbook::book::Book;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use std::io;
use std::process;

const EXCALIDRAW_JS: &[u8] = include_bytes!("assets/excalidraw-init.js");
const EXCALIDRAW_CSS: &[u8] = include_bytes!("assets/excalidraw-style.css");

#[derive(Parser)]
#[command(name = "mdbook-excalidraw")]
#[command(about = "mdbook preprocessor to convert Mermaid diagrams to Excalidraw")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check whether the renderer supports this preprocessor
    Supports { renderer: String },
    /// Install excalidraw assets into the book directory
    Install {
        /// Path to the book directory (containing book.toml)
        #[arg(default_value = ".")]
        dir: String,
    },
}

pub struct ExcalidrawPreprocessor;

impl Preprocessor for ExcalidrawPreprocessor {
    fn name(&self) -> &str {
        "excalidraw"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        log::info!("Running excalidraw preprocessor");

        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chapter) = item {
                log::debug!("Processing chapter: {}", chapter.name);
                match mdbook_excalidraw::add_excalidraw(&chapter.content) {
                    Ok(new_content) => {
                        chapter.content = new_content;
                    }
                    Err(e) => {
                        log::error!("Error processing chapter {}: {}", chapter.name, e);
                    }
                }
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html" || renderer == "markdown"
    }
}

fn handle_preprocessing() -> Result<()> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        log::warn!(
            "mdbook version mismatch: expected {}, got {}",
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let preprocessor = ExcalidrawPreprocessor;
    let processed_book = preprocessor.run(&ctx, book)?;

    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(renderer: &str) -> Result<()> {
    let preprocessor = ExcalidrawPreprocessor;

    if preprocessor.supports_renderer(renderer) {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

fn handle_install(dir: &str) -> Result<()> {
    use std::fs;
    use std::io::Write;
    use std::path::Path;
    use toml_edit::{value, Array, DocumentMut, Item, Table};

    let book_dir = Path::new(dir);
    let book_toml = book_dir.join("book.toml");

    if !book_toml.exists() {
        return Err(anyhow!(
            "book.toml not found in {}. Please provide a valid book directory.",
            dir
        ));
    }

    // Create theme directory if it doesn't exist
    let theme_dir = book_dir.join("theme");
    if !theme_dir.exists() {
        fs::create_dir_all(&theme_dir)?;
        log::info!("Created theme directory: {}", theme_dir.display());
    }

    // Install JavaScript
    let js_path = theme_dir.join("excalidraw.js");
    if js_path.exists() {
        log::info!("excalidraw.js already exists, skipping...");
    } else {
        let mut js_file = fs::File::create(&js_path)?;
        js_file.write_all(EXCALIDRAW_JS)?;
        log::info!("Installed excalidraw.js to {}", js_path.display());
    }

    // Install CSS
    let css_path = theme_dir.join("excalidraw.css");
    if css_path.exists() {
        log::info!("excalidraw.css already exists, skipping...");
    } else {
        let mut css_file = fs::File::create(&css_path)?;
        css_file.write_all(EXCALIDRAW_CSS)?;
        log::info!("Installed excalidraw.css to {}", css_path.display());
    }

    // Update book.toml using toml_edit for safer editing
    let toml_content = fs::read_to_string(&book_toml)?;
    let mut doc = toml_content.parse::<DocumentMut>()?;

    // Add preprocessor configuration
    let empty_table = Item::Table(Table::default());

    let preprocessor = doc.entry("preprocessor").or_insert(empty_table.clone());

    let preprocessor_table = preprocessor
        .as_table_mut()
        .ok_or_else(|| anyhow!("preprocessor section is not a table"))?;

    if !preprocessor_table.contains_key("excalidraw") {
        let mut excalidraw_config = Table::default();
        excalidraw_config["command"] = value("mdbook-excalidraw");
        preprocessor_table["excalidraw"] = Item::Table(excalidraw_config);
        log::info!("Added excalidraw preprocessor configuration");
    } else {
        log::info!("excalidraw preprocessor already configured");
    }

    // Add theme files to output.html configuration
    let output = doc.entry("output").or_insert(empty_table.clone());

    let output_table = output
        .as_table_mut()
        .ok_or_else(|| anyhow!("output section is not a table"))?;

    let html = output_table.entry("html").or_insert(empty_table.clone());

    let html_table = html
        .as_table_mut()
        .ok_or_else(|| anyhow!("output.html section is not a table"))?;

    // Add additional JS
    let additional_js = html_table
        .entry("additional-js")
        .or_insert(Item::Value(toml_edit::Value::Array(Array::default())));

    let js_array = additional_js
        .as_array_mut()
        .ok_or_else(|| anyhow!("additional-js is not an array"))?;

    let excalidraw_js = "theme/excalidraw.js";
    let mut js_exists = false;
    for item in js_array.iter() {
        if let Some(s) = item.as_str() {
            if s == excalidraw_js {
                js_exists = true;
                break;
            }
        }
    }
    if !js_exists {
        js_array.push(excalidraw_js);
        log::info!("Added excalidraw.js to additional-js");
    }

    // Add additional CSS
    let additional_css = html_table
        .entry("additional-css")
        .or_insert(Item::Value(toml_edit::Value::Array(Array::default())));

    let css_array = additional_css
        .as_array_mut()
        .ok_or_else(|| anyhow!("additional-css is not an array"))?;

    let excalidraw_css = "theme/excalidraw.css";
    let mut css_exists = false;
    for item in css_array.iter() {
        if let Some(s) = item.as_str() {
            if s == excalidraw_css {
                css_exists = true;
                break;
            }
        }
    }
    if !css_exists {
        css_array.push(excalidraw_css);
        log::info!("Added excalidraw.css to additional-css");
    }

    // Write updated TOML
    fs::write(&book_toml, doc.to_string())?;

    println!("\n✅ mdbook-excalidraw installed successfully!");
    println!("\nNext steps:");
    println!("  1. Run 'mdbook build' to build your book");
    println!("  2. Mermaid code blocks will be converted to Excalidraw diagrams");
    println!("\nExample usage in markdown:");
    println!("  ```mermaid");
    println!("  graph TD");
    println!("      A[Start] --> B[Process]");
    println!("      B --> C[End]");
    println!("  ```");

    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Supports { renderer }) => handle_supports(&renderer),
        Some(Commands::Install { dir }) => handle_install(&dir),
        None => handle_preprocessing(),
    }
}
