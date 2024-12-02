use crate::rules::register::register;
use crate::types::rule::{Rule, RuleDependency, RuleFiles};

pub fn register_nodejs() -> Result<(), String> {
    register(Rule {
        tech: String::from("nodejs"),
        name: String::from("NodeJS"),
        r#type: String::from("language"),
        files: Some(RuleFiles::FilesArray {
            files: vec![String::from("package.json"), String::from(".nvmrc")],
        }),
        dependencies: Some(vec![
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("nodejs")),
                example: Some(String::from("nodejs:0.0.0")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("node")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("circleci/node")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("cimg/node")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("bitnami/node")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("okteto/node")),
                ..Default::default()
            },
        ]),
        ..Default::default()
    })
}
