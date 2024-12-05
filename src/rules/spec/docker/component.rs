use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    payload::payload::Payload,
    provider::base::{BaseProvider, ProviderFile},
    rules::match_dependencies::match_dependencies,
};

lazy_static::lazy_static! {
    static ref FILES_REG: Regex = Regex::new(r"^docker-compose(.*)?\.y(a)?ml$").unwrap();
}

#[derive(Debug, Deserialize)]
struct DockerComposeService {
    image: Option<String>,
    container_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DockerCompose {
    services: Option<HashMap<String, DockerComposeService>>,
}

pub fn detect_docker_component(
    files: &Vec<ProviderFile>,
    provider: &dyn BaseProvider,
) -> Result<Payload, bool> {
    for file in files {
        if !FILES_REG.is_match(&file.name) {
            continue;
        }

        let content = match provider.open(&file.fp) {
            Some(content) => content,
            None => continue,
        };

        let parsed: DockerCompose = match serde_yaml::from_str(&content) {
            Ok(manifest) => manifest,
            Err(e) => {
                println!("Failed to parse Docker file: {} - {}", file.fp, e);
                continue;
            }
        };

        let services = match parsed.services {
            Some(services) => services,
            None => {
                println!("Failed to parse Docker file - no services: {}", file.fp);
                continue;
            }
        };

        let mut pl = Payload::new("virtual", &file.fp);

        for (name, service) in services {
            let Some(image) = service.image else {
                continue;
            };

            if image.starts_with('$') {
                continue;
            }

            let parts: Vec<&str> = image.split(':').collect();
            let (image_name, image_version) = (parts[0], parts.get(1).unwrap_or(&"latest"));

            let matched = match_dependencies(&[image_name.to_string()], "docker");
            let (tech, reason) = if let Some(first_match) = matched.iter().next() {
                (
                    Some(first_match.0.clone()),
                    format!("matched: {}", first_match.0),
                )
            } else {
                (None, format!("matched: {}", image_name))
            };

            let child = Payload::new(&service.container_name.unwrap_or(name), &file.fp);

            let mut child = child;
            child.tech = tech;
            child.dependencies = vec![vec![
                "docker".to_string(),
                image_name.to_string(),
                image_version.to_string(),
            ]];
            child.reason = vec![reason].into_iter().collect();

            pl.add_child(child);
        }

        return Ok(pl);
    }

    Err(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::fake::FakeProvider;

    #[test]
    fn test_detect_docker_component() {
        let docker_content = r#"
services:
  web:
    image: nginx:1.19
  db:
    image: postgres:13
    container_name: my-postgres
"#;

        let mut files = HashMap::new();
        files.insert("docker-compose.yml".to_string(), docker_content.to_string());

        let mut paths = HashMap::new();
        paths.insert("/".to_string(), vec!["docker-compose.yml".to_string()]);

        let provider = FakeProvider::new(paths, files);

        let files = vec![ProviderFile {
            name: "docker-compose.yml".to_string(),
            fp: "docker-compose.yml".to_string(),
            file_type: crate::provider::base::FileType::File,
        }];

        let result = detect_docker_component(&files, &provider).unwrap();

        assert_eq!(result.name, "virtual");
        assert!(result.path.contains("docker-compose.yml"));

        let children = result.childs;
        assert_eq!(children.len(), 2);

        // Verify web service
        let web = children.iter().find(|c| c.name == "web").unwrap();
        assert!(web.dependencies.contains(&vec![
            "docker".to_string(),
            "nginx".to_string(),
            "1.19".to_string()
        ]));

        // Verify db service
        let db = children.iter().find(|c| c.name == "my-postgres").unwrap();
        assert!(db.dependencies.contains(&vec![
            "docker".to_string(),
            "postgres".to_string(),
            "13".to_string()
        ]));
    }
}
