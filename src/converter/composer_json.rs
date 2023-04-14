use serde_json::Value;

use anyhow::{anyhow, Error, Ok};

use super::{Component, Contributor, ConverterOutput, Decorator, Dependency, Funding};

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
    fn convert(&self, file_contents: String) -> Result<ConverterOutput, Error> {
        let mut output = ConverterOutput::empty();

        let json: Value = serde_json::from_str(&file_contents.as_str()).unwrap();

        output.name = json["name"].as_str().map(|s| s.to_string());
        output.description = json["description"].as_str().map(|s| s.to_string());
        output.version = json["version"].as_str().map(|s| s.to_string());

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

        output.license = json["license"].as_str().map(|s| s.to_string());

        output.keywords = json["keywords"]
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());
        output.homepage_url = Some(json["homepage"].to_string());

        if json["repository"].as_object().is_some() {
            let repo = json["repository"].as_object().unwrap();

            if repo["url"].as_str().is_some() {
                output.repository_url = Some(repo["url"].to_string());
            }
        } else if json["repository"].as_str().is_some() {
            output.repository_url = Some(json["repository"].to_string());
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
        Ok(Funding {
            f_type: funding["type"].as_str().map(|s| s.to_string()),
            url: funding["url"].as_str().map(|s| s.to_string()),
        })
    }
}
