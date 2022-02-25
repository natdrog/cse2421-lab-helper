use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn new(
    file_name: String,
    project_name: String,
    function_type: String,
    user_name: String,
) -> Result<(), Box<dyn Error>> {
    let data = format!(
        "/* BY SUBMITTING THIS FILE TO CARMEN, I CERTIFY THAT I HAVE
** STRICTLY ADHERED TO THE TENURES OF THE
** OHIO STATE UNIVERSITY'S ACADEMIC INTEGRITY POLICY.
** NAME: {}
*/
    
#include \"{}.h\"
    
{} {}(){{
    /* Return default */
    return 0;
}}",
        user_name, project_name, function_type, file_name
    );

    let full_path = format!("./{}.c", file_name);
    let mut file = File::create(full_path)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
