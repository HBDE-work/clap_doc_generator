use std::path::Path;

pub struct MarkdownConfig<'cfg> {
    pub project_path: &'cfg Path,
    pub readme_path: &'cfg Path,
    pub start_marker: &'cfg str,
    pub end_marker: &'cfg str,
}
