use serde_json::{json, Value};
use std::rc::Rc;

use anyhow::Error;

use super::{Component, Decorator};

// Concrete Decorators call the wrapped object and alter its result in some
// way.
struct CargoToml {
    component: Rc<dyn Component>,
}

impl Decorator for CargoToml {
    fn new(component: Rc<dyn Component>) -> Self {
        CargoToml { component }
    }
}

impl Component for CargoToml {
    fn convert(&self, file_contents: String) -> Result<Value, Error> {
        Ok(json!({}))
    }
}
