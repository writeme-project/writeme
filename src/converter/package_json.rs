use serde_json::{json, Value};
use std::rc::Rc;

use anyhow::{Error, Ok};

use super::{Component, ConverterOutput, Decorator, Dependency};

// Concrete Decorators call the wrapped object and alter its result in some
// way.
struct PackageJson {
    component: Rc<dyn Component>,
}

impl Decorator for PackageJson {
    fn new(component: Rc<dyn Component>) -> Self {
        PackageJson { component }
    }
}

impl Component for PackageJson {
    fn convert(&self, file_contents: String) -> Result<ConverterOutput, Error> {
        let output = ConverterOutput::empty();

        Ok(output)
    }

    fn parse_dependency(&self, key: &String, value: &Value) -> Result<Dependency, Error> {
        Ok(Dependency {
            name: key.to_string(),
            version: Some(value.to_string()),
        })
    }
}
