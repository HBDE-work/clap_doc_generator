# Clap Documentation Generator

Extract clap CLI definitions from Rust source code and write them as a markdown table into readme files.

---

## Features

- Parses `#[derive(Parser)]`, `#[derive(Args)]`, `#[derive(Subcommand)]` and `#[derive(ValueEnum)]` directly from source — no build required
- Generates clean markdown tables with options, arguments, defaults, and possible values
- Handles nested subcommands, `#[command(flatten)]`, and `#[command(subcommand)]`
- Writes output between configurable markers in your readme
- Supports recursive directory scanning for monorepos

---

## Requirements

- The target project must use clap derive macros
- The readme must contain the start/end marker comments

---

## Installation

### Install directly from crates-io

`cargo install clap_doc_generator`

This installs the binary as `clapdocs` in your Cargo bin directory

---

## CLI Reference

<!-- CLAP_DOC_GEN_START -->
**Usage:** `clapdocs [OPTIONS]`

Extract clap CLI definitions from Rust source code and write them as a markdown table into readme files

#### Options

| Options | Description | Default |
|------|------|------|
| `-d, --directory <DIRECTORY>` | The directory to scan for Rust projects with clap | `.` |
| `-r, --recursive` | Recursively scan subdirectories for projects |  |
| `--name <NAME>` | The name of the readme file to update | `readme.md` |
| `--start-marker <START-MARKER>` | Marker for the start of the generated section | `<!-- CLAP_DOC_GEN_START -->` |
| `--end-marker <END-MARKER>` | Marker for the end of the generated section | `<!-- CLAP_DOC_GEN_END -->` |

<!-- CLAP_DOC_GEN_END -->

---
