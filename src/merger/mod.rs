use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::converter::ConverterOutput;
use anyhow::{Error, Ok};
use itertools::Itertools;

enum Mergeable {
    Name,
    Description,
    Version,
    Contributors,
}

pub struct Merger;

impl Merger {
    pub fn new() -> Self {
        Merger {}
    }

    fn merge_string(&self, field_name: &str, strings: Vec<Option<String>>) -> Option<String> {
        let with_value = strings
            .iter()
            .filter(|s| s.is_some())
            .map(|s| s.as_ref().unwrap())
            .collect_vec();

        if with_value.is_empty() {
            return None;
        }

        let needs_merge = with_value.len() > 1;

        if !needs_merge {
            return Some(with_value[0].clone());
        }

        let selection = dialoguer::Select::new()
            .with_prompt(format!(
                "Found conflicting values for field {}, select one:",
                field_name
            ))
            .items(&with_value)
            .default(0)
            .interact()
            .unwrap_or(0);

        Some(with_value[selection].clone())
    }

    fn merge_vec(&self, vecs: Vec<&Vec<String>>) -> Option<Vec<String>> {
        let mut merged = Vec::new();

        for vec in vecs {
            merged.extend(vec.clone());
        }

        if merged.is_empty() {
            None
        } else {
            Some(merged)
        }
    }

    pub fn merge(&self, converted_configs: Vec<ConverterOutput>) -> Result<ConverterOutput, Error> {
        let mut output = ConverterOutput::empty();

        output.name = self.merge_string(
            "name",
            converted_configs
                .iter()
                .map(|config| config.name.clone())
                .collect(),
        );

        Ok(output)
    }
}
