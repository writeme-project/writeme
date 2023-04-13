//! The converter module is a decorator pattern implementation which allows us to to convert any given (supported)
//! config file to a common OUTPUT object with the relevant information needed to generate a README file.
//!
//! Resources:
//! - https://refactoring.guru/design-patterns/decorator
//! - https://github.com/lpxxn/rust-design-pattern/blob/master/structural/decorator.rs

use anyhow::Error;
use serde_json::Value;
use std::rc::Rc;

pub mod cargo_toml;
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
}

// The base Decorator class follows the same interface as the other
// components. The primary purpose of this class is to define the wrapping
// interface for all concrete decorators. The default implementation of the
// wrapping code might include a field for storing a wrapped component and
// the means to initialize it.
pub trait Decorator: Component {
    fn new(component: Rc<dyn Component>) -> Self;
}

#[derive(Debug)]
/// Holds the information of a dependency in a config file
pub struct Dependency {
    /// The name of the dependency
    name: String,

    /// The version of the dependency, it may be missing!
    version: Option<String>,
}

#[derive(Debug)]
/// A contributor to the project
pub struct Contributor {
    name: Option<String>,
    email: Option<String>,
    url: Option<String>,
}

#[derive(Debug)]
/// The output object that will be returned from each converter implementation regardless of the config file provided
pub struct ConverterOutput {
    name: Option<String>,
    description: Option<String>,
    version: Option<String>,
    contributors: Option<Vec<Contributor>>,
    license: Option<String>,
    keywords: Option<Vec<String>>,
    repository_url: Option<String>,
    homepage_url: Option<String>,

    /// dependencies of the project
    dependencies: Option<Vec<Dependency>>,

    /// dev dependencies of the project
    dev_dependencies: Option<Vec<Dependency>>,

    /// build dependencies of the project, not every config file supports this
    build_dependencies: Option<Vec<Dependency>>,
}

impl ConverterOutput {
    /// Creates a new empty output object
    fn empty() -> Self {
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
        }
    }
}

/// Converts a given config file to a common Output object
pub struct Converter {
    path: String,
}

impl Converter {
    pub fn new(path: String) -> Self {
        Converter { path }
    }

    pub fn convert<T: Component>(&self, component: &T) -> Result<ConverterOutput, Error> {
        let contents = std::fs::read_to_string(&self.path)
            .expect("Should have been able to read the template file");

        component.convert(contents)
    }
}
