use std::fs::File;
use std::io::Read;
use std::slice::Iter;
use anyhow::{anyhow, Result};

pub fn compute(files: Iter<String>) -> Result<()> {
    for file in files {
        let mut content = "".to_string();
        get_content(&mut content, file)?;
    }
    Ok(())
}

fn get_content(p0: &mut String, p1: &String) -> Result<()>{
    let f = File::open(p1);
    if f.is_err() {
        return Err(anyhow!("Unable to open file at path: {p1}"));
    }
    let mut f = f.unwrap();
    match f.read_to_string(p0) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(anyhow!("{}", e.to_string()))
        }
    }
}