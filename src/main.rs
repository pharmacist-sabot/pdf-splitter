//! A simple PDF splitter that extracts each page into a separate PDF file.

use clap::Parser;
use lopdf::Document;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input PDF file to be split.
    /// Example: /path/to/document.pdf
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Path to the output directory where split pages will be saved.
    /// Defaults to "output_pages" in the current working directory.
    /// Example: /path/to/output/
    #[arg(short, long, value_name = "DIR", default_value = "output_pages")]
    output: PathBuf,
}

fn main() -> lopdf::Result<()> {
    let args = Args::parse();

    // Define input and output paths from CLI args
    let input_path = &args.input;
    let output_dir = &args.output;

    // Validate that the source PDF file exists
    if !input_path.exists() {
        eprintln!("Error: PDF file not found at '{}'", input_path.display());
        eprintln!(
            "Please provide a valid PDF file path (for example: /path/to/document.pdf) using the `--input` / `-i` argument."
        );
        std::process::exit(1);
    }

    // Create the output directory if it doesn't exist.
    // `create_dir_all` handles nested directories and doesn't error if the directory exists.
    fs::create_dir_all(output_dir)?;

    println!("Starting to split: {}", input_path.display());

    // Load the source PDF document
    let doc = Document::load(input_path)?;

    // Get all page numbers from the document
    let pages = doc.get_pages();
    let total_pages = pages.len();
    let all_page_numbers: Vec<u32> = pages.keys().copied().collect();

    for page_number in &all_page_numbers {
        println!("Processing page {}/{}...", page_number, total_pages);

        // Create a new document for each page by cloning the original
        // and removing all pages except the current one.
        let mut new_doc = doc.clone();

        // Collect page numbers to delete (all pages except current)
        let pages_to_delete: Vec<u32> = all_page_numbers
            .iter()
            .filter(|&&p| p != *page_number)
            .copied()
            .collect();

        // Remove unwanted pages
        new_doc.delete_pages(&pages_to_delete);

        // Clean up unused objects and renumber for a smaller file size
        new_doc.prune_objects();
        new_doc.renumber_objects();

        // Save the single-page document
        // Construct the output path using `join` to avoid UTF-8 conversion and string concatenation.
        let output_path = output_dir.join(format!("page_{:03}.pdf", page_number));
        new_doc.save(&output_path)?;
    }

    println!(
        "\n✅ Successfully split {} pages into '{}'",
        total_pages,
        output_dir.display()
    );

    Ok(())
}
