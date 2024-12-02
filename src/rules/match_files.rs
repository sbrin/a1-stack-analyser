use crate::{provider::base::ProviderFile, rules::register::LIST_INDEXED, types::rule::Rule};
use regex::Regex;
use std::collections::HashSet;

pub fn match_files(
    key: &str,
    files: &[ProviderFile],
    matches: &[String],
    match_full_path: bool,
) -> Option<(Rule, String)> {
    for file in files {
        let name = if match_full_path {
            &file.fp
        } else {
            &file.name
        };

        if matches.contains(name) {
            return LIST_INDEXED
                .lock()
                .unwrap()
                .get(key)
                .cloned()
                .map(|rule| (rule, name.to_string()));
        }
    }

    None
}

pub fn match_files_regex(
    key: &str,
    files: &[ProviderFile],
    pattern: &Regex,
    match_full_path: bool,
) -> Option<(Rule, String)> {
    for file in files {
        let name = if match_full_path {
            &file.fp
        } else {
            &file.name
        };

        if pattern.is_match(name) {
            return LIST_INDEXED
                .lock()
                .unwrap()
                .get(key)
                .cloned()
                .map(|rule| (rule, name.to_string()));
        }
    }

    None
}

pub fn match_extensions(
    key: &str,
    list: &HashSet<String>,
    extensions: &HashSet<String>,
) -> Option<(Rule, String)> {
    for ext in list {
        if extensions.contains(ext) {
            return LIST_INDEXED
                .lock()
                .unwrap()
                .get(key)
                .cloned()
                .map(|rule| (rule, ext.to_string()));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::RegexBuilder;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            // Setup test data in LIST_INDEXED
            let mut list = LIST_INDEXED.lock().unwrap();
            list.insert(
                "test_tech".to_string(),
                Rule {
                    tech: "test_tech".to_string(),
                    name: "Test Tech".to_string(),
                    r#type: "test".to_string(),
                    ..Default::default()
                },
            );
        });
    }

    fn create_test_files() -> Vec<ProviderFile> {
        vec![
            ProviderFile {
                name: "test.js".to_string(),
                file_type: crate::provider::base::FileType::File,
                fp: "path/to/test.js".to_string(),
            },
            ProviderFile {
                name: "main.rs".to_string(),
                file_type: crate::provider::base::FileType::File,
                fp: "src/main.rs".to_string(),
            },
        ]
    }

    #[test]
    fn test_match_files_found() {
        initialize();
        let files = create_test_files();
        let matches = vec!["test.js".to_string()];

        let result = match_files("test_tech", &files, &matches, false);
        assert!(result.is_some());

        let (rule, matched_file) = result.unwrap();
        assert_eq!(rule.tech, "test_tech");
        assert_eq!(matched_file, "test.js");
    }

    #[test]
    fn test_match_files_not_found() {
        initialize();
        let files = create_test_files();
        let matches = vec!["nonexistent.file".to_string()];

        let result = match_files("test_tech", &files, &matches, false);
        assert!(result.is_none());
    }

    #[test]
    fn test_match_files_full_path() {
        initialize();
        let files = create_test_files();
        let matches = vec!["src/main.rs".to_string()];

        let result = match_files("test_tech", &files, &matches, true);
        assert!(result.is_some());

        let (rule, matched_file) = result.unwrap();
        assert_eq!(rule.tech, "test_tech");
        assert_eq!(matched_file, "src/main.rs");
    }

    #[test]
    fn test_match_files_regex() {
        initialize();
        let files = create_test_files();
        let pattern = RegexBuilder::new(r"test\.js$").build().unwrap();

        let result = match_files_regex("test_tech", &files, &pattern, false);
        assert!(result.is_some());

        let (rule, matched_file) = result.unwrap();
        assert_eq!(rule.tech, "test_tech");
        assert_eq!(matched_file, "test.js");
    }

    #[test]
    fn test_match_extensions() {
        initialize();
        let mut extensions = HashSet::new();
        extensions.insert("js".to_string());
        let mut list = HashSet::new();
        list.insert("js".to_string());

        let result = match_extensions("test_tech", &list, &extensions);
        assert!(result.is_some());

        let (rule, matched_ext) = result.unwrap();
        assert_eq!(rule.tech, "test_tech");
        assert_eq!(matched_ext, "js");
    }

    #[test]
    fn test_match_extensions_not_found() {
        initialize();
        let mut extensions = HashSet::new();
        extensions.insert("rs".to_string());
        let mut list = HashSet::new();
        list.insert("js".to_string());

        let result = match_extensions("test_tech", &list, &extensions);
        assert!(result.is_none());
    }
}
