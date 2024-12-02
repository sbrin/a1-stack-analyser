use crate::rules::register::register;
use crate::types::rule::{Rule, RuleDependency};

pub fn register_typescript() -> Result<(), String> {
    register(Rule {
        tech: String::from("typescript"),
        name: String::from("Typescript"),
        r#type: String::from("language"),
        dependencies: Some(vec![RuleDependency {
            r#type: String::from("npm"),
            name: Some(String::from("typescript")),
            ..Default::default()
        }]),

        extensions: Some(vec![String::from("ts"), String::from("tsx")]),
        example: Some(String::from("tsconfig.json")),
        ..Default::default()
    })
}
