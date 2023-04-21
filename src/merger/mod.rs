use std::fmt::{Debug, Display};

use crate::converter::ConverterOutput;
use anyhow::{Error, Ok};
use dialoguer::console::style;
use itertools::Itertools;

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
        let with_value = values
            .iter()
            .filter(|s| s.is_some())
            .map(|s| s.as_ref().unwrap().clone())
            .collect_vec();

        // every value of the field is empty, return None
        if with_value.is_empty() {
            return None;
        }

        // does the field need merging? it does so when the filtered non-None values are more than one
        let needs_merge = with_value.len() > 1;

        if !needs_merge {
            return Some(with_value[0].clone());
        }

        // ask the user which value to keep
        let selection = dialoguer::Select::new()
            .with_prompt(format!(
                "\nFound conflicting values for field {}, select one",
                style(field_name).green()
            ))
            .items(&with_value)
            .default(0)
            .interact()
            .unwrap_or(0);

        Some(with_value[selection].clone())
    }

    /// Merges the vector fields of the provided configs into a single value by asking the user which one to keep
    pub fn merge(&self, converted_configs: Vec<ConverterOutput>) -> Result<ConverterOutput, Error> {
        let mut output = ConverterOutput::empty();

        output.name = self.merge_field(
            "name",
            converted_configs
                .iter()
                .map(|config| config.name.clone())
                .collect(),
        );

        output.description = self.merge_field(
            "description",
            converted_configs
                .iter()
                .map(|config| config.description.clone())
                .collect(),
        );

        output.version = self.merge_field(
            "version",
            converted_configs
                .iter()
                .map(|config| config.version.clone())
                .collect(),
        );

        output.contributors = self.merge_field(
            "contributors",
            converted_configs
                .iter()
                .map(|config| config.contributors.clone())
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

        Ok(output)
    }
}
