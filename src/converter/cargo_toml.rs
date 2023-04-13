use serde_json::{json, Value};
use std::rc::Rc;

use anyhow::{anyhow, Error, Ok};

use super::{Component, ConverterOutput, Decorator, Dependency};

/// The Cargo.toml file relevant contents
///
/// Reference: https://doc.rust-lang.org/cargo/reference/manifest.html
struct CargoTomlOutput {
    // [package]
    /// The name of the package
    name: Option<String>,
    /// The version of the package
    version: Option<String>,
    /// The authors of the package
    authors: Option<String>,
    /// The Rust edition, e.g. "2018"
    edition: Option<String>,
    /// The minimum required version of the Rust compiler
    rust_version: Option<String>,
    /// The description of the package
    description: Option<String>,
    /// The documentation url for the package
    documentation: Option<String>,
    /// The readme file for the package
    readme: Option<String>,
    /// The homepage url for the package
    homepage: Option<String>,
    /// The repository url for the package
    repository: Option<String>,
    /// The license of the package
    license: Option<String>,
    /// The license file of the package for NON-standard licenses
    license_file: Option<String>,
    /// The keywords that describe the package
    keywords: Option<String>,
    /// The categories that describe the package on crates.io
    categories: Option<String>,

    build: Option<String>,
    links: Option<String>,
    exclude: Option<String>,
    include: Option<String>,
    publish: Option<String>,
    metadata: Option<String>,
    default_run: Option<String>,
    autobins: Option<String>,
    autoexamples: Option<String>,
    autotests: Option<String>,
    autobenches: Option<String>,
    resolver: Option<String>,

    // [dependencies]
    dependencies: Option<String>,
    dev_dependencies: Option<String>,
    build_dependencies: Option<String>,

    // [badges]
    badges: Option<String>,

    // [features]
    features: Option<String>,

    // [workspace]
    workspace: Option<String>,
}

// Concrete Decorators call the wrapped object and alter its result in some
// way.
pub struct CargoToml {
    component: Rc<dyn Component>,
}

impl Decorator for CargoToml {
    fn new(component: Rc<dyn Component>) -> Self {
        CargoToml { component }
    }
}

impl Component for CargoToml {
    fn parse_dependency(&self, key: &String, value: &Value) -> Result<Dependency, Error> {
        if value.is_string() {
            return Ok(Dependency {
                name: key.to_string(),
                version: Some(value.to_string()),
            });
        } else if value.is_object() {
            let version = value["version"].as_str();

            // there must be a better way to do the below but my rust skills are 🥴
            // future us: improve this!
            if version.is_some() {
                return Ok(Dependency {
                    name: key.to_string(),
                    version: Some(version.unwrap().to_string()),
                });
            } else {
                return Ok(Dependency {
                    name: key.to_string(),
                    version: None,
                });
            }
        }

        Err(anyhow!(
            "Could not parse dependency! Key: {}, Value: {}",
            key,
            value
        ))
    }

    fn convert(&self, file_contents: String) -> Result<ConverterOutput, Error> {
        let mut output = ConverterOutput::empty();

        let json: Value = toml::from_str(&file_contents.as_str()).unwrap();

        output.name = Some(json["package"]["name"].to_string());
        output.description = Some(json["package"]["description"].to_string());
        output.version = Some(json["package"]["version"].to_string());
        output.authors = json["package"]["authors"]
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());
        output.license = Some(json["package"]["license"].to_string());
        output.license_file = Some(json["package"]["license-file"].to_string());
        output.keywords = json["package"]["keywords"]
            .as_array()
            .map(|v| v.iter().map(|s| s.to_string()).collect());
        output.homepage_url = Some(json["package"]["homepage"].to_string());
        output.repository_url = Some(json["package"]["repository"].to_string());

        output.dependencies = json["dependencies"]
            .as_object()
            .map(|v| v.iter().map(|(k, v)| self.parse_dependency(k, v)).collect());

        output.dev_dependencies = json["dev-dependencies"]
            .as_object()
            .map(|v| v.iter().map(|(k, v)| self.parse_dependency(k, v)).collect());

        output.build_dependencies = json["build-dependencies"]
            .as_object()
            .map(|v| v.iter().map(|(k, v)| self.parse_dependency(k, v)).collect());

        Ok(output)
    }
}
