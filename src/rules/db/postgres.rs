use crate::rules::register::register;
use crate::types::rule::{Rule, RuleDependency};

pub fn register_postgres() -> Result<(), String> {
    register(Rule {
        tech: String::from("postgresql"),
        name: String::from("Postgres"),
        r#type: String::from("db"),
        dependencies: Some(vec![
            RuleDependency {
                r#type: String::from("npm"),
                name: Some(String::from("pg")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("npm"),
                name: Some(String::from("postgres")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("npm"),
                name: Some(String::from("postgres-interval")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("npm"),
                name: Some(String::from("@opentelemetry/instrumentation-pg")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("npm"),
                name: Some(String::from("@mikro-orm/postgresql")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("npm"),
                name: Some(String::from("pg-connection-string")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("postgres")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("bitnami/postgresql")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("circleci/postgres")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("cimg/postgres")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("ubuntu/postgres")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("docker"),
                name: Some(String::from("bitnamicharts/postgresql")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("rust"),
                name: Some(String::from("postgres")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("ruby"),
                name: Some(String::from("pg")),
                ..Default::default()
            },
            RuleDependency {
                r#type: String::from("deno"),
                name: Some(String::from("/x/postgres@/")),
                example: Some(String::from("https://deno.land/x/postgres@v0.17.0/mod.ts")),
            },
            RuleDependency {
                r#type: String::from("php"),
                name: Some(String::from("martin-georgiev/postgresql-for-doctrine")),
                ..Default::default()
            },
        ]),
        ..Default::default()
    })
}
