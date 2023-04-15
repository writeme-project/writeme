//! The converter module is a decorator pattern implementation which allows us to to convert any given (supported)
//! config file to a common OUTPUT object with the relevant information needed to generate a README file.
//!
//! Resources:
//! - https://refactoring.guru/design-patterns/decorator
//! - https://github.com/lpxxn/rust-design-pattern/blob/master/structural/decorator.rs

use std::{fmt::Display, fs, path::Path};

use anyhow::{anyhow, Error};
use serde_json::Value;

pub mod cargo_toml;
pub mod composer_json;
pub mod package_json;

// The base Component trait defines operations that can be altered by
// decorators.
pub trait Component {
    /// Convert the config file to the common ConverterOutput object
    fn convert(&self, file_contents: String) -> Result<ConverterOutput, Error>;

    /// Parses a contributor from the config file since they are not always in the same format depending on the
    /// config file type
    fn parse_contributor(&self, contributor: &Value) -> Result<Contributor, Error>;

    /// Parses a dependency from the config file since they are not always in the same format depending on the
    /// config file type
    fn parse_dependency(&self, key: &String, value: &Value) -> Result<Dependency, Error>;

    fn parse_funding(&self, funding: &Value) -> Result<Funding, Error>;
}

// Concrete Components provide default implementations of the operations.
// There might be several variations of these classes.
pub struct ConcreteComponent {}

impl Component for ConcreteComponent {
    fn convert(&self, file_contents: String) -> Result<ConverterOutput, Error> {
        Ok(ConverterOutput {
            name: None,
            description: None,
            version: None,
            contributors: None,
            license: None,
            keywords: None,
            repository_url: None,
            homepage_url: None,
            dependencies: None,
            dev_dependencies: None,
            build_dependencies: None,
            funding: None,
        })
    }

    fn parse_contributor(&self, contributor: &Value) -> Result<Contributor, Error> {
        Ok(Contributor {
            name: contributor["name"].as_str().map(|s| s.to_string()),
            email: contributor["email"].as_str().map(|s| s.to_string()),
            url: contributor["url"].as_str().map(|s| s.to_string()),
        })
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

// The base Decorator class follows the same interface as the other
// components. The primary purpose of this class is to define the wrapping
// interface for all concrete decorators. The default implementation of the
// wrapping code might include a field for storing a wrapped component and
// the means to initialize it.
pub trait Decorator: Component {
    fn new(/* component: Rc<dyn Component> */) -> Self;
}

enum SupportedFile {
    ComposerJson,
    PackageJson,
    CargoToml,
}

impl SupportedFile {
    fn from_str(file_type: &str) -> Result<SupportedFile, Error> {
        match file_type {
            "composer.json" => Ok(SupportedFile::ComposerJson),
            "package.json" => Ok(SupportedFile::PackageJson),
            "Cargo.toml" => Ok(SupportedFile::CargoToml),
            _ => Err(anyhow!("Unsupported file type")),
        }
    }
}

#[derive(Debug, Clone)]
/// Holds the information of a dependency in a config file
pub struct Dependency {
    /// The name of the dependency
    name: String,

    /// The version of the dependency, it may be missing!
    version: Option<String>,
}

impl Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.name,
            self.version.as_ref().unwrap_or(&"None".to_string())
        )
    }
}

#[derive(Debug, Clone)]
/// This Vec variant is needed to implement the Display trait for the Vec<T> scenarios
///
/// Reference: https://stackoverflow.com/a/30633256/11802618
pub struct Dependencies(Vec<Dependency>);

impl Display for Dependencies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dependencies = String::new();
        for dependency in &self.0 {
            dependencies.push_str(&format!("{} ", dependency));
        }
        write!(f, "{}", dependencies)
    }
}

impl FromIterator<Dependency> for Dependencies {
    fn from_iter<I: IntoIterator<Item = Dependency>>(iter: I) -> Self {
        let mut dependencies = Vec::new();
        for dependency in iter {
            dependencies.push(dependency);
        }
        Dependencies(dependencies)
    }
}

#[derive(Debug, Clone)]
/// A contributor to the project
pub struct Contributor {
    name: Option<String>,
    email: Option<String>,
    url: Option<String>,
}

impl Display for Contributor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.name.as_ref().unwrap_or(&"None".to_string()),
            self.email.as_ref().unwrap_or(&"None".to_string()),
            self.url.as_ref().unwrap_or(&"None".to_string())
        )
    }
}

#[derive(Debug, Clone)]
/// This Vec variant is needed to implement the Display trait for the Vec<T> scenarios
///
/// Reference: https://stackoverflow.com/a/30633256/11802618
pub struct Contributors(Vec<Contributor>);

impl Display for Contributors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut contributors = String::new();
        for contributor in &self.0 {
            contributors.push_str(&format!("{} ", contributor));
        }
        write!(f, "{}", contributors)
    }
}

impl FromIterator<Contributor> for Contributors {
    fn from_iter<I: IntoIterator<Item = Contributor>>(iter: I) -> Self {
        let mut contributors = Vec::new();
        for contributor in iter {
            contributors.push(contributor);
        }
        Contributors(contributors)
    }
}

#[derive(Debug, Clone)]
/// How a project could be funded
pub struct Funding {
    f_type: Option<String>,
    url: Option<String>,
}

impl Display for Funding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.f_type.as_ref().unwrap_or(&"None".to_string()),
            self.url.as_ref().unwrap_or(&"None".to_string())
        )
    }
}

#[derive(Debug, Clone)]
/// This Vec variant is needed to implement the Display trait for the Vec<T> scenarios
///
/// Reference: https://stackoverflow.com/a/30633256/11802618
pub struct Fundings(Vec<Funding>);

impl Display for Fundings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fundings = String::new();
        for funding in &self.0 {
            fundings.push_str(&format!("{} ", funding));
        }
        write!(f, "{}", fundings)
    }
}

impl FromIterator<Funding> for Fundings {
    fn from_iter<I: IntoIterator<Item = Funding>>(iter: I) -> Self {
        let mut fundings = Vec::new();
        for funding in iter {
            fundings.push(funding);
        }
        Fundings(fundings)
    }
}

#[derive(Debug, Clone)]
/// The output object that will be returned from each converter implementation regardless of the config file provided
pub struct ConverterOutput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub contributors: Option<Contributors>,
    pub license: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub repository_url: Option<String>,
    pub homepage_url: Option<String>,

    /// dependencies of the project
    pub dependencies: Option<Dependencies>,

    /// dev dependencies of the project
    pub dev_dependencies: Option<Dependencies>,

    /// build dependencies of the project, not every config file supports this
    pub build_dependencies: Option<Dependencies>,

    /// funding of the project, not every config file supports this (eg. Cargo.toml)
    pub funding: Option<Fundings>,
}

impl ConverterOutput {
    /// Creates a new empty output object
    pub fn empty() -> Self {
        ConverterOutput {
            name: None,
            description: None,
            version: None,
            contributors: None,
            license: None,
            keywords: None,
            repository_url: None,
            homepage_url: None,
            dependencies: None,
            dev_dependencies: None,
            build_dependencies: None,
            funding: None,
        }
    }
}

/// Converts a given config file to a common Output object
pub struct Converter;

impl Converter {
    pub fn new() -> Self {
        Converter {}
    }
    // pub fn convert<T: Component>(
    //     &self,
    //     path: &str,
    //     component: &T,
    // ) -> Result<ConverterOutput, Error> {
    //     let contents = std::fs::read_to_string(&path)
    //         .expect("Should have been able to read the template file");

    //     component.convert(contents)
    // }
    /// Gets the filename from a path string
    fn get_filename(path: &str) -> Option<&str> {
        let path = Path::new(path);

        path.file_name().and_then(|s| s.to_str())
    }

    pub fn convert(&self, path: &str) -> Result<ConverterOutput, Error> {
        let contents =
            fs::read_to_string(path).expect("Should have been able to read the template file");

        let config_file = match Converter::get_filename(path)
            .and_then(|filename| Some(SupportedFile::from_str(filename)))
        {
            Some(Ok(f)) => f,
            Some(Err(e)) => return Err(anyhow!(e)),
            None => return Err(anyhow!("File not found")),
        };

        let output = match config_file {
            SupportedFile::PackageJson => package_json::PackageJson::new().convert(contents),
            SupportedFile::ComposerJson => composer_json::ComposerJson::new().convert(contents),
            SupportedFile::CargoToml => cargo_toml::CargoToml::new().convert(contents),
            // _ => Err(anyhow!("File type not supported")),
        };

        output
    }
}
