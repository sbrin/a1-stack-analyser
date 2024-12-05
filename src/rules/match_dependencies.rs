use crate::rules::loader::DEPENDENCIES;
use std::collections::HashMap;

pub fn match_dependencies(pkgs: &[String], dep_type: &str) -> HashMap<String, Vec<String>> {
    let mut matched: HashMap<String, Vec<String>> = HashMap::new();
    let dependencies = DEPENDENCIES.lock().unwrap();

    if let Some(type_deps) = dependencies.get(dep_type) {
        for dep in pkgs {
            println!("Match deps: {:?} - {:?}", pkgs, type_deps);
            for matcher in type_deps {
                if matcher.match_pattern.is_match(dep) {
                    matched
                        .entry(matcher.tech.clone())
                        .or_insert_with(Vec::new)
                        .push(format!("matched: {}", matcher.tech));
                }
            }
        }
    }
    println!("matched deps: {:?}", matched);
    matched
}

// ... existing code ...

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::loader::{self, DependencyMatcher};
    use crate::rules::register;
    use crate::types::rule::{Rule, RuleDependency};
    use regex::Regex;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            // Initialize if needed
        });
    }

    fn clear_test_storage() {
        DEPENDENCIES
            .lock()
            .unwrap()
            .iter_mut()
            .for_each(|(_, v)| v.clear());
    }

    #[test]
    #[serial_test::serial]
    fn test_match_dependencies_basic() {
        initialize();
        clear_test_storage();

        // Create and register a test rule
        let rule = Rule {
            name: "test-rule".to_string(),
            tech: "test-tech".to_string(),
            r#type: "language".to_string(),
            dependencies: Some(vec![RuleDependency {
                name: Some("test-dep".to_string()),
                r#type: "npm".to_string(),
                ..Default::default()
            }]),
            ..Default::default()
        };

        register::register(rule.clone()).unwrap();
        loader::load_one(&rule);

        // Test matching dependencies
        let pkgs = vec!["test-dep".to_string()];
        let result = match_dependencies(&pkgs, "npm");

        assert!(!result.is_empty());
        assert_eq!(result.len(), 1);
        assert!(result.contains_key("test-tech"));
    }

    #[test]
    #[serial_test::serial]
    fn test_match_dependencies_no_matches() {
        initialize();
        clear_test_storage();

        let pkgs = vec!["non-existent-dep".to_string()];
        let result = match_dependencies(&pkgs, "npm");

        assert!(result.is_empty());
    }

    #[test]
    #[serial_test::serial]
    fn test_match_dependencies_multiple_matches() {
        initialize();
        clear_test_storage();

        // Manually insert test matchers
        let mut dependencies = DEPENDENCIES.lock().unwrap();
        if let Some(npm_deps) = dependencies.get_mut("npm") {
            npm_deps.push(DependencyMatcher {
                match_pattern: Regex::new("^dep-1$").unwrap(),
                tech: "tech-1".to_string(),
            });
            npm_deps.push(DependencyMatcher {
                match_pattern: Regex::new("^dep-2$").unwrap(),
                tech: "tech-2".to_string(),
            });
        }
        drop(dependencies);

        let pkgs = vec![
            "dep-1".to_string(),
            "dep-2".to_string(),
            "dep-3".to_string(),
        ];
        let result = match_dependencies(&pkgs, "npm");

        assert_eq!(result.len(), 2);
        assert!(result.contains_key("tech-1"));
        assert!(result.contains_key("tech-2"));
    }

    #[test]
    #[serial_test::serial]
    fn test_match_dependencies_invalid_type() {
        initialize();
        clear_test_storage();

        let pkgs = vec!["some-dep".to_string()];
        let result = match_dependencies(&pkgs, "invalid-type");

        assert!(result.is_empty());
    }
}
