use serde_json::Value;

use anyhow::{anyhow, Error, Ok};

use super::{Component, Contributor, ConverterOutput, Decorator, Dependency, Funding};

/// The package.json parser
///
/// Reference: https://docs.npmjs.com/cli/v9/configuring-npm/package-json#dependencies
pub struct PackageJson {
    // component: Rc<dyn Component>,
}

impl Decorator for PackageJson {
    fn new(/* component: Rc<dyn Component> */) -> Self {
        // PackageJson { component }
        PackageJson {}
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

        output.keywords = json["keywords"]
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());
        output.homepage_url = Some(json["package"]["homepage"].to_string());

        if json["repository"].as_object().is_some() {
            let repo = json["repository"].as_object().unwrap();

            if repo["url"].as_str().is_some() {
                output.repository_url = Some(repo["url"].to_string());
            }
        } else if json["repository"].as_str().is_some() {
            output.repository_url = Some(json["repository"].to_string());
        }

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

        if json["funding"].is_array() {
            output.funding = json["funding"].as_array().map(|v| {
                v.iter()
                    .map(|f| self.parse_funding(f))
                    .filter_map(|f| f.ok())
                    .collect()
            });
        } else if json["funding"].is_object() || json["funding"].is_string() {
            output.funding = Some(vec![self.parse_funding(&json["funding"]).unwrap()]);
        }

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

    fn parse_funding(&self, funding: &Value) -> Result<super::Funding, Error> {
        if funding.is_string() {
            return Ok(Funding {
                url: Some(funding.as_str().unwrap().to_string()),
                f_type: None,
            });
        }

        if funding.is_object() {
            let attrs = funding.as_object().unwrap();

            return Ok(Funding {
                url: attrs["url"].as_str().map(|s| s.to_string()),
                f_type: attrs["type"].as_str().map(|s| s.to_string()),
            });
        }

        Err(anyhow!("Could not parse funding! Value: {}", funding))
    }
}
