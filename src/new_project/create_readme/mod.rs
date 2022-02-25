use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn new(path: String, user_name: String, project_name: String) -> Result<(), Box<dyn Error>> {
    let data = format!(
        "/* BY SUBMITTING THIS FILE TO CARMEN, I CERTIFY THAT I HAVE
** STRICTLY ADHERED TO THE TENURES OF THE
** OHIO STATE UNIVERSITY'S ACADEMIC INTEGRITY POLICY.
** NAME: {}
*/
",
        user_name
    );

    let project_name = project_name.to_uppercase();

    let full_path = format!("{}/{}README", path, project_name);
    let mut file = File::create(full_path)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
