# pdf-splitter

A simple, fast command-line tool to split PDF documents into individual pages.

## Features

- 📄 Splits a multi-page PDF into separate single-page PDF files
- 🚀 Fast processing using the [lopdf](https://crates.io/crates/lopdf) library
- 🧹 Automatically prunes unused objects for smaller output files
- 📁 Organizes output with zero-padded filenames (e.g., `page_001.pdf`)

## Installation

### From Source

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed, then:

```bash
git clone https://github.com/pharmacist-sabot/pdf-splitter.git
cd pdf-splitter
cargo build --release
```

The binary will be available at `target/release/pdf-splitter`.

## Usage

1. Create an `input` directory in the project root:

   ```bash
   mkdir input
   ```

2. Place your PDF file in the `input` directory and rename it to `sample.pdf`:

   ```bash
   cp /path/to/your/file.pdf input/sample.pdf
   ```

3. Run the splitter:

   ```bash
   cargo run --release
   ```

4. Find your split pages in the `output_pages` directory.

### Example Output

```
Starting to split: input/sample.pdf
Processing page 1/10...
Processing page 2/10...
Processing page 3/10...
...
✅ Successfully split 10 pages into 'output_pages'
```

## Output Structure

```
output_pages/
├── page_001.pdf
├── page_002.pdf
├── page_003.pdf
└── ...
```

## Dependencies

| Crate | Version | Description |
|-------|---------|-------------|
| [lopdf](https://crates.io/crates/lopdf) | 0.39.0 | PDF document manipulation library |

## Requirements

- Rust 2024 Edition
- Input PDF file at `input/sample.pdf`

## License

This project is open source. See the repository for license details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
