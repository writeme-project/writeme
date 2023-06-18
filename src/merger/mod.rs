use std::fmt::{Debug, Display};

use crate::converter::parts::license::{License, SupportedLicense};
use crate::converter::{ConverterOutput, Repository};
use crate::dialoguer::{select_option, SelectOption};
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
    ) -> Option<T> {
        let options = values
            .iter()
            .map(|value| SelectOption {
                name: format!("{}", value),
                value: value.value.clone(),
            })
            .collect();

        select_option(field_name, options, None)
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
        );

        if converted_configs.iter().all(|config| {
            config.license.is_none()
                || config.license.as_ref().unwrap().name == SupportedLicense::Unknown
        }) {
            let supported: Vec<String> = SupportedLicense::iter()
                .map(|license| license.to_string())
                .collect();

            let available = supported
                .iter()
                .map(|license| SelectOption {
                    name: license.clone(),
                    value: Some(license.clone()),
                })
                .collect();

            let selected = select_option(
                "LICENSE",
                available,
                Some("I was unable to find a license in your project! Select one from the list below"
                .to_string()),
            );

            if selected.is_some() {
                output.license = Some(License::from_name(selected.unwrap()));
            }
        } else {
            let selected = self.merge_field(
                "license",
                converted_configs
                    .iter()
                    .filter(|config| {
                        config.license.is_some()
                            && config.license.as_ref().unwrap().name != SupportedLicense::Unknown
                    })
                    .map(|config| SelectOption {
                        name: config.license.as_ref().unwrap().name.to_string(),
                        value: Some(config.license.as_ref().unwrap()),
                    })
                    .collect(),
            );

            if selected.is_some() {
                output.license = Some(selected.unwrap().clone());
            }
        }

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
