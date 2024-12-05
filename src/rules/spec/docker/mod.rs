pub mod component;
use component::detect_docker_component;

use crate::rules::register::register;
use crate::types::rule::{Rule, RuleDependency, RuleFiles};

pub fn register_docker() -> Result<(), String> {
    register(Rule {
        tech: String::from("docker"),
        name: String::from("Docker"),
        r#type: String::from("tool"),
        files: Some(RuleFiles::FilesArray {
            files: vec![
                String::from(".dockerignore"),
                String::from("Dockerfile"),
                String::from("docker-compose.yml"),
                String::from("docker-compose.yaml"),
            ],
        }),
        dependencies: Some(vec![RuleDependency {
            r#type: String::from("githubAction"),
            name: Some(String::from("docker/login-action")),
            ..Default::default()
        }]),
        detect: Some(vec![detect_docker_component]),
        ..Default::default()
    })
}
