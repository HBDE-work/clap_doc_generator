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

| Options                       | Description                                       |
| ----------------------------- | ------------------------------------------------- |
| `-d, --directory <DIRECTORY>` | The directory to scan for Rust projects with clap |
| `-r, --recursive`             | Recursively scan subdirectories for projects      |

#### Commands

| Command    | Description                                          |
| ---------- | ---------------------------------------------------- |
| `markdown` | Create clap reference in markdown syntax             |
| `jenkins`  | Create rust binary cli wrapper for jenkins libraries |

### `clapdocs markdown`

Create clap reference in markdown syntax

#### Options

| Options                         | Description                                   | Default                       |
| ------------------------------- | --------------------------------------------- | ----------------------------- |
| `--name <NAME>`                 | The name of the readme file to update         | `readme.md`                   |
| `--start-marker <START-MARKER>` | Marker for the start of the generated section | `<!-- CLAP_DOC_GEN_START -->` |
| `--end-marker <END-MARKER>`     | Marker for the end of the generated section   | `<!-- CLAP_DOC_GEN_END -->`   |

### `clapdocs jenkins`

Create rust binary cli wrapper for jenkins libraries

#### Options

| Options                               | Description                                                         | Default         | Values                   |
| ------------------------------------- | ------------------------------------------------------------------- | --------------- | ------------------------ |
| `-o, --output-dir <OUTPUT-DIR>`       | Output directory for generated files                                |                 |                          |
| `--package-name <PACKAGE-NAME>`       | Package path prefix for generated Groovy classes                    | `groovypackage` |                          |
| `--execution-model <EXECUTION-MODEL>` | Execution model used by generated code to invoke the wrapped binary | `sh`            | `sh`, `ps`, `bat`, `jvm` |
| `--json-output`                       | Assume all commands emit JSON to stdout                             |                 |                          |

<!-- CLAP_DOC_GEN_END -->

## Library Usage

**usage within build.rs:**

the ideal case is that the binary will always create valid json output

```toml
# Cargo.toml
[build-dependencies]
clap_doc_generator = { version = "0.3.1", features = ["markdown", "jenkins"]}
```

```rust
// build.rs
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let toml_path = PathBuf::from(env!("CARGO_MANIFEST_PATH"));
    let project_path = toml_path
        .parent()
        .expect("Cannot establish manifest directory");
    let out_dir = project_path.join("groovy_binding");
    let pkg_name = env!("CARGO_PKG_NAME");
    let execution_model = clapdocs::ExecutionModel::Jvm;

    clapdocs::generate_docs(
        project_path,
        project_path.join("readme.md").as_path(),
        "<!-- CLAP_DOC_GEN_START -->",
        "<!-- CLAP_DOC_GEN_END -->",
    )?;
    clapdocs::generate_jenkins(
        project_path,
        out_dir.as_path(),
        pkg_name,
        execution_model,
        true,
    )
}
```
