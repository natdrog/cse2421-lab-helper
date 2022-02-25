use std::error::Error;
use std::fs;
use std::io;

mod create_header;
mod create_main;
mod create_makefile;
mod create_readme;

struct Project {
    project_name: String,
    user_name: String,
}

pub struct Config {
    pub query: String,
    pub command: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        } else if args.len() > 3 {
            return Err("too many arguments");
        }

        let query = args[1].clone();
        let command = args[2].clone();

        if query != "new" {
            return Err("invalid query");
        }
        if command != "project" && command != "file" {
            return Err("invalid command");
        }

        Ok(Config { query, command })
    }
}

pub fn create_new_project() -> Result<(), Box<dyn Error>> {
    println!("Please enter the name of the project you are trying to create: ");
    let mut this_project = Project {
        project_name: String::from(""),
        user_name: String::from(""),
    };
    io::stdin()
        .read_line(&mut this_project.project_name)
        .expect("Error reading from console");

    println!("Please enter your name: ");
    io::stdin()
        .read_line(&mut this_project.user_name)
        .expect("Error reading from console");
    this_project.project_name = this_project.project_name.trim().to_string();
    this_project.user_name = this_project.user_name.trim().to_string();

    let path = format!("./{}", this_project.project_name);

    fs::create_dir(path.clone()).expect("Problem creating directory");

    create_main::new(
        path.clone(),
        this_project.project_name.clone(),
        this_project.user_name.clone(),
    )
    .expect("Problem writing to file");

    create_makefile::new(
        path.clone(),
        this_project.project_name.clone(),
        this_project.user_name.clone(),
    )
    .expect("Problem creating Makefile");

    create_header::new(
        path.clone(),
        this_project.user_name.clone(),
        this_project.project_name.clone(),
    )
    .expect("Problem creating header");

    create_readme::new(path, this_project.user_name, this_project.project_name)
        .expect("Problem crating Readme");

    Ok(())
}
