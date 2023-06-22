use std::fmt::{Debug, Display};

use crate::{
    converter::{ConverterOutput, Repository},
    dialoguer::{select_option, SelectOption},
    elements::license::{License, SupportedLicense},
};
use anyhow::{Error, Ok};
use itertools::Itertools;
use strum::IntoEnumIterator;

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
        values: Vec<SelectOption<T>>,
        custom_label: Option<String>,
    ) -> Option<T> {
        select_option(field_name, values, custom_label)
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
                .map(|config| SelectOption {
                    value: config.name.clone(),
                    name: config.source_config_file_path.clone(),
                })
                .collect(),
            None,
        );

        output.description = self.merge_field(
            "description",
            converted_configs
                .iter()
                .filter(|config| {
                    config.description.is_some() && !config.description.as_ref().unwrap().is_empty()
                })
                .unique_by(|item| item.description.clone())
                .map(|config| SelectOption {
                    value: config.description.clone(),
                    name: config.source_config_file_path.clone(),
                })
                .collect(),
            None,
        );

        output.version = self.merge_field(
            "version",
            converted_configs
                .iter()
                .filter(|config| {
                    config.version.is_some() && !config.version.as_ref().unwrap().is_empty()
                })
                .unique_by(|item| item.version.clone())
                .map(|config| SelectOption {
                    value: config.version.clone(),
                    name: config.source_config_file_path.clone(),
                })
                .collect(),
            None,
        );

        output.license = self.merge_licenses(converted_configs.clone());

        let repository_url = self.merge_field(
            "repository",
            converted_configs
                .iter()
                .filter(|config| {
                    config.repository.is_some()
                        && !config.repository.as_ref().unwrap().url.is_empty()
                })
                .unique_by(|item| item.repository.as_ref().unwrap().url.clone())
                .map(|config| SelectOption {
                    value: Option::from(config.repository.as_ref().unwrap().url.clone()),
                    name: config.source_config_file_path.clone(),
                })
                .collect(),
            None,
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

    fn merge_licenses(&self, converted_configs: Vec<ConverterOutput>) -> Option<License>{
        let selected:Option<License>;

        if converted_configs.iter().all(|config| {
            config.license.is_none()
                || config.license.as_ref().unwrap().name == SupportedLicense::Unknown
        }) {
            let available = SupportedLicense::iter()
                .map(|license| SelectOption {
                    name: license.to_string(),
                    value: Some(License::from_name(license.to_string())),
                })
                .collect();

            selected = self.merge_field(
                "license", 
                available, 
                Some("I was unable to find a license in your project! Select one from the list below".to_string())
            );

            return selected;
        } 

        selected = self.merge_field(
            "license",
            converted_configs
                .iter()
                .filter(|config| {
                    config.license.is_some()
                        && config.license.as_ref().unwrap().name != SupportedLicense::Unknown
                })
                .unique_by(|item| item.license.as_ref().unwrap().name.clone())
                .map(|config| SelectOption {
                    name: config.source_config_file_path.clone(),
                    value: Some(config.license.clone().unwrap()),
                })
                .collect(),
            None,
        );

        return selected
    }
}
