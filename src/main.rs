use std::fs;
use std::fs::File;
use std::io::prelude::*;
use terminal_spinners::{ SpinnerBuilder, DOTS };
use std::io::Result;

fn animate_loading() {
    let handle = SpinnerBuilder::new().spinner(&DOTS).text(" Writing README.md").start();
    let res =   writeme();
    match res {
        Ok(_) => handle.done(),
        Err(_) => handle.error(),
    }
}

fn writeme() -> Result<()> {
    let mut file = File::create("README.MD")?;
    // make a html string 
    let readme = read_template().replace("{title_icon}", "🖋️")
    .replace("{title_name}", "WRITEME")
    .replace("{titel_description}", "Repo to auto-gen the README.md file from your code");

    file.write_all(readme.as_bytes())?;
    Ok(())
}

fn read_template() -> String {
    let contents = fs::read_to_string("TEMPLATE.md")
        .expect("Should have been able to read the template file");
    return contents;
}

fn main() {
    animate_loading();
}


