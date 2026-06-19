use std::fs;
use std::path::Path;

use super::command_tree::build_command_info;
use super::extraction::extract_items;
use super::model_command::CommandInfo;
use super::model_parsed::ParsedSource;

pub fn analyze_project_source(
    project_path: &Path,
    binary_name: &str,
) -> Result<CommandInfo, String> {
    let source_files = collect_rust_sources(project_path)?;
    let mut parsed = ParsedSource::default();

    for file_path in &source_files {
        let content = fs::read_to_string(file_path)
            .map_err(|err| format!("Failed to read {}: {err}", file_path.display()))?;

        let syntax = syn::parse_file(&content)
            .map_err(|err| format!("Failed to parse {}: {err}", file_path.display()))?;

        extract_items(&syntax.items, &mut parsed);
    }

    let root_struct = parsed
        .arg_structs
        .iter()
        .find(|s| s.command_attr.name.is_some())
        .or_else(|| parsed.arg_structs.first())
        .ok_or_else(|| "No struct deriving Parser or Args found in project".to_string())?
        .clone();

    Ok(build_command_info(binary_name, &root_struct, &parsed))
}

fn collect_rust_sources(project_path: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let src_dir = project_path.join("src");
    if !src_dir.exists() {
        return Err(format!(
            "No src/ directory found in {}",
            project_path.display()
        ));
    }

    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files)
        .map_err(|err| format!("Failed to scan source files: {err}"))?;

    if files.is_empty() {
        return Err("No .rs files found in src/".to_string());
    }

    Ok(files)
}

fn collect_rs_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_dir() {
            collect_rs_files(&path, files)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    Ok(())
}
