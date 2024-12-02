use std::path::Path;

#[derive(Debug, Clone)]
pub enum LangType {
    Data,
    Markup,
    Programming,
    Prose,
}

#[derive(Debug, Clone)]
pub struct LangListItem {
    extensions: Vec<String>,
    group: Option<String>,
    name: String,
    lang_type: LangType,
}

impl LangListItem {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }
}

// Source: https://github.com/github/linguist/blob/5a0c74277548122267d84283910abd5e0b89380e/lib/linguist/languages.yml#L1528
pub fn raw_list() -> Vec<LangListItem> {
    vec![
        LangListItem {
            extensions: vec![".bsl".to_string(), ".os".to_string()],
            group: None,
            name: "1C Enterprise".to_string(),
            lang_type: LangType::Programming,
        },
        LangListItem {
            extensions: vec![".2da".to_string()],
            group: None,
            name: "2-Dimensional Array".to_string(),
            lang_type: LangType::Data,
        },
    ]
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

// ... existing code ...

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
