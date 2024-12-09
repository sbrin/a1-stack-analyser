use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    payload::payload::Payload,
    provider::base::{BaseProvider, ProviderFile},
    rules::match_dependencies::match_dependencies,
};

const FILES: [&str; 1] = ["Cargo.toml"];

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Option<Package>,
    dependencies: Option<HashMap<String, Dependency>>,
    #[serde(rename = "dev-dependencies")]
    dev_dependencies: Option<HashMap<String, Dependency>>,
    #[serde(rename = "build-dependencies")]
    build_dependencies: Option<HashMap<String, Dependency>>,
    workspace: Option<Workspace>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    // Add other fields as needed
}

#[derive(Debug, Deserialize)]
struct Workspace {
    dependencies: Option<HashMap<String, Dependency>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Dependency {
    Simple(String),
    Detailed {
        version: Option<String>,
        path: Option<String>,
        git: Option<String>,
        branch: Option<String>,
        rev: Option<String>,
    },
}

pub fn detect_rust_component(
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

        let cargo_toml: CargoToml = match toml::from_str(&content) {
            Ok(manifest) => manifest,
            Err(e) => {
                println!("Failed to parse Cargo.toml: {} - {}", file.fp, e);
                continue;
            }
        };

        let mut pl = if let Some(package) = cargo_toml.package {
            Payload::new(&package.name, &file.fp)
        } else {
            Payload::new("virtual", &file.fp)
        };

        // Collect all dependencies into a single HashMap
        let mut deps = HashMap::new();

        if let Some(ref deps_map) = cargo_toml.dependencies {
            deps.extend(deps_map);
        }
        if let Some(ref dev_deps) = cargo_toml.dev_dependencies {
            deps.extend(dev_deps);
        }
        if let Some(ref build_deps) = cargo_toml.build_dependencies {
            deps.extend(build_deps);
        }
        if let Some(ref workspace) = cargo_toml.workspace {
            if let Some(ref workspace_deps) = workspace.dependencies {
                deps.extend(workspace_deps);
            }
        }

        // Match dependencies and create flattened dependency list
        let techs = match_dependencies(
            &deps.keys().map(|s| s.to_string()).collect::<Vec<_>>(),
            "rust",
        );

        let deps_flatten: Vec<Vec<String>> = deps
            .into_iter()
            .map(|(name, value)| {
                let version = match value {
                    Dependency::Simple(ref version) => version.to_string(),
                    Dependency::Detailed {
                        ref version,
                        ref path,
                        ref git,
                        ref branch,
                        ref rev,
                        ..
                    } => {
                        if let Some(path_str) = path {
                            let version_suffix =
                                version.as_ref().map(|v| format!(":{}", v)).unwrap_or_default();
                            format!("path:{}{}", path_str, version_suffix)
                        } else if let Some(git_str) = git {
                            let suffix = branch
                                .as_ref()
                                .or(rev.as_ref())
                                .map(|s| s.as_str())
                                .unwrap_or("latest");
                            format!("git:{}#{}", git_str, suffix)
                        } else {
                            version
                                .as_ref()
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "latest".to_string())
                        }
                    }
                };
                vec!["rust".to_string(), name.to_string(), version]
            })
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
    fn test_detect_rust_component() {
        // Example Cargo.toml content
        let cargo_content = r#"
[package]
name = "test_project"
version = "0.1.0"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
axum = { git = "https://github.com/tokio-rs/axum", branch = "main" }
local_dep = { path = "../local_dep" }

[dev-dependencies]
mockall = "0.11"

[build-dependencies]
cc = "1.0"

[workspace.dependencies]
workspace_dep = "1.0"
"#;

        // Setup FakeProvider with Cargo.toml
        let mut files = HashMap::new();
        files.insert("Cargo.toml".to_string(), cargo_content.to_string());

        let mut paths = HashMap::new();
        paths.insert("/".to_string(), vec!["Cargo.toml".to_string()]);

        let provider = FakeProvider::new(paths, files);

        let files = vec![ProviderFile {
            name: "Cargo.toml".to_string(),
            fp: "Cargo.toml".to_string(),
            file_type: crate::provider::base::FileType::File,
        }];

        // Test the function
        let result = detect_rust_component(&files, &provider).unwrap();

        // Verify results
        assert_eq!(result.name, "test_project");
        assert!(result.path.contains("Cargo.toml"));

        // Verify dependencies are correctly parsed
        let deps = result.dependencies;
        assert!(deps.contains(&vec![
            "rust".to_string(),
            "serde".to_string(),
            "1.0".to_string()
        ]));
        assert!(deps.contains(&vec![
            "rust".to_string(),
            "tokio".to_string(),
            "1.0".to_string()
        ]));
        assert!(deps.contains(&vec![
            "rust".to_string(),
            "axum".to_string(),
            "git:https://github.com/tokio-rs/axum#main".to_string()
        ]));
        assert!(deps.contains(&vec![
            "rust".to_string(),
            "local_dep".to_string(),
            "path:../local_dep".to_string()
        ]));
        assert!(deps.contains(&vec![
            "rust".to_string(),
            "mockall".to_string(),
            "0.11".to_string()
        ]));
        assert!(deps.contains(&vec![
            "rust".to_string(),
            "cc".to_string(),
            "1.0".to_string()
        ]));
        assert!(deps.contains(&vec![
            "rust".to_string(),
            "workspace_dep".to_string(),
            "1.0".to_string()
        ]));
    }
}
