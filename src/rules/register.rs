use crate::types::rule::Rule;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

// Static storage equivalents
lazy_static! {
    static ref REGISTERED_TECH: std::sync::Mutex<HashSet<String>> =
        std::sync::Mutex::new(HashSet::new());
    pub static ref REGISTERED_RULES: std::sync::Mutex<Vec<Rule>> =
        std::sync::Mutex::new(Vec::new());
    pub static ref LIST_INDEXED: std::sync::Mutex<HashMap<String, Rule>> =
        std::sync::Mutex::new(HashMap::new());
    pub static ref NAME_TO_KEY: std::sync::Mutex<HashMap<String, String>> =
        std::sync::Mutex::new(HashMap::new());
}

pub fn register(rule: Rule) -> Result<(), String> {
    let mut registered_tech = REGISTERED_TECH.lock().unwrap();

    if registered_tech.contains(&rule.tech) {
        return Err(format!("Already registered {}", rule.name));
    }

    registered_tech.insert(rule.tech.clone());

    REGISTERED_RULES.lock().unwrap().push(rule.clone());
    LIST_INDEXED.lock().unwrap().insert(rule.tech.clone(), rule.clone());
    NAME_TO_KEY.lock().unwrap().insert(rule.name.clone(), rule.tech);

    Ok(())
}

pub fn register_all() -> Result<(), String> {
    // Import all rule registration functions
    use crate::rules::*; // This will import any other rule modules you add

    // Register each rule
    let registrations = vec![
        analytics::amplitude::register_amplitude(),
        js::react::register_react(),
        js::typescript::register_typescript(),
        spec::nodejs::register_nodejs(),
        spec::rust::register_rust(),
        db::postgres::register_postgres(),
        spec::docker::register_docker(),
    ];

    // Check if any registration failed
    for result in registrations {
        result?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            // Currently empty as lazy_static! handles our initialization
            // Keep this function if you anticipate needing test-specific setup in the future
        });
    }

    fn clear_test_storage() {
        REGISTERED_TECH.lock().unwrap().clear();
        REGISTERED_RULES.lock().unwrap().clear();
        LIST_INDEXED.lock().unwrap().clear();
        NAME_TO_KEY.lock().unwrap().clear();
    }

    #[test]
    #[serial_test::serial]
    fn test_register_success() {
        initialize();
        clear_test_storage();
        // Create a test rule
        let rule = Rule {
            name: "Test Rule".to_string(),
            tech: "test_tech".to_string(),
            r#type: "test_type".to_string(),
            ..Default::default()
        };

        // Test successful registration
        let result = register(rule);
        assert!(result.is_ok());
    }

    #[test]
    #[serial_test::serial]
    fn test_register_duplicate() {
        initialize();
        clear_test_storage();
        // Create a test rule
        let rule1 = Rule {
            name: "Test Rule 2".to_string(),
            tech: "test_tech_2".to_string(),
            r#type: "test_type2".to_string(),
            ..Default::default()
        };

        let rule2 = Rule {
            name: "Test Rule 3".to_string(),
            tech: "test_tech_2".to_string(),
            r#type: "test_type3".to_string(),
            ..Default::default()
        };

        // First registration should succeed
        let result1 = register(rule1);
        assert!(result1.is_ok());

        // Second registration with same tech should fail
        let result2 = register(rule2);
        assert!(result2.is_err());
    }

    #[test]
    #[serial_test::serial]
    fn test_register_all() {
        initialize();
        clear_test_storage();
        // Test the register_all function
        let result = register_all();
        assert!(result.is_ok());

        // Verify that rules were actually registered
        let registered_rules = REGISTERED_RULES.lock().unwrap();
        println!("Registered rules: {:#?}", *registered_rules);

        assert!(!registered_rules.is_empty());
    }

    #[test]
    #[serial_test::serial]
    fn test_static_storage_variables() {
        initialize();
        clear_test_storage();

        // Create a test rule
        let rule = Rule {
            name: "Storage Test Rule".to_string(),
            tech: "storage_test_tech".to_string(),
            r#type: "test_type".to_string(),
            ..Default::default()
        };

        // Register the rule
        let result = register(rule.clone());
        assert!(result.is_ok());

        // Test REGISTERED_RULES
        {
            let registered_rules = REGISTERED_RULES.lock().unwrap();
            assert_eq!(registered_rules.len(), 1);
            assert_eq!(registered_rules[0].name, "Storage Test Rule");
            assert_eq!(registered_rules[0].tech, "storage_test_tech");
        }

        // Test LIST_INDEXED
        {
            let list_indexed = LIST_INDEXED.lock().unwrap();
            assert_eq!(list_indexed.len(), 1);
            assert!(list_indexed.contains_key("storage_test_tech"));
            let indexed_rule = list_indexed.get("storage_test_tech").unwrap();
            assert_eq!(indexed_rule.name, "Storage Test Rule");
        }

        // Test NAME_TO_KEY
        {
            let name_to_key = NAME_TO_KEY.lock().unwrap();
            assert_eq!(name_to_key.len(), 1);
            assert!(name_to_key.contains_key("Storage Test Rule"));
            assert_eq!(
                name_to_key.get("Storage Test Rule").unwrap(),
                "storage_test_tech"
            );
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_multiple_rules_storage() {
        initialize();
        clear_test_storage();

        // Create two test rules
        let rule1 = Rule {
            name: "First Rule".to_string(),
            tech: "tech1".to_string(),
            r#type: "test_type".to_string(),
            ..Default::default()
        };

        let rule2 = Rule {
            name: "Second Rule".to_string(),
            tech: "tech2".to_string(),
            r#type: "test_type".to_string(),
            ..Default::default()
        };

        // Register both rules
        register(rule1.clone()).unwrap();
        register(rule2.clone()).unwrap();

        // Verify all storage variables contain both rules
        {
            let registered_rules = REGISTERED_RULES.lock().unwrap();
            assert_eq!(registered_rules.len(), 2);
        }

        {
            let list_indexed = LIST_INDEXED.lock().unwrap();
            assert_eq!(list_indexed.len(), 2);
            assert!(list_indexed.contains_key("tech1"));
            assert!(list_indexed.contains_key("tech2"));
        }

        {
            let name_to_key = NAME_TO_KEY.lock().unwrap();
            assert_eq!(name_to_key.len(), 2);
            assert_eq!(name_to_key.get("First Rule").unwrap(), "tech1");
            assert_eq!(name_to_key.get("Second Rule").unwrap(), "tech2");
        }
    }
}
