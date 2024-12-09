use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    payload::payload::Payload,
    provider::base::{BaseProvider, ProviderFile},
    rules::match_dependencies::match_dependencies,
};

const FILES: [&str; 1] = ["package.json"];

#[derive(Debug, Deserialize)]
struct PackageJson {
    name: Option<String>,
    dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: Option<HashMap<String, String>>,
}

pub fn detect_node_component(
    files: &Vec<ProviderFile>,
    provider: &dyn BaseProvider,
) -> Result<Payload, bool> {
    for file in files {
        if !FILES.contains(&file.name.as_str()) {
            continue;
        }

        let content = match provider.open(&file.fp) {
            Some(content) => content,
            None => continue,
        };

        let package_json: PackageJson = match serde_json::from_str(&content) {
            Ok(manifest) => manifest,
            Err(e) => {
                println!("Failed to parse package.json: {} - {}", file.fp, e);
                continue;
            }
        };

        let name = match package_json.name {
            Some(name) => name,
            None => continue,
        };

        let mut pl = Payload::new(&name, &file.fp);

        // Collect all dependencies into a single HashMap
        let mut deps = HashMap::new();
        if let Some(ref deps_map) = package_json.dependencies {
            deps.extend(deps_map.clone());
        }
        if let Some(ref dev_deps) = package_json.dev_dependencies {
            deps.extend(dev_deps.clone());
        }

        // Match dependencies and create flattened dependency list
        let techs = match_dependencies(
            &deps.keys().map(|s| s.to_string()).collect::<Vec<_>>(),
            "npm",
        );

        let deps_flatten: Vec<Vec<String>> = deps
            .into_iter()
            .map(|(name, version)| vec!["npm".to_string(), name.to_string(), version])
            .collect();

        pl.add_techs(&techs);
        pl.dependencies = deps_flatten;

        return Ok(pl);
    }

    Err(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::fake::FakeProvider;

    #[test]
    fn test_detect_node_component() {
        let package_content = r#"
{
    "name": "test_project",
    "version": "1.0.0",
    "dependencies": {
        "express": "^4.17.1",
        "react": "17.0.2"
    },
    "devDependencies": {
        "jest": "^27.0.0",
        "typescript": "4.5.4"
    }
}"#;

        // Setup FakeProvider with package.json
        let mut files = HashMap::new();
        files.insert("package.json".to_string(), package_content.to_string());

        let mut paths = HashMap::new();
        paths.insert("/".to_string(), vec!["package.json".to_string()]);

        let provider = FakeProvider::new(paths, files);

        let files = vec![ProviderFile {
            name: "package.json".to_string(),
            fp: "package.json".to_string(),
            file_type: crate::provider::base::FileType::File,
        }];

        // Test the function
        let result = detect_node_component(&files, &provider).unwrap();

        // Verify results
        assert_eq!(result.name, "test_project");
        assert!(result.path.contains("package.json"));

        // Verify dependencies are correctly parsed
        let deps = result.dependencies;
        assert!(deps.contains(&vec![
            "npm".to_string(),
            "express".to_string(),
            "^4.17.1".to_string()
        ]));
        assert!(deps.contains(&vec![
            "npm".to_string(),
            "react".to_string(),
            "17.0.2".to_string()
        ]));
        assert!(deps.contains(&vec![
            "npm".to_string(),
            "jest".to_string(),
            "^27.0.0".to_string()
        ]));
        assert!(deps.contains(&vec![
            "npm".to_string(),
            "typescript".to_string(),
            "4.5.4".to_string()
        ]));
    }
}
