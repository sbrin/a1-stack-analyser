use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub enum LangType {
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "markup")]
    Markup,
    #[serde(rename = "programming")]
    Programming,
    #[serde(rename = "prose")]
    Prose,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LangListItem {
    pub extensions: Vec<String>,
    pub group: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub lang_type: LangType,
}

const LANGUAGES_JSON: &str = include_str!("languages.json");

pub fn raw_list() -> Vec<LangListItem> {
    serde_json::from_str(LANGUAGES_JSON).expect("Failed to parse languages.json")
}

pub fn languages() -> Vec<LangListItem> {
    raw_list()
        .into_iter()
        .filter(|l| matches!(l.lang_type, LangType::Programming))
        .collect()
}

pub fn others() -> Vec<LangListItem> {
    raw_list()
        .into_iter()
        .filter(|l| !matches!(l.lang_type, LangType::Programming))
        .collect()
}

/// Detect language of a file at this level.
pub fn detect_lang(filename: &str) -> Option<LangListItem> {
    let ext = Path::new(filename)
        .extension()
        .and_then(|os_str| os_str.to_str())
        .map(|s| format!(".{}", s));

    let ext = match ext {
        Some(e) => e,
        None => return None,
    };

    languages()
        .into_iter()
        .find(|lang| lang.extensions.contains(&ext))
        .or_else(|| {
            others()
                .into_iter()
                .find(|lang| lang.extensions.contains(&ext))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_list() {
        let list = raw_list();
        assert!(!list.is_empty());

        // Test first item
        let first = &list[0];
        assert_eq!(first.name, "1C Enterprise");
        assert_eq!(first.extensions, vec![".bsl", ".os"]);
        assert!(matches!(first.lang_type, LangType::Programming));
    }

    #[test]
    fn test_languages() {
        let langs = languages();
        // All items should be programming languages
        assert!(langs
            .iter()
            .all(|l| matches!(l.lang_type, LangType::Programming)));
        // Should contain 1C Enterprise
        assert!(langs.iter().any(|l| l.name == "1C Enterprise"));
    }

    #[test]
    fn test_others() {
        let other_langs = others();
        // No programming languages should be present
        assert!(other_langs
            .iter()
            .all(|l| !matches!(l.lang_type, LangType::Programming)));
        // Should contain 2-Dimensional Array
        assert!(other_langs.iter().any(|l| l.name == "2-Dimensional Array"));
    }

    #[test]
    fn test_detect_lang() {
        // Test programming language detection
        let bsl_file = detect_lang("test.bsl");
        assert!(bsl_file.is_some());
        assert_eq!(bsl_file.unwrap().name, "1C Enterprise");

        // Test data format detection
        let data_file = detect_lang("test.2da");
        assert!(data_file.is_some());
        assert_eq!(data_file.unwrap().name, "2-Dimensional Array");

        // Test non-existent extension
        let invalid_file = detect_lang("test.nonexistent");
        assert!(invalid_file.is_none());

        // Test file without extension
        let no_ext = detect_lang("testfile");
        assert!(no_ext.is_none());
    }
}
