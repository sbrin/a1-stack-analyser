use crate::provider::base::ProviderFile;

pub fn match_all_files(
    files: Vec<ProviderFile>,
    base_path: &str,
) -> std::collections::HashMap<AllowedKeys, Vec<String>> {
    let mut matched: std::collections::HashMap<AllowedKeys, Vec<String>> =
        std::collections::HashMap::new();

    // Match files
    for rule in &rules_techs {
        if let Some(res) = rule(&files) {
            matched.insert(
                res[0].tech.clone(),
                vec![format!("matched file: {}", res[1].replace(base_path, ""))],
            );
        }
    }

    // Match extensions
    let mut exts = std::collections::HashSet::new();
    for file in &files {
        exts.insert(
            path::Path::new(&file.name)
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        );
    }
    for rule in &rules_extensions {
        if let Some(res) = rule(&exts) {
            if matched.contains_key(&res[0].tech) {
                continue;
            }
            matched.insert(
                res[0].tech.clone(),
                vec![format!("matched extension: {}", res[1])],
            );
        }
    }

    matched
}
