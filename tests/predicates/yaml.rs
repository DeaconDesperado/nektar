use predicates::prelude::*;
use serde_yaml::Value as YamlValue;
use std::fmt;

pub struct IsYaml;

impl predicates::reflection::PredicateReflection for IsYaml {}

impl Predicate<str> for IsYaml {
    fn eval(&self, variable: &str) -> bool {
        serde_yaml::from_str::<YamlValue>(&variable).is_ok()
    }
}
impl fmt::Display for IsYaml {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var.is_json()")
    }
}
