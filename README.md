# PDF Splitter

> A fast, beautiful macOS desktop app that splits any multi-page PDF into individual page files — built with Tauri 2 and Vue 3.

![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)
![Tauri](https://img.shields.io/badge/Tauri-2.x-24C8D8.svg)
![Vue](https://img.shields.io/badge/Vue-3.x-42b883.svg)
![TypeScript](https://img.shields.io/badge/TypeScript-5.x-3178C6.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg)

---

## Overview

**PDF Splitter** is a production-grade macOS desktop application that takes any multi-page PDF document and extracts every page into its own individual PDF file.

The app is built on a native [Tauri 2](https://tauri.app/) shell with a [Vue 3](https://vuejs.org/) + TypeScript renderer, giving you:

- **Native macOS performance** — a lean Rust binary with zero Electron overhead.
- **Parallel page processing** — [Rayon](https://github.com/rayon-rs/rayon) distributes page extraction across all CPU cores.
- **Live progress feedback** — real-time per-page progress streamed from Rust to the UI via Tauri events.
- **Beautiful, native-feel UI** — glassmorphism design with macOS system colours, SF Pro typography, dark-mode support, and spring-feel animations.
- **Drag & drop** — drop a PDF straight onto the window and hit **Split PDF**.

---

## Screenshots

```
┌──────────────────────────────────────────┐
│  ● ○ ○   PDF Splitter                   │
├──────────────────────────────────────────┤
│                                          │
│  📄 PDF Splitter                         │
│  Extract every page into its own PDF     │
│                                          │
│  ┌─────────────────────────────────┐     │
│  │                                 │     │
│  │   Drop your PDF here            │     │
│  │   or click to browse files      │     │
│  │                                 │     │
│  │        [ + Select PDF ]         │     │
│  └─────────────────────────────────┘     │
│                                          │
│  Supports all PDF versions · Any pages   │
└──────────────────────────────────────────┘
```

---

## Features

| Feature                     | Details                                                                                                                                  |
| --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| **Clone & Prune splitting** | Clones the full document per page then prunes unreachable objects — guarantees 100 % fidelity for fonts, images, and embedded resources. |
| **Parallel processing**     | Rayon worker pool; scales to all available CPU cores automatically.                                                                      |
| **Real-time progress**      | Per-page `split://progress` Tauri events drive an animated progress bar in the UI.                                                       |
| **Native file dialogs**     | macOS file-picker and folder-picker via `tauri-plugin-dialog`.                                                                           |
| **Reveal in Finder**        | "Show in Finder" opens the output folder with the generated files selected.                                                              |
| **Dark mode**               | Full `prefers-color-scheme` support with a dedicated dark-mode token set.                                                                |
| **Drag & drop**             | Drop a `.pdf` file directly onto the app window.                                                                                         |
| **macOS DMG installer**     | `tauri build` produces a ready-to-distribute `.dmg` for Apple Silicon and Intel.                                                         |
| **Strict Rust**             | `clippy::pedantic` + `clippy::nursery` lints enforced; `unsafe_code` forbidden.                                                          |

---

## Tech Stack

### Backend — Rust / Tauri 2

| Crate                                                                                           | Role                                         |
| ----------------------------------------------------------------------------------------------- | -------------------------------------------- |
| [`tauri`](https://crates.io/crates/tauri) `2.x`                                                 | macOS native shell, IPC bus, event system    |
| [`tauri-plugin-dialog`](https://crates.io/crates/tauri-plugin-dialog) `2.x`                     | Native file / folder picker dialogs          |
| [`tauri-plugin-opener`](https://crates.io/crates/tauri-plugin-opener) `2.x`                     | Reveal output folder in Finder               |
| [`lopdf`](https://crates.io/crates/lopdf) `0.39`                                                | PDF parsing, page extraction, object pruning |
| [`rayon`](https://crates.io/crates/rayon) `1.x`                                                 | Data-parallel page processing                |
| [`thiserror`](https://crates.io/crates/thiserror) `2.x`                                         | Ergonomic, structured error types            |
| [`serde`](https://crates.io/crates/serde) + [`serde_json`](https://crates.io/crates/serde_json) | IPC serialisation                            |

### Frontend — Vue 3 / TypeScript

| Package                                                                  | Role                                      |
| ------------------------------------------------------------------------ | ----------------------------------------- |
| [`vue`](https://www.npmjs.com/package/vue) `3.x`                         | Reactive UI framework (Composition API)   |
| [`@tauri-apps/api`](https://www.npmjs.com/package/@tauri-apps/api) `2.x` | Tauri IPC (`invoke`, `listen`)            |
| [`vite`](https://www.npmjs.com/package/vite) `6.x`                       | Lightning-fast dev server and build tool  |
| [`vue-tsc`](https://www.npmjs.com/package/vue-tsc)                       | TypeScript type checking for `.vue` files |

---

## Architecture

### Application State Machine

```
idle ──pick/drop──► ready ──split──► processing ──► complete
 ▲                    │                   │              │
 │                    │                  error           │
 │                    └──────────────────► error         │
 └──────────────────────────────────── reset ────────────┘
```

### Tauri IPC Contract

#### Commands

| Command            | Signature                                   | Description                              |
| ------------------ | ------------------------------------------- | ---------------------------------------- |
| `get_page_count`   | `(path: string) → u32`                      | Count pages without splitting            |
| `get_file_info`    | `(path: string) → { pageCount, sizeBytes }` | Page count + file size in one round-trip |
| `pick_pdf_file`    | `() → string \| null`                       | Open native PDF file picker              |
| `pick_output_dir`  | `() → string \| null`                       | Open native folder picker                |
| `split_pdf`        | `(inputPath, outputDir) → SplitResult`      | Split with live progress events          |
| `reveal_in_finder` | `(path: string) → void`                     | Reveal path in macOS Finder              |

#### Events

| Event              | Payload                        | When                       |
| ------------------ | ------------------------------ | -------------------------- |
| `split://progress` | `{ current, total, fileName }` | After each page is written |
| `split://complete` | `SplitResult`                  | When all pages are done    |

### PDF Splitting Strategy — Clone & Prune

PDF documents are deeply interconnected: fonts, images, and content streams are typically defined once and shared across many pages. The safest approach for extracting a single page while preserving 100 % fidelity is:

1. **Load** — parse the source document into memory _once_.
2. **Clone** — for each target page, clone the entire in-memory document.
3. **Delete** — remove every page reference that is _not_ the target.
4. **Prune** — run `lopdf::Document::prune_objects` to garbage-collect every object no longer reachable from the surviving page.
5. **Renumber** — compact object IDs with `renumber_objects` to minimise output file size.
6. **Save** — write the single-page document to disk.

Steps 2–6 run **concurrently across all CPU cores** via Rayon. A 100-page document is processed in roughly `⌈100 / num_cpus⌉` sequential page-times rather than `100 × t_page`.

---

## Project Structure

```
pdf-splitter/
├── src-tauri/                   # Rust / Tauri backend
│   ├── src/
│   │   ├── lib.rs               # App bootstrap, plugin & command registration
│   │   ├── main.rs              # Binary entry point (calls lib::run)
│   │   ├── commands.rs          # Tauri command handlers (thin IPC wrappers)
│   │   └── pdf/
│   │       ├── mod.rs           # Public re-exports
│   │       ├── splitter.rs      # Core Clone & Prune logic + Rayon parallelism
│   │       └── error.rs         # PdfError type with Serialize impl
│   ├── capabilities/
│   │   └── default.json         # Tauri permission grants
│   ├── icons/                   # App icons (all sizes, .icns, .ico)
│   ├── Cargo.toml
│   ├── build.rs
│   └── tauri.conf.json          # Window config + macOS DMG bundle settings
│
├── src/                         # Vue 3 + TypeScript frontend
│   ├── main.ts                  # Vue app entry point
│   ├── App.vue                  # Root component — state machine orchestrator
│   ├── components/
│   │   ├── DropZone.vue         # Idle state: drag & drop / file picker
│   │   ├── FileCard.vue         # Ready state: file info + split button
│   │   ├── ProgressView.vue     # Processing state: live progress bar
│   │   ├── ResultView.vue       # Complete state: file list + Finder button
│   │   └── ErrorView.vue        # Error state: message + retry action
│   ├── composables/
│   │   └── usePdfSplitter.ts    # All Tauri IPC + application state
│   ├── types/
│   │   └── index.ts             # Shared TypeScript types & utility functions
│   └── assets/styles/
│       └── main.css             # macOS design system (tokens, components, animations)
│
├── _legacy/cli/                 # Original CLI tool (archived for reference)
├── index.html                   # Vite HTML entry
├── vite.config.ts
├── tsconfig.json
├── tsconfig.node.json
└── package.json
```

---

## Getting Started

### Prerequisites

| Tool                     | Version | Install                           |
| ------------------------ | ------- | --------------------------------- |
| Rust + Cargo             | ≥ 1.80  | [rustup.rs](https://rustup.rs/)   |
| Node.js                  | ≥ 18    | [nodejs.org](https://nodejs.org/) |
| npm                      | ≥ 9     | Bundled with Node.js              |
| Xcode Command Line Tools | Latest  | `xcode-select --install`          |

### Development

```bash
# 1. Clone the repository
git clone https://github.com/yourusername/pdf-splitter.git
cd pdf-splitter

# 2. Install frontend dependencies
npm install

# 3. Start the development server (hot-reload UI + Rust rebuild on change)
npm run tauri:dev
```

The app window opens automatically. Changes to Vue files reload instantly; changes to Rust files trigger a Cargo recompile.

### Production Build

```bash
# Build the optimised binary + macOS app bundle + DMG installer
npm run tauri:build
```

Output artefacts:

| Artefact               | Path                                                                     |
| ---------------------- | ------------------------------------------------------------------------ |
| macOS `.app` bundle    | `src-tauri/target/release/bundle/macos/PDF Splitter.app`                 |
| macOS `.dmg` installer | `src-tauri/target/release/bundle/dmg/PDF Splitter_<version>_aarch64.dmg` |

### Installing the DMG

1. Open `PDF Splitter_1.0.0_aarch64.dmg`.
2. Drag **PDF Splitter.app** to your **Applications** folder.
3. Launch from Spotlight or Launchpad.

> **Gatekeeper note:** Because the app is not notarised, macOS may show a security prompt on first launch. Right-click the `.app` → **Open** → **Open** to allow it.

---

## Quality Checks

```bash
# Run all Rust unit tests (22 tests across splitter, error, and command modules)
cd src-tauri && cargo test

# Clippy — pedantic + nursery lints, zero warnings tolerated
cd src-tauri && cargo clippy --all-targets -- -D warnings

# Rustfmt — canonical code style
cd src-tauri && cargo fmt --all --check

# TypeScript type checking
npm run type-check

# Frontend production build (includes type check)
npm run build
```

---

## Contributing

Contributions are welcome! Please follow the conventions below:

1. **Fork** the repository and create a feature branch:
   ```bash
   git checkout -b feature/my-improvement
   ```
2. **Rust**: run `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test` before committing.
3. **TypeScript / Vue**: run `npm run type-check` and `npm run build`.
4. **Commit messages**: use [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).
5. Open a **Pull Request** with a clear description of what changed and why.

### Adding a New Tauri Command

1. Implement the function in `src-tauri/src/commands.rs` with `#[tauri::command]`.
2. Register it in `src-tauri/src/lib.rs` inside `tauri::generate_handler![…]`.
3. Add the matching `invoke<ReturnType>('command_name', { … })` call in `src/composables/usePdfSplitter.ts`.
4. Add the TypeScript type for the response in `src/types/index.ts`.
5. Grant any required permissions in `src-tauri/capabilities/default.json`.

---

## Roadmap

- [ ] **Drag-and-drop path loading** — wire the `drop` event in `DropZone.vue` directly to a `load_pdf_from_path` command (avoids re-opening the picker).
- [ ] **Page range selection** — choose which pages to extract rather than all pages.
- [ ] **PDF merging** — companion feature to re-combine split pages.
- [ ] **Notarisation & signing** — Apple Developer ID code-signing for seamless Gatekeeper approval.
- [ ] **Auto-updater** — Tauri's built-in `tauri-plugin-updater` for OTA updates.
- [ ] **Progress cancel** — abort a running split mid-operation.

---

## License

This project is open source and available under the [MIT License](LICENSE).

---

## Acknowledgements

- [lopdf](https://github.com/J-F-Liu/lopdf) — the excellent Rust PDF library that powers the splitting engine.
- [Tauri](https://tauri.app/) — for making lightweight, native desktop apps possible with web technologies.
- [Rayon](https://github.com/rayon-rs/rayon) — fearless data parallelism in Rust.
