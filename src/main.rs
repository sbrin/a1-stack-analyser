use std::collections::HashMap;

mod payload;
mod provider;
mod rules;
mod types;

use payload::payload::Payload;
use provider::{base::BaseProvider, fake::FakeProvider};
use rules::{
    loader::load_all_rules,
    register::{register_all, REGISTERED_RULES},
};

pub struct AnalyserOptions<P: BaseProvider> {
    provider: P,
}

pub fn analyser<P: BaseProvider>(opts: AnalyserOptions<P>) -> Payload {
    let provider = opts.provider;
    let mut pl = Payload::new("main", "/");

    pl.recurse(&provider, &provider.base_path());

    pl
}

fn main() {
    // println!(
    //     "{:?}",
    //     analyser(AnalyserOptions {
    //         provider: FakeProvider::new(
    //             HashMap::from_iter([("/".to_string(), vec![])]),
    //             HashMap::new(),
    //         ),
    //     })
    // );
}

// ... existing code ...

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_should_not_find_anything() {
        register_all();
        load_all_rules(&REGISTERED_RULES.lock().unwrap());

        let provider = FakeProvider::new(
            HashMap::from_iter([("/".to_string(), vec![])]),
            HashMap::new(),
        );

        let result = analyser(AnalyserOptions { provider });

        assert_eq!(result.name, "main");
        assert!(result.path.contains("/"));
        assert!(result.tech.is_none());
        assert!(result.languages.is_empty());
        assert!(result.childs.is_empty());
        assert!(result.techs.is_empty());
        assert!(result.dependencies.is_empty());
        assert!(result.edges.is_empty());
        assert!(result.reason.is_empty());
    }

    #[test]
    fn test_should_register_component_of_same_tech() {
        let docker_compose = r#"version: '3'
services:
  db:
    container_name: db
    image: postgres:15.1-alpine
    ports:
      - '5432:5432'
    environment:
      - POSTGRES_PASSWORD=postgres"#;

        register_all();
        load_all_rules(&REGISTERED_RULES.lock().unwrap());

        let provider = FakeProvider::new(
            HashMap::from_iter([(
                "/".to_string(),
                vec!["package.json".to_string(), "docker-compose.yml".to_string()],
            )]),
            HashMap::from_iter([
                (
                    "/docker-compose.yml".to_string(),
                    docker_compose.to_string(),
                ),
                (
                    "/package.json".to_string(),
                    r#"{ "name": "test", "dependencies": {"pg": "1.0.0"}}"#.to_string(),
                ),
            ]),
        );

        let result = analyser(AnalyserOptions { provider });
        // println!("analyser result: {:#?}", result);
        // Add assertions based on your actual implementation
        assert_eq!(result.name, "main");
        assert!(result.path.contains("/"));
        assert_eq!(result.childs.len(), 2); // Should have two child nodes
    }
}
