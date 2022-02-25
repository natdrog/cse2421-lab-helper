use std::env;
use std::error::Error;
use std::process;

use new_project::Config;

mod new_file;
mod new_project;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.command == "project" {
        new_project::create_new_project().expect("Problem creating new project");
    } else if config.command == "file" {
        new_file::create_new_file().expect("Problem crating new file");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
