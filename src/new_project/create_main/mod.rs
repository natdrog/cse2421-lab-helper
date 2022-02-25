use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn new(path: String, project_name: String, user_name: String) -> Result<(), Box<dyn Error>> {
    let data = format!(
        "/* BY SUBMITTING THIS FILE TO CARMEN, I CERTIFY THAT I HAVE
** STRICTLY ADHERED TO THE TENURES OF THE
** OHIO STATE UNIVERSITY'S ACADEMIC INTEGRITY POLICY.
** NAME: {}
*/
    
#include \"{}.h\"
    
int main(){{
    /* Return default */
    return 0;
}}",
        user_name, project_name,
    );
    let mainfile = format!("{}main", project_name.clone());

    let full_path = format!("{}/{}.c", path, mainfile);
    let mut file = File::create(full_path)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
