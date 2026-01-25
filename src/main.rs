//! A simple PDF splitter that extracts each page into a separate PDF file.

use lopdf::Document;
use std::fs;
use std::path::Path;

fn main() -> lopdf::Result<()> {
    // Define input and output paths
    let input_path = Path::new("input/sample.pdf");
    let output_dir = Path::new("output_pages");

    // Validate that the source PDF file exists
    if !input_path.exists() {
        eprintln!("Error: PDF file not found at '{}'", input_path.display());
        eprintln!("Please create an 'input' directory and place your PDF file inside.");
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
        let output_filename = format!(
            "{}/page_{:03}.pdf",
            output_dir.to_str().unwrap(),
            page_number
        );
        new_doc.save(&output_filename)?;
    }

    println!(
        "\n✅ Successfully split {} pages into '{}'",
        total_pages,
        output_dir.display()
    );

    Ok(())
}
