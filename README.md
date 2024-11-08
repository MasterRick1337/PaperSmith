# PaperSmith

_A free and open-source writing application for authors_

### Table of contents

- [Status](#status)
- [To-Do](#to-do)
- [Roadmap](#roadmap)
  - [Done](#done)
  - [Planned](#planned)
  - [Future Ideas](#future-ideas)
- [Installation](#installation)
- [Build from source](#build-from-source)

### Status

This project is in a very early stage of development and currently NOT ready for usage.

### To-Do

- MasterRick1337: Writing features & Markdown Compiler
- Toll25: Project Explorer & Miscellaneous Stuff
- DotDo1: Statistic Window
- Alllpacka: Markdown Compiler, Settings & CI/CD

### Roadmap

Features that are either already finished, are in progress or are planned for future development. It also includes ideas for later implementation.

#### Done

- [x] Split-View
- [x] Project Explorer
- [x] Load Project
- [x] Statistics
- [x] Project creation wizard
- [x] Markdown Formatting

#### Planned

- [ ] Autosaving
- [ ] Automatic Backups
- [ ] Settings menu
- [ ] Export options
- [ ] Spellcheck
- [ ] Saving files
- [ ] Statistics Window
- [ ] Single-View

#### Future ideas

- [ ] Multiple open documents
- [ ] Page-full layout
- [ ] Grammar check

### Installation

Get the binary from the Github releases.

Not yet packaged anywhere.

### Build from source

1. Install rust
2. Add `wasm32-unknown-unknown` target to rust
3. Install tauri dependencies
4. Install tauri-cli (Version < 2.0)
5. Install trunk
6. Clone repo & cd into it
7. Run `cargo tauri dev`
