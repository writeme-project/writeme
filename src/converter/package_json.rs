use serde_json::{json, Value};
use std::rc::Rc;

use anyhow::{anyhow, Error, Ok};

use super::{Component, Contributor, ConverterOutput, Decorator, Dependency};

// Concrete Decorators call the wrapped object and alter its result in some
// way.
pub struct PackageJson {
    component: Rc<dyn Component>,
}

impl Decorator for PackageJson {
    fn new(component: Rc<dyn Component>) -> Self {
        PackageJson { component }
    }
}

impl Component for PackageJson {
    fn convert(&self, file_contents: String) -> Result<ConverterOutput, Error> {
        let mut output = ConverterOutput::empty();

        let json: Value = serde_json::from_str(&file_contents.as_str()).unwrap();

        output.name = json["name"].as_str().map(|s| s.to_string());
        output.description = json["description"].as_str().map(|s| s.to_string());
        output.version = json["version"].as_str().map(|s| s.to_string());

        if json["author"].as_object().is_some() {
            let author = self.parse_contributor(&json["author"]);

            if author.is_ok() {
                output.contributors = Some(vec![author.unwrap()]);
            }
        }

        if json["contributors"].as_array().is_some() {
            let contributors = json["contributors"].as_array().unwrap();

            output.contributors = Some(
                contributors
                    .iter()
                    .map(|c| self.parse_contributor(c))
                    .filter_map(|c| c.ok())
                    .collect(),
            );
        }

        output.license = json["license"].as_str().map(|s| s.to_string());

        output.keywords = json["package"]["keywords"]
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());
        output.homepage_url = Some(json["package"]["homepage"].to_string());
        output.repository_url = Some(json["package"]["repository"].to_string());

        output.dependencies = json["dependencies"].as_object().map(|v| {
            v.iter()
                // .map(|(k, v)| self.parse_dependency(k, v))
                .filter_map(|(key, value)| {
                    let dependency = self.parse_dependency(key, value);

                    dependency.ok()
                })
                .collect()
        });

        output.dev_dependencies = json["devDependencies"].as_object().map(|v| {
            v.iter()
                // .map(|(k, v)| self.parse_dependency(k, v))
                .filter_map(|(key, value)| {
                    let dependency = self.parse_dependency(key, value);

                    dependency.ok()
                })
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

        Ok(super::Contributor {
            name: attrs["name"].as_str().map(|s| s.to_string()),
            email: attrs["email"].as_str().map(|s| s.to_string()),
            url: attrs["url"].as_str().map(|s| s.to_string()),
        })
    }

    fn parse_dependency(&self, key: &String, value: &Value) -> Result<Dependency, Error> {
        Ok(Dependency {
            name: key.to_string(),
            version: Some(value.to_string()),
        })
    }
}
