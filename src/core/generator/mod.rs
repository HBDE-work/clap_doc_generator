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

use self::model_command::CommandInfo;
use self::source_analysis::analyze_project_source;
use self::utils::read_binary_name;

#[cfg(feature = "jenkins")]
pub use self::render_types::jenkins_library::JenkinsLibraryConfig;
#[cfg(feature = "markdown")]
pub use self::render_types::markdown::MarkdownConfig;

#[cfg(feature = "markdown")]
pub fn generate_docs(markdown_config: MarkdownConfig) -> Result<(), String> {
    let project_path = markdown_config.project_path;
    let readme_path = markdown_config.readme_path;
    let start_marker = markdown_config.start_marker;
    let end_marker = markdown_config.end_marker;

    let command_tree = read_source(project_path)?;

    let markdown = render_types::markdown::render(&command_tree);
    utils::update_readme(readme_path, &markdown, start_marker, end_marker)
}

#[cfg(feature = "jenkins")]
pub fn generate_jenkins(jenkins_config: JenkinsLibraryConfig) -> Result<(), String> {
    let project_path = jenkins_config.project_path;
    let output_dir = jenkins_config.output_dir;
    let package_name = jenkins_config.package_name;
    let execution_model = jenkins_config.execution_model;
    let json_output = jenkins_config.json_output;
    let stubs = jenkins_config.stubs;

    let command_tree = read_source(project_path)?;

    render_types::jenkins_library::render(
        &command_tree,
        output_dir,
        package_name,
        execution_model,
        json_output,
        stubs,
    )
}

fn read_source(project_path: &Path) -> Result<CommandInfo, String> {
    let binary_name = read_binary_name(project_path)?;

    analyze_project_source(project_path, &binary_name)
}
