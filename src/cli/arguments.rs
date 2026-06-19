use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(
    name = "clap_doc_generator",
    version,
    about = "Extract clap CLI definitions from Rust source code and generate documentation or language bindings"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    /// The directory to scan for Rust projects with clap
    #[arg(short, long, global = true, default_value = ".")]
    pub directory: String,

    /// Recursively scan subdirectories for projects
    #[arg(short, long, global = true)]
    pub recursive: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[cfg(feature = "markdown")]
    /// Create clap reference in markdown syntax
    Markdown {
        /// The name of the readme file to update
        #[arg(long, default_value = "readme.md")]
        name: String,

        /// Marker for the start of the generated section
        #[arg(long, default_value = "<!-- CLAP_DOC_GEN_START -->")]
        start_marker: String,

        /// Marker for the end of the generated section
        #[arg(long, default_value = "<!-- CLAP_DOC_GEN_END -->")]
        end_marker: String,
    },

    #[cfg(feature = "jenkins")]
    /// Create rust binary cli wrapper for jenkins libraries
    Jenkins {
        /// Output directory for generated files
        #[arg(short, long)]
        output_dir: Option<String>,

        /// Package path prefix for generated Groovy classes
        #[arg(long, default_value = "groovypackage")]
        package_name: String,

        /// Assume all commands emit JSON to stdout
        #[arg(long)]
        json_output: bool,
    },
}
