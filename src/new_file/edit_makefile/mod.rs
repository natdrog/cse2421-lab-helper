use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub fn new(file_name: String, project_name: String) -> Result<(), Box<dyn Error>> {
    fs::rename("Makefile", "Makefile2")?;

    let rf = File::open("Makefile2").unwrap();
    let rf = BufReader::new(rf);

    let wf = OpenOptions::new()
        .write(true)
        .create(true)
        .open("Makefile")
        .expect("unable to open file");
    let mut wf = BufWriter::new(wf);

    let first_find = format!("{}:", project_name);
    let mut just_after_first = false;
    for line in rf.lines() {
        let mut line = line.expect("Unable to read line");
        if just_after_first {
            for _ in 0..(project_name.len() + 3) {
                line.pop();
            }
            writeln!(wf, "{}{}.o -o {}", line, file_name, project_name)?;
            just_after_first = false;
        } else {
            if line.starts_with(&first_find[..]) {
                writeln!(wf, "{} {}.o", line, file_name)?;
                just_after_first = true;
            } else if line.starts_with("clean:") {
                writeln!(wf, "{0}.o: {0}.c {1}.h", file_name, project_name)?;
                writeln!(wf, "\tgcc $(gcc_opt) {}.c", file_name)?;
                writeln!(wf, "{}", line)?;
            } else {
                writeln!(wf, "{}", line)?;
            }
        }
    }

    fs::remove_file("Makefile2").expect("Problem deleting makefile copy");

    Ok(())
}
