use std::error::Error;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

struct CFile {
    file_name: String,
    project_name: String,
    user_name: String,
    function_type: String,
}

mod create_c_file;
mod edit_makefile;

pub fn create_new_file() -> Result<(), Box<dyn Error>> {
    let mut this_file = CFile {
        file_name: String::from(""),
        project_name: String::from(""),
        user_name: String::from(""),
        function_type: String::from(""),
    };
    println!("Please enter the name of the file you are trying to create (no .c): ");

    io::stdin()
        .read_line(&mut this_file.file_name)
        .expect("Error reading from console");
    println!("Please enter the type of function you want to add: ");

    io::stdin()
        .read_line(&mut this_file.function_type)
        .expect("Error reading from console");

    println!("Please enter the name of the project you are currently working on: ");

    io::stdin()
        .read_line(&mut this_file.project_name)
        .expect("Error reading from console");
    println!("Please enter your name: ");

    io::stdin()
        .read_line(&mut this_file.user_name)
        .expect("Error reading from console");

    this_file.file_name = this_file.file_name.trim().to_string();
    this_file.project_name = this_file.project_name.trim().to_string();
    this_file.user_name = this_file.user_name.trim().to_string();
    this_file.function_type = this_file.function_type.trim().to_string();

    create_c_file::new(
        this_file.file_name.clone(),
        this_file.project_name.clone(),
        this_file.function_type.clone(),
        this_file.user_name,
    )
    .expect("Error creating new file");

    let header_path = format!("{}.h", this_file.project_name);
    let mut header_file = OpenOptions::new().append(true).open(header_path).unwrap();

    writeln!(
        header_file,
        "{} {}();",
        this_file.function_type, this_file.file_name
    )?;

    edit_makefile::new(this_file.file_name, this_file.project_name)
        .expect("Error Editing Makefile");

    Ok(())
}
