# Clap Document Generator

Extract clap CLI definitions from Rust source code and generate a command line reference in markdown tables or thin-wrapper for other code languages.

## Requirements

- The target projects must use clap derive macros

## Based on clap

- Parses `#[derive(Parser)]`, `#[derive(Args)]`, `#[derive(Subcommand)]` and `#[derive(ValueEnum)]` directly from source
- Handles nested subcommands, `#[command(flatten)]`, and `#[command(subcommand)]`


## Features

### Markdown (default)

- Generates markdown tables with options, arguments, defaults, and possible values
- Writes output between configurable markers in your readme
- Supports recursive directory scanning for monorepos

### Jenkins

- generate a Jenkins compatible thin wrapper to invoke rust cli from Jenkins
- export files for /src and /vars

## Installation

### Install directly from crates-io

`cargo install clap_doc_generator`

This installs the binary as `clapdocs` in your Cargo bin directory

## CLI Reference

<!-- CLAP_DOC_GEN_START -->
**Usage:** `clapdocs [OPTIONS] <COMMAND>`

Extract clap CLI definitions from Rust source code and generate documentation or language bindings

#### Options

| Options | Description |
|------|------|
| `-d, --directory <DIRECTORY>` | The directory to scan for Rust projects with clap |
| `-r, --recursive` | Recursively scan subdirectories for projects |

#### Commands

| Command | Description |
|---------|-------------|
| `markdown` | Create clap reference in markdown syntax |
| `jenkins` | Create rust binary cli wrapperfor jenkins libraries |

### `clapdocs markdown`

Create clap reference in markdown syntax

#### Options

| Options | Description | Default |
|------|------|------|
| `--name <NAME>` | The name of the readme file to update | `readme.md` |
| `--start-marker <START-MARKER>` | Marker for the start of the generated section | `<!-- CLAP_DOC_GEN_START -->` |
| `--end-marker <END-MARKER>` | Marker for the end of the generated section | `<!-- CLAP_DOC_GEN_END -->` |

### `clapdocs jenkins`

Create rust binary cli wrapperfor jenkins libraries

#### Options

| Options | Description | Default |
|------|------|------|
| `-o, --output-dir <OUTPUT-DIR>` | Output directory for generated files |  |
| `--package-name <PACKAGE-NAME>` | Package path prefix for generated Groovy classes | `groovypackage` |
| `--json-output` | Assume all commands emit JSON to stdout |  |

<!-- CLAP_DOC_GEN_END -->
