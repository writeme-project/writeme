use std::fmt::{Debug, Display};

use crate::converter::{ConverterOutput, Repository};
use crate::dialoguer::conflict;
use anyhow::{Error, Ok};
use itertools::Itertools;

#[derive(Clone, Debug)]
/// Identifies a value that needs to be merged. It contains the value itself and some metadata
pub struct MergeValue<T> {
    pub value: Option<T>,
    pub source_config_file_path: String,
}

impl<T: Display> Display for MergeValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(value) => write!(f, "{} ({})", value, self.source_config_file_path),
            None => write!(f, "None ({})", self.source_config_file_path),
        }
    }
}

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
        values: Vec<MergeValue<T>>,
    ) -> Option<T> {
        conflict(field_name, values)
    }

    /// Merges the vector fields of the provided configs into a single value by asking the user which one to keep
    pub fn merge(&self, converted_configs: Vec<ConverterOutput>) -> Result<ConverterOutput, Error> {
        let mut output = ConverterOutput::empty();

        output.name = self.merge_field(
            "name",
            converted_configs
                .iter()
                .filter(|config| config.name.is_some() && !config.name.as_ref().unwrap().is_empty())
                .unique_by(|item| item.name.clone())
                .map(|config| MergeValue {
                    value: config.name.clone(),
                    source_config_file_path: config.source_config_file_path.clone(),
                })
                .collect(),
        );

        output.description = self.merge_field(
            "description",
            converted_configs
                .iter()
                .filter(|config| {
                    config.description.is_some() && !config.description.as_ref().unwrap().is_empty()
                })
                .unique_by(|item| item.description.clone())
                .map(|config| MergeValue {
                    value: config.description.clone(),
                    source_config_file_path: config.source_config_file_path.clone(),
                })
                .collect(),
        );

        output.version = self.merge_field(
            "version",
            converted_configs
                .iter()
                .filter(|config| {
                    config.version.is_some() && !config.version.as_ref().unwrap().is_empty()
                })
                .unique_by(|item| item.version.clone())
                .map(|config| MergeValue {
                    value: config.version.clone(),
                    source_config_file_path: config.source_config_file_path.clone(),
                })
                .collect(),
        );

        output.license = self.merge_field(
            "license",
            converted_configs
                .iter()
                .filter(|config| {
                    config.license.is_some() && !config.license.as_ref().unwrap().is_empty()
                })
                .unique_by(|item| item.license.clone())
                .map(|config| MergeValue {
                    value: config.license.clone(),
                    source_config_file_path: config.source_config_file_path.clone(),
                })
                .collect(),
        );

        let repository_url = self.merge_field(
            "repository",
            converted_configs
                .iter()
                .filter(|config| {
                    config.repository.is_some()
                        && !config.repository.as_ref().unwrap().url.is_empty()
                })
                .unique_by(|item| item.repository.as_ref().unwrap().url.clone())
                .map(|config| MergeValue {
                    value: Option::from(config.repository.as_ref().unwrap().url.clone()),
                    source_config_file_path: config.source_config_file_path.clone(),
                })
                .collect(),
        );
        output.repository = Option::from(Repository::new(repository_url.unwrap_or("".to_string())));

        // don't merge authors, contributors, dependencies, dev_dependencies, build_dependencies, funding
        // but apply a distinct on them, base on each unique property
        output.contributors = Some(
            converted_configs
                .iter()
                .flat_map(|config| config.contributors.clone())
                .flatten()
                .unique_by(|item| item.email.clone())
                .collect(),
        );

        output.dependencies = Some(
            converted_configs
                .iter()
                .flat_map(|config| config.dependencies.clone())
                .flatten()
                .unique_by(|item| item.name.clone())
                .collect(),
        );

        output.dev_dependencies = Some(
            converted_configs
                .iter()
                .flat_map(|config| config.dev_dependencies.clone())
                .flatten()
                .unique_by(|item| item.name.clone())
                .collect(),
        );

        output.build_dependencies = Some(
            converted_configs
                .iter()
                .flat_map(|config| config.build_dependencies.clone())
                .flatten()
                .unique_by(|item| item.name.clone())
                .collect(),
        );

        output.funding = Some(
            converted_configs
                .iter()
                .flat_map(|config| config.funding.clone())
                .flatten()
                .unique_by(|item| item.url.clone())
                .collect(),
        );

        Ok(output)
    }
}
