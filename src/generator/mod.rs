mod attr_parser;
mod command_tree;
mod extraction;
mod model_command;
mod model_parsed;
mod render_markdown;
mod source_analysis;
mod type_helper;
mod utils;

use std::path::Path;

use self::render_markdown::render_markdown;
use self::source_analysis::analyze_project_source;
use self::utils::read_binary_name;
use self::utils::update_readme;

pub fn generate_docs(
    project_path: &Path,
    readme_path: &Path,
    start_marker: &str,
    end_marker: &str,
) -> Result<(), String> {
    let binary_name = read_binary_name(project_path)?;
    let command_tree = analyze_project_source(project_path, &binary_name)?;
    let markdown = render_markdown(&command_tree);

    update_readme(readme_path, &markdown, start_marker, end_marker)
}
