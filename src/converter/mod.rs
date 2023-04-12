//! The converter module is a decorator pattern implementation which allows us to to convert any given (supported)
//! config file to a common JSON object.
//!
//! Resources:
//! - https://refactoring.guru/design-patterns/decorator
//! - https://github.com/lpxxn/rust-design-pattern/blob/master/structural/decorator.rs

use anyhow::Error;
use serde_json::{json, Value};
use std::rc::Rc;

mod cargo_toml;
mod package_json;

// The base Component trait defines operations that can be altered by
// decorators.
trait Component {
    /// Convert the file to a JSON object
    fn convert(&self, file_contents: String) -> Result<Value, Error>;
}

// Concrete Components provide default implementations of the operations.
// There might be several variations of these classes.
struct ConcreteComponent {}

impl Component for ConcreteComponent {
    fn convert(&self, file_contents: String) -> Result<Value, Error> {
        Ok(json!({}))
    }
}

// The base Decorator class follows the same interface as the other
// components. The primary purpose of this class is to define the wrapping
// interface for all concrete decorators. The default implementation of the
// wrapping code might include a field for storing a wrapped component and
// the means to initialize it.
trait Decorator: Component {
    fn new(component: Rc<dyn Component>) -> Self;
}

/// Converts a given config file to a common JSON object
struct Converter {
    path: String,
}

impl Converter {
    fn new(path: String) -> Self {
        Converter { path }
    }

    fn convert<T: Component>(&self, Component: &T) -> Result<Value, Error> {
        let contents = std::fs::read_to_string(&self.path)
            .expect("Should have been able to read the template file");

        Component.convert(contents)
    }
}
