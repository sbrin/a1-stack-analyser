use crate::{
    provider::base::{BaseProvider, FileType, IGNORED_DIVE_PATHS},
    rules::{loader::RULES_COMPONENTS, register::NAME_TO_KEY},
    types::rule::ComponentMatcher,
};
use std::collections::{HashMap, HashSet};

use super::{
    helpers::{find_hosting, find_implicit_component},
    languages::detect_lang,
    match_all_files::match_all_files,
};

#[derive(Debug, Clone)]
pub struct Payload {
    pub id: String,
    pub name: String,
    pub path: HashSet<String>,
    pub tech: Option<String>,
    pub languages: HashMap<String, i32>,
    pub childs: Vec<Payload>,
    pub techs: HashSet<String>,
    pub dependencies: Vec<Vec<String>>,
    pub edges: Vec<Edge>,
    pub parent: Option<Box<Payload>>,
    pub reason: HashSet<String>,
    pub components: Vec<ComponentMatcher>, // Add this new field
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub target: Box<Payload>,
    pub read: bool,
    pub write: bool,
}

impl Payload {
    pub fn new(name: &str, folder_path: &str) -> Self {
        let mut path = HashSet::new();
        path.insert(folder_path.to_string());
        let components = RULES_COMPONENTS.lock().unwrap().clone();
        Self {
            id: generate_id(),
            name: name.to_string(),
            path,
            tech: None,
            languages: HashMap::new(),
            childs: Vec::new(),
            techs: HashSet::new(),
            dependencies: Vec::new(),
            edges: Vec::new(),
            parent: None,
            reason: HashSet::new(),
            components,
        }
    }

    pub fn recurse<P: BaseProvider>(&mut self, provider: &P, file_path: &str) {
        let files = provider.list_dir(file_path);
        let mut ctx = self.clone();
        let components = self.components.clone();
        for rule in &components {
            let res = rule(&files, provider);
            let payloads = match res {
                Ok(payload) => vec![payload],
                Err(_) => {
                    println!("no match res");
                    continue;
                }
            };
            // println!("Recurse Payloads: {:?}", payloads);
            for pl in payloads {
                if pl.name != "virtual" {
                    ctx = pl.clone();
                    self.add_child(pl);
                } else {
                    self.combine_dependencies(&pl);
                    for child in pl.childs {
                        self.add_child(child);
                    }
                }
            }
        }

        // println!("Recurse provider: {:#?} - {:?}", &provider, ctx.id);

        let matched = match_all_files(&files, &provider.base_path());
        ctx.add_techs(&matched);

        // println!("Recurse files: {:?}", &files);
        // Handle directories separately
        for file in files {
            // println!("File {:#?}", file);
            if matches!(file.file_type, FileType::File) {
                ctx.detect_lang(&file.name);
                continue;
            }

            if IGNORED_DIVE_PATHS.contains(&file.name.as_str()) {
                continue;
            }

            // ... existing directory handling code ...
            let new_path = &file.fp;
            // println!("Checking directory: {}", new_path); // Debug print
            ctx.recurse(provider, new_path);
        }
    }

    /// Register a relationship between this Payload and another one.
    pub fn add_edges(&mut self, pl: Payload) {
        self.edges.push(Edge {
            target: Box::new(pl),
            read: true,
            write: true,
        });
    }

    /// Helper to add a lang entry to languages.
    pub fn add_lang(&mut self, name: &str, count: i32) {
        let entry = self.languages.entry(name.to_string()).or_insert(0);
        *entry += count;

        // Using the NAME_TO_KEY from register.rs
        let name_to_key = NAME_TO_KEY.lock().unwrap();
        if let Some(tech_key) = name_to_key.get(name) {
            if !self.techs.contains(tech_key) {
                self.add_tech(tech_key, &[]);
            }
        }
    }

    /// Register a parent of this Payload
    pub fn set_parent(&mut self, pl: Option<Payload>) {
        self.parent = pl.map(Box::new);
    }

    /// Detect language of a file at this level.
    pub fn detect_lang(&mut self, filename: &str) {
        // println!("detect lang {:?} - {:?}", filename, self.id);
        if let Some(lang) = detect_lang(filename) {
            let lang_name = lang.group.unwrap_or(lang.name);
            self.add_lang(&lang_name, 1);
        }
    }

    pub fn add_techs(&mut self, tech_map: &HashMap<String, Vec<String>>) {
        for (tech, reasons) in tech_map {
            self.add_tech(tech, reasons);
        }
    }

    pub fn add_tech(&mut self, tech: &str, reasons: &[String]) {
        self.techs.insert(tech.to_string());
        self.reason.extend(reasons.iter().cloned());

        // Note: These functions need to be implemented separately
        find_implicit_component(self, tech, reasons);
        find_hosting(self, tech);
    }

    pub fn add_child(&mut self, service: Payload) -> &mut Payload {
        // Find existing child with same name or tech
        let existing_idx = self.childs.iter().position(|s| {
            s.name == service.name
                || (s.tech.is_some() && service.tech.is_some() && s.tech == service.tech)
        });

        if let Some(idx) = existing_idx {
            let existing = &mut self.childs[idx];
            // Merge paths
            existing.path.extend(service.path);
            // Merge dependencies
            existing.dependencies.extend(service.dependencies);
            existing
        } else {
            // Add as new child
            self.childs.push(service);
            self.childs.last_mut().unwrap()
        }
    }

    fn combine_dependencies(&mut self, other: &Payload) {
        // Create a HashMap for deduplication using joined strings as keys
        let mut dedup: HashMap<String, Vec<String>> = HashMap::new();

        // Add existing dependencies
        for dep in &self.dependencies {
            dedup.insert(dep.join("_"), dep.clone());
        }

        // Add new dependencies from other payload
        for dep in &other.dependencies {
            dedup.insert(dep.join("_"), dep.clone());
        }

        // Update dependencies with deduplicated values
        self.dependencies = dedup.into_values().collect();
    }
}

fn generate_id() -> String {
    // Implement your ID generation logic here
    // For now, just return a placeholder
    uuid::Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        provider::fake::FakeProvider,
        rules::{
            loader::load_all_rules,
            register::{register_all, REGISTERED_RULES},
        },
    };

    #[test]
    fn test_recurse() {
        // Create a more realistic directory structure
        let mut paths = HashMap::new();
        paths.insert(
            "/test".to_string(),
            vec![
                "main.rs".to_string(),
                "Cargo.toml".to_string(),
                "src/".to_string(),
            ],
        );
        paths.insert(
            "/test/src/".to_string(),
            vec!["lib.rs".to_string(), "main.rs".to_string()],
        );

        // println!("Available paths in provider: {:?}", paths); // Debug

        register_all();
        load_all_rules(&REGISTERED_RULES.lock().unwrap());

        let files = HashMap::new();
        let provider = FakeProvider::new(paths, files);
        let mut payload = Payload::new("test_service", "/test");

        // println!("provider result: {:?}", provider);

        payload.recurse(&provider, "/test");

        // println!("payload result: {:?}", payload);

        // Add assertions to verify the recursion results
        assert!(
            !payload.languages.is_empty(),
            "Should detect languages from .rs files"
        );
        assert!(
            payload.languages.contains_key("Rust"),
            "Should detect Rust language"
        );
        assert!(
            payload.path.contains("/test"),
            "Should contain the base path"
        );

        // println!("Languages detected: {:?}", payload.languages);
        // println!("Techs detected: {:?}", payload.techs);
        // println!("Reasons detected: {:?}", payload.reason);
        // println!("childs detected: {:?}", payload.childs);
        assert!(false);
        // Verify file count - we expect 2 Rust files in total
        assert_eq!(
            *payload.languages.get("Rust").unwrap_or(&0),
            3,
            "Should count two Rust lang"
        );

        // Verify Cargo.toml was detected
        assert!(
            payload.techs.contains("rust"),
            "Should detect Rust tech from Cargo.toml"
        );
        assert!(
            payload.reason.contains("matched file: Cargo.toml"),
            "Should list Cargo.toml as a reason"
        );
    }

    #[test]
    fn test_add_edges() {
        let mut payload = Payload::new("service1", "/path1");
        let target = Payload::new("service2", "/path2");

        payload.add_edges(target.clone());

        assert_eq!(payload.edges.len(), 1);
        assert_eq!(payload.edges[0].target.name, "service2");
        assert!(payload.edges[0].read);
        assert!(payload.edges[0].write);
    }

    #[test]
    fn test_add_lang() {
        let mut payload = Payload::new("service1", "/path1");

        payload.add_lang("rust", 1);
        payload.add_lang("rust", 2);

        assert_eq!(payload.languages.len(), 1);
        assert_eq!(payload.languages.get("rust"), Some(&3));
    }

    #[test]
    fn test_set_parent() {
        let mut payload = Payload::new("child", "/child");
        let parent = Payload::new("parent", "/parent");

        payload.set_parent(Some(parent.clone()));

        assert!(payload.parent.is_some());
        assert_eq!(payload.parent.as_ref().unwrap().name, "parent"); // Changed this line

        // Test removing parent
        payload.set_parent(None);
        assert!(payload.parent.is_none());
    }

    #[test]
    fn test_detect_lang() {
        let mut payload = Payload::new("service1", "/path1");

        payload.detect_lang("main.rs");
        assert!(payload.languages.contains_key("Rust"));

        payload.detect_lang("script.py");
        assert!(payload.languages.contains_key("Python"));

        payload.detect_lang("unknown.xyz");
        assert_eq!(payload.languages.len(), 2); // Should not add unknown extensions
    }

    #[test]
    fn test_add_techs() {
        let mut payload = Payload::new("service1", "/path1");
        let mut tech_map = HashMap::new();
        tech_map.insert("rust".to_string(), vec!["Cargo.toml".to_string()]);
        tech_map.insert("docker".to_string(), vec!["Dockerfile".to_string()]);

        payload.add_techs(&tech_map);

        assert_eq!(payload.techs.len(), 2);
        assert!(payload.techs.contains("rust"));
        assert!(payload.techs.contains("docker"));
        assert_eq!(payload.reason.len(), 2);
        assert!(payload.reason.contains("Cargo.toml"));
        assert!(payload.reason.contains("Dockerfile"));
    }

    #[test]
    fn test_add_tech() {
        let mut payload = Payload::new("service1", "/path1");
        let reasons = vec!["package.json".to_string()];

        payload.add_tech("nodejs", &reasons);

        assert!(payload.techs.contains("nodejs"));
        assert!(payload.reason.contains("package.json"));
    }

    #[test]
    fn test_add_child() {
        // Create parent payload
        let mut parent = Payload::new("parent", "/parent");

        // Test adding new child
        let child1 = Payload::new("service1", "/path1");
        parent.add_child(child1);
        assert_eq!(parent.childs.len(), 1);
        assert_eq!(parent.childs[0].name, "service1");

        // Test merging with existing child
        let mut child2 = Payload::new("service1", "/path2");
        child2.dependencies.push(vec!["dep1".to_string()]);
        parent.add_child(child2);

        // Verify merge results
        assert_eq!(parent.childs.len(), 1); // Still only one child
        let merged_child = &parent.childs[0];
        assert_eq!(merged_child.name, "service1");
        assert_eq!(merged_child.path.len(), 2); // Both paths are present
        assert!(merged_child.path.contains("/path1"));
        assert!(merged_child.path.contains("/path2"));
    }

    #[test]
    fn test_combine_dependencies() {
        // Create two payloads
        let mut payload1 = Payload::new("service1", "/path1");
        let mut payload2 = Payload::new("service2", "/path2");

        // Add some dependencies to both payloads
        payload1.dependencies = vec![
            vec!["dep1".to_string(), "dep2".to_string()],
            vec!["dep3".to_string()],
        ];

        payload2.dependencies = vec![
            vec!["dep1".to_string(), "dep2".to_string()], // Duplicate dependency
            vec!["dep4".to_string()],
        ];

        // Combine dependencies
        payload1.combine_dependencies(&payload2);

        // Verify results
        assert_eq!(payload1.dependencies.len(), 3); // Should have 3 unique dependency sets

        // Convert dependencies to a set of strings for easier comparison
        let deps: HashSet<String> = payload1.dependencies.iter().map(|dep| dep.join("_")).collect();

        assert!(deps.contains("dep1_dep2"));
        assert!(deps.contains("dep3"));
        assert!(deps.contains("dep4"));
    }
}
