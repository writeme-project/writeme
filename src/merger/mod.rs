use std::fmt::{Debug, Display};

use crate::converter::ConverterOutput;
use crate::dialoguer::conflict;
use anyhow::{Error, Ok};

/// Merges the information of multiple config files into a single object
///
/// If there are conflicting values, the user will be asked to select one of them interactively
pub struct Merger;

impl Merger {
    pub fn new() -> Self {
        Merger {}
    }

    /// Merges the provided values of a field into a single value by asking the user which one to keep
    fn merge_field<T: Clone + Debug + Display>(
        &self,
        field_name: &str,
        values: Vec<Option<T>>,
    ) -> Option<T> {
        conflict(field_name, values.clone())
    }

    /// Merges the vector fields of the provided configs into a single value by asking the user which one to keep
    pub fn merge(&self, converted_configs: Vec<ConverterOutput>) -> Result<ConverterOutput, Error> {
        let mut output = ConverterOutput::empty();

        output.name = self.merge_field(
            "name",
            converted_configs
                .iter()
                .map(|config| config.name.clone())
                .filter(|item| item.is_some() && !item.as_ref().unwrap().is_empty())
                .collect(),
        );

        output.description = self.merge_field(
            "description",
            converted_configs
                .iter()
                .map(|config| config.description.clone())
                .filter(|item| item.is_some() && !item.as_ref().unwrap().is_empty())
                .collect(),
        );

        output.version = self.merge_field(
            "version",
            converted_configs
                .iter()
                .map(|config| config.version.clone())
                .filter(|item| item.is_some() && !item.as_ref().unwrap().is_empty())
                .collect(),
        );

        output.license = self.merge_field(
            "license",
            converted_configs
                .iter()
                .map(|config| config.license.clone())
                .filter(|item| item.is_some() && !item.as_ref().unwrap().is_empty())
                .collect(),
        );

        // don't merge authors, contributors, dependencies, dev_dependencies, build_dependencies, funding
        output.contributors = Some(
            converted_configs
                .iter()
                .flat_map(|config| config.contributors.clone())
                .flatten()
                .collect(),
        );

        output.dependencies = Some(
            converted_configs
                .iter()
                .flat_map(|config| config.dependencies.clone())
                .flatten()
                .collect(),
        );

        output.dev_dependencies = Some(
            converted_configs
                .iter()
                .flat_map(|config| config.dev_dependencies.clone())
                .flatten()
                .collect(),
        );

        output.build_dependencies = Some(
            converted_configs
                .iter()
                .flat_map(|config| config.build_dependencies.clone())
                .flatten()
                .collect(),
        );

        output.funding = Some(
            converted_configs
                .iter()
                .flat_map(|config| config.funding.clone())
                .flatten()
                .collect(),
        );

        Ok(output)
    }
}
