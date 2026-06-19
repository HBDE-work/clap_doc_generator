mod attr_parser;
mod command_tree;
mod extraction;
mod model_parsed;
mod render_types;
mod source_analysis;
mod type_helper;
mod utils;

pub mod model_command;

use std::path::Path;

use self::source_analysis::analyze_project_source;
use self::utils::read_binary_name;

#[cfg(feature = "markdown")]
pub fn generate_docs(
    project_path: &Path,
    readme_path: &Path,
    start_marker: &str,
    end_marker: &str,
) -> Result<(), String> {
    let binary_name = read_binary_name(project_path)?;
    let command_tree = analyze_project_source(project_path, &binary_name)?;
    let markdown = render_types::markdown::render(&command_tree);

    utils::update_readme(readme_path, &markdown, start_marker, end_marker)
}

#[cfg(feature = "jenkins")]
pub fn generate_jenkins(
    project_path: &Path,
    output_dir: &Path,
    package_name: &str,
    json_output: bool,
) -> Result<(), String> {
    let binary_name = read_binary_name(project_path)?;
    let command_tree = analyze_project_source(project_path, &binary_name)?;

    render_types::jenkins_library::render(&command_tree, output_dir, package_name, json_output)
}
