#![doc = include_str!("../../readme.md")]

mod arguments;

use std::path::Path;

use clap::Parser;

use arguments::Args;
use arguments::Commands;
use clapdocs::ScanOptions;
use clapdocs::Target;
use clapdocs::find_projects;

#[cfg(feature = "jenkins")]
use clapdocs::generator::JenkinsLibraryConfig;
#[cfg(feature = "markdown")]
use clapdocs::generator::MarkdownConfig;

fn main() {
    let args = Args::parse();

    let root_dir = Path::new(&args.directory);
    if !root_dir.exists() {
        eprintln!("ERROR: Directory '{}' does not exist", args.directory);
        std::process::exit(1);
    }

    match args.command {
        #[cfg(feature = "markdown")]
        Commands::Markdown {
            name,
            start_marker,
            end_marker,
        } => {
            use clapdocs::generate_docs;

            let options = ScanOptions {
                root_dir,
                recursive: args.recursive,
                readme_name: Some(&name),
            };
            let targets = find_or_exit(&options);

            run_generation(&targets, "updated", |target| {
                let readme_path = target.readme_path.as_deref()?;
                print!(
                    "(Crate) '{}' => '{}' ... ",
                    target.name,
                    readme_path.display()
                );

                let markdown_config = MarkdownConfig {
                    project_path: &target.project_path,
                    readme_path,
                    start_marker: &start_marker,
                    end_marker: &end_marker,
                };

                Some(generate_docs(markdown_config))
            });
        }
        #[cfg(feature = "jenkins")]
        Commands::Jenkins {
            output_dir,
            package_name,
            execution_model,
            json_output,
            stubs,
        } => {
            use clapdocs::generate_jenkins;

            let output_dir = match output_dir.as_deref() {
                Some(dir) => Path::new(dir),
                None => {
                    eprintln!("ERROR: --output-dir is required in jenkins mode");
                    std::process::exit(1);
                }
            };

            let options = ScanOptions {
                root_dir,
                recursive: args.recursive,
                readme_name: None,
            };
            let targets = find_or_exit(&options);

            run_generation(&targets, "generated", |target| {
                print!(
                    "(Crate) '{}' => '{}' ... ",
                    target.name,
                    output_dir.display()
                );

                let jenkins_config = JenkinsLibraryConfig {
                    project_path: &target.project_path,
                    output_dir,
                    package_name: &package_name,
                    execution_model,
                    json_output,
                    stubs,
                };

                Some(generate_jenkins(jenkins_config))
            });
        }
    }
}

fn find_or_exit(options: &ScanOptions) -> Vec<Target> {
    match find_projects(options) {
        Ok(targets) => {
            println!("Found {} target(s)\n", targets.len());
            targets
        }
        Err(err) => {
            eprintln!("ERROR: {err}");
            std::process::exit(1);
        }
    }
}

fn run_generation(
    targets: &[Target],
    verb: &str,
    mut generate: impl FnMut(&Target) -> Option<Result<(), String>>,
) {
    let mut success_count = 0;
    let mut failure_count = 0;

    for target in targets {
        match generate(target) {
            None => continue,
            Some(Ok(())) => {
                println!("{verb} OK");
                success_count += 1;
            }
            Some(Err(err)) => {
                println!("Failed: {err}");
                failure_count += 1;
            }
        }
    }

    println!("\n{success_count} {verb}, {failure_count} failed");

    if failure_count > 0 {
        std::process::exit(1);
    }
}
