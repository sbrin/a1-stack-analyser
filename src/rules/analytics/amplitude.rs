use crate::rules::register::register;
use crate::types::rule::{Rule, RuleDependency};

pub fn register_amplitude() -> Result<(), String> {
    register(Rule {
        tech: String::from("amplitude"),
        name: String::from("Amplitude Analytics"),
        r#type: String::from("analytics"),
        dependencies: Some(vec![
            RuleDependency {
                r#type: String::from("npm"),
                name: Some(String::from("amplitude-js")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("npm"),
                name: Some(String::from("@amplitude/analytics-browser")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("php"),
                name: Some(String::from("zumba/amplitude-php")),
                ..Default::default()
            },
        ]),
        ..Default::default()
    })
}
