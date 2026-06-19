use serde::Deserialize;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Target {
    pub readme_path: Option<PathBuf>,
    pub project_path: PathBuf,
    pub name: String,
}

pub struct ScanOptions<'a> {
    pub root_dir: &'a Path,
    pub recursive: bool,
    pub readme_name: Option<&'a str>,
}

#[derive(Deserialize)]
pub struct CargoToml {
    pub package: Option<Package>,
    pub dependencies: Option<toml::Value>,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
}
