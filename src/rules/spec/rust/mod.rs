pub mod component;
use component::detect_rust_component;

use crate::rules::register::register;
use crate::types::rule::{Rule, RuleDependency, RuleFiles};

pub fn register_rust() -> Result<(), String> {
    register(Rule {
        tech: String::from("rust"),
        name: String::from("Rust"),
        r#type: String::from("language"),
        files: Some(RuleFiles::FilesArray {
            files: vec![String::from("Cargo.toml")],
        }),
        dependencies: Some(vec![
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("rust")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("cimg/rust")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("circleci/rust")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("rustlang/rust")),
                ..Default::default()
            },
        ]),
        detect: Some(vec![detect_rust_component]),
        ..Default::default()
    })
}
