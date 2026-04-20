use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "clap_doc_generator",
    version,
    about = "Extract clap CLI definitions from Rust source code and write them as a markdown table into readme files"
)]
pub struct Args {
    /// The directory to scan for Rust projects with clap
    #[arg(short, long, default_value = ".")]
    pub directory: String,

    /// Recursively scan subdirectories for projects
    #[arg(short, long)]
    pub recursive: bool,

    /// The name of the readme file to update
    #[arg(long, default_value = "readme.md")]
    pub name: String,

    /// Marker for the start of the generated section
    #[arg(long, default_value = "<!-- CLAP_DOC_GEN_START -->")]
    pub start_marker: String,

    /// Marker for the end of the generated section
    #[arg(long, default_value = "<!-- CLAP_DOC_GEN_END -->")]
    pub end_marker: String,
}
