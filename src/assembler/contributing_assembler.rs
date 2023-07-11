use crate::utils::paths;
use anyhow::Error;

use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct ContributingAssembler {}

impl ContributingAssembler {
    pub fn new() -> Self {
        ContributingAssembler {}
    }

    pub fn assemble(&mut self, output_path: &str) -> Result<(), Error> {
        let mut contributing_file = match File::create(output_path) {
            Ok(f) => f,
            Err(e) => {
                return Err(Error::new(e));
            }
        };

        let contributing_tpl = paths::read_util_file_contents(paths::UtilityPath::BodyContributing);

        contributing_file.write_all(contributing_tpl.as_bytes())?;

        Ok(())
    }
}
