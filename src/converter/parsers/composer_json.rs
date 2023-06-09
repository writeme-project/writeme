use std::str::FromStr;

use anyhow::{anyhow, Error};
use serde_json::Value;
use strum::IntoEnumIterator;

use crate::converter::{
    Component, Contributor, ConverterOutput, Decorator, Dependency, Funding, FundingType, License,
    Repository,
};

/// The composer.json parser
///
/// Reference: https://getcomposer.org/doc/04-schema.md
pub struct ComposerJson {
    // component: Rc<dyn Component>,
}

impl Decorator for ComposerJson {
    fn new(/* component: Rc<dyn Component> */) -> Self {
        // ComposerJson { component }
        ComposerJson {}
    }
}

impl Component for ComposerJson {
    fn convert(&self, file_path: String, file_contents: String) -> Result<ConverterOutput, Error> {
        let mut output = ConverterOutput::empty();

        output.source_config_file_path = file_path;

        let json: Value = serde_json::from_str(file_contents.as_str()).unwrap();

        if !json["name"].is_null()
            && json["name"].as_str().is_some()
            && !json["name"].as_str().unwrap().is_empty()
        {
            output.name = Some(json["name"].to_string());
        }

        if !json["version"].is_null()
            && json["version"].as_str().is_some()
            && !json["version"].as_str().unwrap().is_empty()
        {
            output.version = Some(json["version"].to_string());
        }

        if !json["description"].is_null()
            && json["description"].as_str().is_some()
            && !json["description"].as_str().unwrap().is_empty()
        {
            output.description = Some(json["description"].to_string());
        }

        if !json["repository_url"].is_null()
            && json["repository_url"].as_str().is_some()
            && !json["repository_url"].as_str().unwrap().is_empty()
        {
            output.repository = Some(Repository::new(json["repository_url"].to_string()));
        }

        if json["authors"].as_array().is_some() {
            let contributors = json["authors"].as_array().unwrap();

            output.contributors = Some(
                contributors
                    .iter()
                    .map(|c| self.parse_contributor(c))
                    .filter_map(|c| c.ok())
                    .collect(),
            );
        }

        if !json["license"].is_null()
            && json["license"].as_str().is_some()
            && !json["license"].as_str().unwrap().is_empty()
        {
            output.license = Some(License::from_name(json["license"].to_string()));
        }

        output.keywords = json["keywords"]
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());
        output.homepage_url = Some(json["homepage"].to_string());

        if json["repository"].as_object().is_some() {
            let repo = json["repository"].as_object().unwrap();

            if repo["url"].as_str().is_some() && !json["url"].as_str().unwrap().is_empty() {
                output.repository = Some(Repository::new(repo["url"].to_string()));
            }
        } else if json["repository"].as_str().is_some()
            && !json["repository"].as_str().unwrap().is_empty()
        {
            output.repository = Some(Repository::new(json["repository"].to_string()));
        }

        output.dependencies = json["require"].as_object().map(|v| {
            v.iter()
                // .map(|(k, v)| self.parse_dependency(k, v))
                .filter_map(|(key, value)| {
                    let dependency = self.parse_dependency(key, value);

                    dependency.ok()
                })
                .collect()
        });

        output.dev_dependencies = json["require-dev"].as_object().map(|v| {
            v.iter()
                // .map(|(k, v)| self.parse_dependency(k, v))
                .filter_map(|(key, value)| {
                    let dependency = self.parse_dependency(key, value);

                    dependency.ok()
                })
                .collect()
        });

        output.funding = json["funding"].as_array().map(|v| {
            v.iter()
                .map(|f| self.parse_funding(f))
                .filter_map(|f| f.ok())
                .collect()
        });

        output.trim();
        Ok(output)
    }

    fn parse_contributor(&self, contributor: &Value) -> Result<Contributor, Error> {
        let as_obj = contributor.as_object();

        if as_obj.is_none() {
            return Err(anyhow!(
                "Could not parse contributor! Value: {}",
                contributor
            ));
        }

        let attrs = as_obj.unwrap();

        let name = attrs.get("name").map(|s| s.to_string());

        let email = attrs.get("email").map(|s| s.to_string());

        let url = attrs.get("homepage").map(|s| s.to_string());

        Ok(Contributor { name, email, url })
    }

    fn parse_dependency(&self, key: &String, value: &Value) -> Result<Dependency, Error> {
        Ok(Dependency {
            name: key.to_string(),
            version: Some(value.to_string()),
        })
    }

    fn parse_funding(&self, funding: &Value) -> Result<Funding, Error> {
        let possible_values = FundingType::iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>();

        let f_type = funding["type"].to_string();
        let url = funding["url"].to_string();

        for possible_value in possible_values.iter() {
            if f_type.contains(possible_value) || url.contains(possible_value) {
                let f_type = match FundingType::from_str(possible_value) {
                    Ok(t) => t,
                    Err(_e) => {
                        return Err(anyhow!("Unsupported funding type"));
                    }
                };

                let funding = Funding {
                    f_type,
                    url: Some(url),
                };

                return Ok(funding);
            }
        }

        Err(anyhow!("Unsupported funding type"))
    }
}
