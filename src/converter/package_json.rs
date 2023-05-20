use std::str::FromStr;

use serde_json::Value;

use anyhow::{anyhow, Error};

use super::{
    Component, Contributor, Contributors, ConverterOutput, Decorator, Dependency, EnumIterator,
    Funding, FundingType, Fundings, SupportedFile,
};

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

        output.source_config_file = SupportedFile::PackageJson;

        let json: Value = serde_json::from_str(&file_contents.as_str()).unwrap();

        if !json["name"].is_null()
            && json["name"].as_str().is_some()
            && json["name"].as_str().unwrap().len() > 0
        {
            output.name = Some(json["name"].to_string());
        }

        if !json["version"].is_null()
            && json["version"].as_str().is_some()
            && json["version"].as_str().unwrap().len() > 0
        {
            output.version = Some(json["version"].to_string());
        }

        if !json["description"].is_null()
            && json["description"].as_str().is_some()
            && json["description"].as_str().unwrap().len() > 0
        {
            output.description = Some(json["description"].to_string());
        }

        if !json["repository_url"].is_null()
            && json["repository_url"].as_str().is_some()
            && json["repository_url"].as_str().unwrap().len() > 0
        {
            output.repository_url = Some(json["repository_url"].to_string());
        }

        if json["author"].as_object().is_some() {
            let author = self.parse_contributor(&json["author"]);

            if author.is_ok() {
                output.contributors = Some(Contributors(vec![author.unwrap()]));
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

        if !json["license"].is_null()
            && json["license"].as_str().is_some()
            && json["license"].as_str().unwrap().len() > 0
        {
            output.license = Some(json["license"].to_string());
        }

        output.keywords = json["keywords"]
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());
        output.homepage_url = Some(json["package"]["homepage"].to_string());

        if json["repository"].as_object().is_some() {
            let repo = json["repository"].as_object().unwrap();

            if !repo["url"].is_null() && repo["url"].as_str().is_some() {
                output.repository_url = Some(repo["url"].to_string());
            }
        } else if json["repository"].as_str().is_some()
            && json["repository"].as_str().unwrap().len() > 0
        {
            output.repository_url = Some(json["repository"].to_string());
            println!("{:?}", json["repository_url"]);
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
            match self.parse_funding(&json["funding"]) {
                Ok(f) => {
                    output.funding = Some(Fundings(vec![f]));
                }
                Err(_e) => (),
            };
        }

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

    fn parse_funding(&self, funding: &Value) -> Result<Funding, Error> {
        let possible_values = FundingType::enum_iterator()
            .map(|t| t.to_string())
            .collect::<Vec<_>>();

        let mut to_compare = vec![];

        if funding.is_string() {
            to_compare.push(funding.to_string());
        } else {
            let f_type = funding["type"].to_string();
            let url = funding["url"].to_string();

            to_compare.push(url);
            to_compare.push(f_type);
        }

        for possible_value in possible_values.iter() {
            let is_in: bool = to_compare.iter().any(|v| v.contains(possible_value));

            if is_in {
                let f_type = match FundingType::from_str(&possible_value) {
                    Ok(t) => t,
                    Err(_e) => {
                        return Err(anyhow!("Unsupported funding type"));
                    }
                };

                let funding = Funding {
                    f_type,
                    url: Some(to_compare[0].clone()),
                };

                return Ok(funding);
            }
        }

        Err(anyhow!("Unsupported funding type"))
    }
}
