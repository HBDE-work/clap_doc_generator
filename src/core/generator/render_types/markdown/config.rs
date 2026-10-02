use std::path::Path;

pub struct MarkdownConfig<'cfg> {
    pub project_path: &'cfg Path,
    pub readme_path: &'cfg Path,
    pub start_marker: &'cfg str,
    pub end_marker: &'cfg str,
}

impl<'cfg> MarkdownConfig<'cfg> {
    pub fn from(
        project_path: &'cfg Path,
        readme_path: &'cfg Path,
        start_marker: &'cfg str,
        end_marker: &'cfg str,
    ) -> MarkdownConfig<'cfg> {
        MarkdownConfig {
            project_path,
            readme_path,
            start_marker,
            end_marker,
        }
    }
}
