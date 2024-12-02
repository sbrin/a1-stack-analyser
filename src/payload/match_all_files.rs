use crate::provider::base::ProviderFile;
use crate::rules::loader::{RULES_EXTENSIONS, RULES_TECHS};
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn match_all_files(files: &[ProviderFile], base_path: &str) -> HashMap<String, Vec<String>> {
    let mut matched: HashMap<String, Vec<String>> = HashMap::new();

    // Match files
    let rules_techs = RULES_TECHS.lock().unwrap();
    for rule in rules_techs.iter() {
        if let Ok((rule, path)) = rule(files.to_vec()) {
            let path_display = path.replace(base_path, "");
            matched.insert(rule.tech, vec![format!("matched file: {}", path_display)]);
        }
    }

    // Match extensions
    let mut exts = HashSet::new();
    for file in files {
        println!("File name: {}", file.name);
        let extension = Path::new(&file.name).extension();
        println!("Extension: {:?}", extension);

        if let Some(ext) = Path::new(&file.name).extension() {
            if let Some(ext_str) = ext.to_str() {
                exts.insert(ext_str.to_string());
            }
        }
    }

    let rules_extensions = RULES_EXTENSIONS.lock().unwrap();
    for rule in rules_extensions.iter() {
        if let Ok((rule, ext)) = rule(exts.clone()) {
            if !matched.contains_key(&rule.tech) {
                matched.insert(rule.tech, vec![format!("matched extension: {}", ext)]);
            }
        }
    }

    matched
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        provider::base::FileType,
        rules::{
            loader::load_all_rules,
            register::{register_all, LIST_INDEXED, REGISTERED_RULES},
        },
    };

    #[test]
    fn test_match_all_files() {
        register_all().unwrap();
        let registered_rules = {
            let rules = REGISTERED_RULES.lock().unwrap();
            rules.clone()
        };
        println!("Registered rules: {:#?}", registered_rules);

        load_all_rules(&registered_rules);

        // Empty case
        let res = match_all_files(&[], "/");
        assert!(res.is_empty());

        // Extension based matching
        let files = vec![ProviderFile {
            name: "index.tsx".to_string(),
            fp: "/index.tsx".to_string(),
            file_type: FileType::File,
        }];
        let res = match_all_files(&files, "/");

        println!("Matched rules: {:?}", res.keys().collect::<Vec<_>>());

        assert!(res.contains_key("typescript"));
        assert!(res.contains_key("react"));
        assert_eq!(
            res.get("typescript").unwrap(),
            &vec!["matched extension: tsx".to_string()]
        );
        assert_eq!(
            res.get("react").unwrap(),
            &vec!["matched extension: tsx".to_string()]
        );

        // File based matching
        let files = vec![ProviderFile {
            name: "package.json".to_string(),
            fp: "/package.json".to_string(),
            file_type: FileType::File,
        }];
        let res = match_all_files(&files, "/");
        assert!(res.contains_key("nodejs"));
        assert_eq!(
            res.get("nodejs").unwrap(),
            &vec!["matched file: package.json".to_string()]
        );
    }
}
