mod assembler;
mod converter;
mod dialoguer;
mod merger;
mod scanner;
mod utils;

/// Method used to Scan the project merges the data found and assembles it to create a README file
fn writeme() {
    let converter = converter::Converter::new();
    let merger = merger::Merger::new();

    let configs = scanner::scan_configs().unwrap();
    let mut outputs = vec![];

    for config in configs {
        let output = converter.convert(&config);

        // if unable to convert the file skip it
        if output.is_err() {
            continue;
        }
        outputs.push(output.unwrap());
    }

    let merged = match merger.merge(outputs) {
        Ok(merged) => merged,
        Err(e) => {
            println!("Error: Failed to merge: {}", e);
            return;
        }
    };

    match assembler::Assembler::new(merged).assemble() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: Failed to assemble: {}", e);
            return;
        }
    };
}

fn main() {
    dialoguer::hello();
    writeme();
    dialoguer::bye();
}
