pub mod generator;
pub mod scanner;

pub use self::scanner::find::ScanOptions;
pub use self::scanner::find::Target;
pub use self::scanner::find::find_projects;

#[cfg(feature = "markdown")]
pub use self::generator::generate_docs;
#[cfg(feature = "jenkins")]
pub use self::generator::generate_jenkins;
