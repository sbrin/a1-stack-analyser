use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use crate::rules::match_files::{match_extensions, match_files, match_files_regex};
use crate::rules::register::REGISTERED_RULES;
use crate::types::rule::{
    ComponentMatcher, ExtensionMatcher, Rule, RuleDependency, RuleFiles, TechMatcher,
};

lazy_static! {
    pub static ref RULES_TECHS: Mutex<Vec<TechMatcher>> = Mutex::new(Vec::new());
    pub static ref RULES_EXTENSIONS: Mutex<Vec<ExtensionMatcher>> = Mutex::new(Vec::new());
    pub static ref RULES_COMPONENTS: Mutex<Vec<ComponentMatcher>> = Mutex::new(Vec::new());

    pub static ref DEPENDENCIES: Mutex<HashMap<String, Vec<DependencyMatcher>>> = Mutex::new({
        let mut m = HashMap::new();
        // Initialize with empty vectors for each supported dependency type
        m.insert("terraform.resource".to_string(), Vec::new());
        m.insert("deno".to_string(), Vec::new());
        m.insert("docker".to_string(), Vec::new());
        m.insert("golang".to_string(), Vec::new());
        m.insert("npm".to_string(), Vec::new());
        m.insert("php".to_string(), Vec::new());
        m.insert("python".to_string(), Vec::new());
        m.insert("ruby".to_string(), Vec::new());
        m.insert("rust".to_string(), Vec::new());
        m.insert("terraform".to_string(), Vec::new());
        m.insert("githubAction".to_string(), Vec::new());
        m
    });

    pub static ref RAW_LIST: Mutex<Vec<RuleEntry>> = Mutex::new(Vec::new());
}

#[derive(Debug)]
pub struct DependencyMatcher {
    pub match_pattern: Regex,
    pub tech: String,
}

pub enum RuleEntry {
    Dependency { ref_rule: RuleDependency },
    Extension { ref_rule: Rule },
    File { ref_rule: Rule },
}

pub fn load_all_rules(registered_rules: &[Rule]) {
    for rule in registered_rules.iter() {
        load_one(&rule);
    }
    // println!("Loaded {} rules", registered_rules.len());
}

pub fn load_one(rule: &Rule) {
    // Handle dependencies
    if let Some(deps) = &rule.dependencies {
        for dep in deps {
            if let Some(name) = &dep.name {
                if name.is_empty() {
                    panic!(
                        "empty dependency name for {} ({} > {})",
                        rule.name, rule.r#type, rule.tech
                    );
                }

                let pattern = Regex::new(&format!("^{}$", name)).unwrap();
                let mut dependencies = DEPENDENCIES.lock().unwrap();
                if let Some(dep_list) = dependencies.get_mut(&dep.r#type) {
                    dep_list.push(DependencyMatcher {
                        match_pattern: pattern,
                        tech: rule.tech.clone(),
                    });
                }

                RAW_LIST.lock().unwrap().push(RuleEntry::Dependency {
                    ref_rule: dep.clone(),
                });
            }
        }
    }

    // Handle file matchers
    match &rule.files {
        Some(RuleFiles::FilesArray { files }) => {
            let tech = rule.tech.clone();
            let files_clone = files.clone();
            RULES_TECHS.lock().unwrap().push(Box::new(move |file_list| {
                match_files(&tech, &file_list, &files_clone, false).ok_or(false)
            }));
            RAW_LIST.lock().unwrap().push(RuleEntry::File {
                ref_rule: rule.clone(),
            });
        }
        Some(RuleFiles::FilesRegex { files, .. }) => {
            let tech = rule.tech.clone();
            let pattern = files.clone();
            RULES_TECHS.lock().unwrap().push(Box::new(move |file_list| {
                match_files_regex(&tech, &file_list, &pattern, false).ok_or(false)
            }));
            RAW_LIST.lock().unwrap().push(RuleEntry::File {
                ref_rule: rule.clone(),
            });
        }
        _ => {}
    }

    // Handle extensions
    if let Some(extensions) = &rule.extensions {
        let tech = rule.tech.clone();
        let exts: HashSet<String> = extensions.iter().cloned().collect();
        RULES_EXTENSIONS.lock().unwrap().push(Box::new(move |list| {
            match_extensions(&tech, &list, &exts).ok_or(false)
        }));
        RAW_LIST.lock().unwrap().push(RuleEntry::Extension {
            ref_rule: rule.clone(),
        });
    }

    // Handle component detectors
    if let Some(detectors) = &rule.detect {
        RULES_COMPONENTS.lock().unwrap().extend(detectors.iter().cloned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::payload::payload::Payload;
    use crate::provider::base::{BaseProvider, FileType, ProviderFile};
    use crate::rules::register::{self};
    use std::collections::HashSet;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            // Currently empty as lazy_static! handles our initialization
            // Keep this function if you anticipate needing test-specific setup in the future
        });
    }

    fn clear_test_storage() {
        DEPENDENCIES.lock().unwrap().iter_mut().for_each(|(_, v)| v.clear());
        RULES_TECHS.lock().unwrap().clear();
        RULES_EXTENSIONS.lock().unwrap().clear();
        RULES_COMPONENTS.lock().unwrap().clear();
        RAW_LIST.lock().unwrap().clear();
        REGISTERED_RULES.lock().unwrap().clear();
    }

    #[test]
    #[serial_test::serial]
    fn test_load_one_with_dependency() {
        initialize();
        clear_test_storage();

        let rule = Rule {
            name: "test-rule".to_string(),
            tech: "python".to_string(),
            r#type: "language".to_string(),
            dependencies: Some(vec![RuleDependency {
                name: Some("django".to_string()),
                r#type: "python".to_string(),
                ..Default::default()
            }]),
            ..Default::default()
        };

        register::register(rule.clone()).unwrap();

        load_one(&rule);

        // Verify dependency was added
        let deps = DEPENDENCIES.lock().unwrap();
        let python_deps = deps.get("python").unwrap();
        assert_eq!(python_deps.len(), 1);
        assert_eq!(python_deps[0].tech, "python");
        assert!(python_deps[0].match_pattern.is_match("django"));
    }

    #[test]
    #[serial_test::serial]
    fn test_load_one_with_files_array() {
        initialize();
        clear_test_storage();

        let rule = Rule {
            name: "test-rule2".to_string(),
            tech: "nodejs".to_string(),
            r#type: "language".to_string(),
            dependencies: None,
            files: Some(RuleFiles::FilesArray {
                files: vec!["package.json".to_string()],
            }),
            ..Default::default()
        };

        register::register(rule.clone()).unwrap();

        load_one(&rule);

        // Verify file matcher was added
        let techs = RULES_TECHS.lock().unwrap();
        assert_eq!(techs.len(), 1);

        // Test the matcher
        let matcher = &techs[0];
        let test_files = vec![ProviderFile {
            name: "package.json".to_string(),
            file_type: FileType::File,
            fp: "package.json".to_string(),
        }];
        assert!(matcher(test_files).is_ok());
        let test_files = vec![ProviderFile {
            name: "other.file".to_string(),
            file_type: FileType::File,
            fp: "other.file".to_string(),
        }];
        assert!(matcher(test_files).is_err());
    }
    #[test]
    #[serial_test::serial]
    fn test_load_one_with_extensions() {
        initialize();
        clear_test_storage();

        let rule = Rule {
            name: "test-rule3".to_string(),
            tech: "php".to_string(),
            r#type: "language".to_string(),
            extensions: Some(vec![String::from("php")]),
            ..Default::default()
        };

        // Register the rule first so it's available in LIST_INDEXED
        register::register(rule.clone()).unwrap();

        load_one(&rule);

        // Verify extension matcher was added
        let extensions = RULES_EXTENSIONS.lock().unwrap();

        assert_eq!(extensions.len(), 1);

        // Test the matcher
        let matcher = &extensions[0];
        let mut test_files = HashSet::new();
        test_files.insert("php".to_string());
        assert!(matcher(test_files.clone()).is_ok());

        test_files.clear();
        test_files.insert("js".to_string());
        assert!(matcher(test_files).is_err());
    }

    #[test]
    #[serial_test::serial]
    fn test_load_one_with_component_detectors() {
        initialize();
        clear_test_storage();

        let detector: ComponentMatcher =
            |_files: &Vec<ProviderFile>, _provider: &dyn BaseProvider| Ok(Payload::new("test", ""));

        let rule = Rule {
            name: "test-rule".to_string(),
            tech: "web".to_string(),
            r#type: "framework".to_string(),
            detect: Some(vec![detector]),
            ..Default::default()
        };

        load_one(&rule);

        // Verify component detector was added
        let components = RULES_COMPONENTS.lock().unwrap();
        assert_eq!(components.len(), 1);
    }

    #[test]
    #[serial_test::serial]
    #[should_panic(expected = "empty dependency name")]
    fn test_load_one_with_empty_dependency_name() {
        initialize();
        clear_test_storage();

        let rule = Rule {
            name: "test-rule".to_string(),
            tech: "python".to_string(),
            r#type: "language".to_string(),
            dependencies: Some(vec![RuleDependency {
                name: Some("".to_string()),
                r#type: "python".to_string(),
                ..Default::default()
            }]),
            ..Default::default()
        };

        load_one(&rule);
    }

    #[test]
    #[serial_test::serial]
    fn test_load_all_rules() {
        initialize();
        clear_test_storage();

        // Create and register multiple test rules
        let rules = vec![
            Rule {
                name: "Test Rule 1".to_string(),
                tech: "tech1".to_string(),
                r#type: "language".to_string(),
                dependencies: Some(vec![RuleDependency {
                    name: Some("dep1".to_string()),
                    r#type: "python".to_string(),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            Rule {
                name: "Test Rule 2".to_string(),
                tech: "tech2".to_string(),
                r#type: "language".to_string(),
                files: Some(RuleFiles::FilesArray {
                    files: vec!["test.file".to_string()],
                }),
                ..Default::default()
            },
        ];

        // Register the rules
        for rule in rules {
            register::register(rule).unwrap();
        }

        // Call load_all_rules
        load_all_rules(&REGISTERED_RULES.lock().unwrap());

        // Verify rules were loaded
        let dependencies = DEPENDENCIES.lock().unwrap();
        let python_deps = dependencies.get("python").unwrap();
        assert_eq!(python_deps.len(), 1);
        assert_eq!(python_deps[0].tech, "tech1");

        let techs = RULES_TECHS.lock().unwrap();
        assert_eq!(techs.len(), 1);

        let raw_list = RAW_LIST.lock().unwrap();
        assert_eq!(raw_list.len(), 2);
    }
}
