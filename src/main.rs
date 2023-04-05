use std::fs::File;
use std::io::prelude::*;
use terminal_spinners::{ SpinnerBuilder, DOTS };

fn animate_loading() {
    let handle = SpinnerBuilder::new().spinner(&DOTS).text(" Writing README.md").start();
    let res =   writeme();
    match res {
        Ok(_) => handle.done(),
        Err(_) => handle.error(),
    }
}

fn writeme() -> std::io::Result<()> {
    let mut file = File::create("README.MD")?;
    // make a html string 
    let html = "<p align='center'>
    <h1 align='center'>
        🖋️ WRITEME
    </h1>
    <p align='center'> Repo to auto-gen the README.md file from your code </p>
</p>";

    file.write_all(html.as_bytes())?;
    Ok(())
}

fn main() {
    animate_loading();
}