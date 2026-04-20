use std::fs;
use std::path::Path;

pub fn read_binary_name(project_path: &Path) -> Result<String, String> {
    let cargo_toml_path = project_path.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_toml_path)
        .map_err(|err| format!("Failed to read Cargo.toml: {err}"))?;

    let toml: toml::Value =
        toml::from_str(&content).map_err(|err| format!("Failed to parse Cargo.toml: {err}"))?;

    if let Some(bins) = toml.get("bin").and_then(|b| b.as_array())
        && let Some(name) = bins
            .first()
            .and_then(|b| b.get("name"))
            .and_then(|n| n.as_str())
    {
        return Ok(name.to_string());
    }

    toml.get("package")
        .and_then(|p| p.get("name"))
        .and_then(|n| n.as_str())
        .map(ToString::to_string)
        .ok_or_else(|| "Could not find package name in Cargo.toml".to_string())
}

pub fn update_readme(
    readme_path: &Path,
    new_docs: &str,
    start_marker: &str,
    end_marker: &str,
) -> Result<(), String> {
    let content =
        fs::read_to_string(readme_path).map_err(|err| format!("Failed to read readme: {err}"))?;

    let marker_start = content
        .find(start_marker)
        .ok_or_else(|| format!("Could not find start marker '{start_marker}' in readme"))?;

    let insert_start = content[marker_start..]
        .find('\n')
        .map(|pos| marker_start + pos + 1)
        .ok_or_else(|| "Could not find newline after start marker".to_string())?;

    let insert_end = find_line_start_marker(&content[insert_start..], end_marker)
        .map(|pos| insert_start + pos)
        .ok_or_else(|| format!("Could not find end marker '{end_marker}' in readme"))?;

    let mut new_content = String::with_capacity(content.len() + new_docs.len());
    new_content.push_str(&content[..insert_start]);

    let trimmed_docs = new_docs.trim_end();
    if !trimmed_docs.is_empty() {
        new_content.push_str(trimmed_docs);
    }
    new_content.push_str("\n\n");

    new_content.push_str(&content[insert_end..]);

    fs::write(readme_path, new_content).map_err(|err| format!("Failed to write readme: {err}"))?;

    Ok(())
}

pub fn first_sentence(text: &str) -> String {
    let trimmed = text.trim();
    if let Some(dot_pos) = trimmed.find(". ") {
        trimmed[..=dot_pos].to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn to_kebab_case(name: &str) -> String {
    let mut result = String::with_capacity(name.len() + 4);
    for (index, ch) in name.chars().enumerate() {
        if ch.is_uppercase() {
            if index > 0 {
                result.push('-');
            }
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        } else {
            result.push(ch);
        }
    }
    result
}

/// Find `marker` only when it appears at the start of a line within `text`.
fn find_line_start_marker(text: &str, marker: &str) -> Option<usize> {
    // Check if the text starts with the marker (position 0 = start of line).
    if text.starts_with(marker) {
        return Some(0);
    }

    // Otherwise look for \nMARKER.
    let needle = format!("\n{marker}");
    text.find(&needle).map(|pos| pos + 1)
}
