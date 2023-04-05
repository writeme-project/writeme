use std::fs::File;
use std::io::prelude::*;
use terminal_spinners::{ SpinnerBuilder, DOTS };
use std::env;

fn animate_loading() {
    let handle = SpinnerBuilder::new().spinner(&DOTS).text(" Writing README.md").start();
    let res =   write_readme();
    match res {
        Ok(_) => handle.done(),
        Err(_) => handle.error(),
    }
}

fn write_readme() -> std::io::Result<()> {
    let mut file = File::create("README.MD")?;
    file.write_all(b"# WRITEME")?;
    Ok(())
}

fn main() {
    animate_loading();
}