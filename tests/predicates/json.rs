use predicates::prelude::*;
use serde_json::Value as JsonValue;
use std::fmt;

pub struct IsJson;

impl predicates::reflection::PredicateReflection for IsJson {}

impl Predicate<str> for IsJson {
    fn eval(&self, variable: &str) -> bool {
        serde_json::from_str::<JsonValue>(variable).is_ok()
    }
}
impl fmt::Display for IsJson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var.is_json()")
    }
}
