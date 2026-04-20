use clap::Parser;
use std::path::Path;

mod arguments;
mod generator;
mod scanner;

use arguments::Args;
use scanner::find;

fn main() {
    let args = Args::parse();

    let root_dir = Path::new(&args.directory);
    if !root_dir.exists() {
        eprintln!("ERROR: Directory '{}' does not exist", args.directory);
        std::process::exit(1);
    }

    let targets = match find::find_targets(root_dir, args.recursive, &args.name) {
        Ok(targets) => targets,
        Err(err) => {
            eprintln!("ERROR: {err}");
            std::process::exit(1);
        }
    };

    println!("Found {} target(s)\n", targets.len());

    let mut success_count = 0;
    let mut failure_count = 0;

    for target in &targets {
        print!(
            "(Crate) '{}' => '{}' ... ",
            target.name,
            target.readme_path.display()
        );

        match generator::generate_docs(
            &target.project_path,
            &target.readme_path,
            &args.start_marker,
            &args.end_marker,
        ) {
            Ok(()) => {
                println!("Updated OK");
                success_count += 1;
            }
            Err(err) => {
                println!("Failed: {err}");
                failure_count += 1;
            }
        }
    }

    println!("\n{success_count} updated, {failure_count} failed");

    if failure_count > 0 {
        std::process::exit(1);
    }
}
