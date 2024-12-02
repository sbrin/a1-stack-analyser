use crate::rules::register::register;
use crate::types::rule::{Rule, RuleDependency};

pub fn register_react() -> Result<(), String> {
    register(Rule {
        tech: String::from("react"),
        name: String::from("React"),
        r#type: String::from("language"),
        dependencies: Some(vec![RuleDependency {
            r#type: String::from("npm"),
            name: Some(String::from("react")),
            ..Default::default()
        }]),
        extensions: Some(vec![String::from("tsx"), String::from("jsx")]),
        ..Default::default()
    })
}
