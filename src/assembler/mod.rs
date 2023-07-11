use crate::{converter::ConverterOutput, utils::outputs};
use anyhow::Error;

pub mod contributing_assembler;
pub mod readme_assembler;

use contributing_assembler::ContributingAssembler;
use readme_assembler::ReadmeAssembler;

#[derive(Debug)]
pub struct Assembler {
    converted_config: ConverterOutput,
}

impl Assembler {
    pub fn new(converted_config: ConverterOutput) -> Self {
        Assembler { converted_config }
    }

    pub fn assemble(&mut self, project_location: &str, paths: &Vec<String>) -> Result<(), Error> {
        let output_contributing = &format!("{}/{}", project_location, outputs::CONTRIBUTING);
        match ContributingAssembler::new().assemble(output_contributing) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        let output_readme = &format!("{}/{}", project_location, outputs::README);
        match ReadmeAssembler::new(self.converted_config.clone()).assemble(output_readme, paths) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
