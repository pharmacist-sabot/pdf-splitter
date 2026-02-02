# PDF Splitter

A robust, command-line utility written in Rust for splitting multi-page PDF documents into individual PDF files.

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## 📖 Overview

`pdf-splitter` is a CLI tool designed to take a single PDF file and extract every page into its own separate file. It is built with safety and simplicity in mind, leveraging the power of [lopdf](https://github.com/J-F-Liu/lopdf) to handle complex PDF structures reliably.

Whether you are processing invoices, scanning archives, or breaking down ebooks, `pdf-splitter` provides a fast and reliable way to decompose your documents.

## ✨ Features

- **Simple CLI Interface**: Easy-to-use command line arguments powered by `clap`.
- **Automatic Output Management**: Automatically creates output directories if they don't exist.
- **Robust Processing**: Handles complex PDF objects by preserving document context during split operations.
- **Optimization**: Includes an optimization step (`prune_objects` and `renumber_objects`) to keep resulting file sizes reasonable.

## 🚀 Installation

Ensure you have [Rust and Cargo installed](https://rustup.rs/) on your system.

### From Source

Clone the repository and build using Cargo:

```bash
git clone https://github.com/yourusername/pdf-splitter.git
cd pdf-splitter
cargo install --path .
```

Or run it directly without installing:

```bash
cargo run --release -- --input document.pdf
```

## 💡 Usage

The basic usage requires an input file path. By default, pages will be saved to an `output_pages` directory.

```bash
pdf-splitter --input documents/report.pdf
```

### Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--input` | `-i` | Path to the source PDF file | **Required** |
| `--output` | `-o` | Directory to save extracted pages | `output_pages` |
| `--help` | `-h` | Print help information | |
| `--version` | `-V` | Print version information | |

### Examples

**Split a file to a custom directory:**

```bash
pdf-splitter -i tax_return_2023.pdf -o ./split_files
```

**Help Output:**

```text
Usage: pdf-splitter [OPTIONS] --input <FILE>

Options:
  -i, --input <FILE>  Path to the input PDF file to be split. Example: /path/to/document.pdf
  -o, --output <DIR>  Path to the output directory where split pages will be saved.
                      Defaults to "output_pages" in the current working directory.
                      Example: /path/to/output/ [default: output_pages]
  -h, --help          Print help
  -V, --version       Print version
```

## 🏗 Design Decisions

### The "Clone & Prune" Strategy

PDF is a complex format where pages often share resources (fonts, images, content streams) defined globally in the document. Simply extracting a page's content stream into a new blank document often results in missing fonts or broken images.

To ensure 100% fidelity:
1. **Load**: We load the entire source document into memory.
2. **Clone**: For *each* page extraction, we clone the entire document structure.
3. **Delete**: We remove all references to pages *other* than the target page.
4. **Prune**: We run a garbage collection pass (`prune_objects`) to remove objects that are no longer referenced by the remaining page.

**Why?**
This approach guarantees that any shared resource required by the page is preserved. While purely copying objects to a new document is theoretically more efficient, it is significantly more error-prone due to the interconnected nature of PDF reference graphs. The "Clone & Prune" method trades some CPU cycles for correctness.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is open source and available under the MIT License.
