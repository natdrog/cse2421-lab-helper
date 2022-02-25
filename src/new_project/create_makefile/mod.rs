use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn new(path: String, project_name: String, user_name: String) -> Result<(), Box<dyn Error>> {
    let project_name_capital = project_name.clone().to_uppercase().to_string();

    let data = format!(
        "# BY SUBMITTING THIS FILE TO CARMEN, I CERTIFY THAT I HAVE
# STRICTLY ADHERED TO THE TENURES OF THE
# OHIO STATE UNIVERSITY'S ACADEMIC INTEGRITY POLICY.
# NAME: {0}
        
gcc_opt = -ansi -pedantic -g -Wimplicit-function-declaration -Wreturn-type -c
        
all: {1} {1}.zip

{1}.zip: Makefile {1}main.c {2}README
\tzip {1}.zip Makefile {1}main.c {2}README
        
{1}: {1}main.o
\tgcc {1}main.o -o {1}
        
{1}main.o: {1}main.c {1}.h
\tgcc $(gcc_opt) {1}main.c
        
clean: 
\trm -rf *.o {1} {1}.zip
",
        user_name, project_name, project_name_capital
    );

    let full_path = format!("{}/Makefile", path);
    let mut file = File::create(full_path)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
