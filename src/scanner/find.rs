use ignore::WalkBuilder;
use std::fs;
use std::path::Path;

use super::datamodel::CargoToml;
use super::datamodel::Target;

/// Scan `root_dir` for files whose name matches `readme_name`, then resolve
/// each match to its owning clap project by walking up to find `Cargo.toml`.
pub fn find_targets(
    root_dir: &Path,
    recursive: bool,
    readme_name: &str,
) -> Result<Vec<Target>, String> {
    let readme_name_lower = readme_name.to_lowercase();

    let mut walker = WalkBuilder::new(root_dir);
    walker.standard_filters(true).follow_links(true);

    if !recursive {
        walker.max_depth(Some(1));
    }

    let mut targets = Vec::new();

    for entry in walker.build().flatten() {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();

        if file_name.to_lowercase() != readme_name_lower {
            continue;
        }

        let readme_path = path.to_path_buf();

        match find_clap_project(&readme_path) {
            Some(target) => targets.push(target),
            None => {
                eprintln!(
                    "  (!) {} has no parent clap project, skipping",
                    readme_path.display()
                );
            }
        }
    }

    if targets.is_empty() {
        return Err(format!(
            "No files named '{readme_name}' with a parent clap project found"
        ));
    }

    Ok(targets)
}

/// Walk upwards from the readme's directory to find a `Cargo.toml` that
/// declares clap as a dependency.
fn find_clap_project(readme_path: &Path) -> Option<Target> {
    let mut search_dir = readme_path.parent()?;

    loop {
        let cargo_path = search_dir.join("Cargo.toml");

        if cargo_path.is_file()
            && let Some(target) = try_as_clap_project(search_dir, readme_path)
        {
            return Some(target);
        }

        search_dir = search_dir.parent()?;
    }
}

fn try_as_clap_project(project_dir: &Path, readme_path: &Path) -> Option<Target> {
    let cargo_path = project_dir.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_path).ok()?;
    let cargo_toml: CargoToml = toml::from_str(&content).ok()?;

    let package_name = cargo_toml.package.as_ref()?.name.clone();

    let has_clap = cargo_toml
        .dependencies
        .as_ref()
        .and_then(|deps| deps.as_table())
        .is_some_and(|deps| deps.contains_key("clap"));

    if !has_clap {
        return None;
    }

    Some(Target {
        readme_path: readme_path.to_path_buf(),
        project_path: project_dir.to_path_buf(),
        name: package_name,
    })
}
