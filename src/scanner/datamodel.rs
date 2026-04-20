use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Target {
    /// The readme file to update.
    pub readme_path: PathBuf,
    /// The project root (directory containing the Cargo.toml with clap).
    pub project_path: PathBuf,
    /// The project name from Cargo.toml.
    pub name: String,
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
