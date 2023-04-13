//! The converter module is a decorator pattern implementation which allows us to to convert any given (supported)
//! config file to a common OUTPUT object with the relevant information needed to generate a README file.
//!
//! Resources:
//! - https://refactoring.guru/design-patterns/decorator
//! - https://github.com/lpxxn/rust-design-pattern/blob/master/structural/decorator.rs

use anyhow::Error;
use serde_json::{json, Value};
use std::rc::Rc;

pub mod cargo_toml;
mod package_json;

// The base Component trait defines operations that can be altered by
// decorators.
pub trait Component {
    /// Convert the file to a JSON object
    fn convert(&self, file_contents: String) -> Result<Output, Error>;
}

// Concrete Components provide default implementations of the operations.
// There might be several variations of these classes.
pub struct ConcreteComponent {}

impl Component for ConcreteComponent {
    fn convert(&self, file_contents: String) -> Result<Output, Error> {
        Ok(Output {
            name: None,
            description: None,
            version: None,
            authors: None,
            license: None,
            license_file: None,
            keywords: None,
            repository_url: None,
            homepage_url: None,
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
/// The output object that will be returned from each converter implementation regardless of the config file provided
pub struct Output {
    name: Option<String>,
    description: Option<String>,
    version: Option<String>,
    authors: Option<Vec<String>>,
    license: Option<String>,
    license_file: Option<String>,
    keywords: Option<Vec<String>>,
    repository_url: Option<String>,
    homepage_url: Option<String>,
}

impl Output {
    /// Creates a new empty output object
    fn empty() -> Self {
        Output {
            name: None,
            description: None,
            version: None,
            authors: None,
            license: None,
            license_file: None,
            keywords: None,
            repository_url: None,
            homepage_url: None,
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

    pub fn convert<T: Component>(&self, component: &T) -> Result<Output, Error> {
        let contents = std::fs::read_to_string(&self.path)
            .expect("Should have been able to read the template file");

        component.convert(contents)
    }
}
