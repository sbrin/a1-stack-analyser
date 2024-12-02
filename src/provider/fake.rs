use std::collections::HashMap;
use std::path::Path;
use async_trait::async_trait;

use super::base::{BaseProvider, ProviderFile, FileType};

pub struct FakeProvider {
    base_path: String,
    paths: HashMap<String, Vec<String>>,
    files: HashMap<String, String>,
}

impl FakeProvider {
    pub fn new(paths: HashMap<String, Vec<String>>, files: HashMap<String, String>) -> Self {
        Self {
            base_path: "/".to_string(),
            paths,
            files,
        }
    }
}

#[async_trait]
impl BaseProvider for FakeProvider {
    fn base_path(&self) -> String {
        self.base_path.clone()
    }

    async fn list_dir(&self, path_relative: &str) -> Vec<ProviderFile> {
        match self.paths.get(path_relative) {
            None => vec![],
            Some(files) => {
                let mut result = files.iter()
                    .map(|file| {
                        let is_dir = file.ends_with('/');
                        let name = if is_dir {
                            file[..file.len()-1].to_string()
                        } else {
                            file.clone()
                        };
                        
                        ProviderFile {
                            name,
                            file_type: if is_dir { FileType::Dir } else { FileType::File },
                            fp: Path::new(path_relative).join(file).to_string_lossy().to_string(),
                        }
                    })
                    .collect::<Vec<_>>();
                result.sort_by(|a, b| a.name.cmp(&b.name));
                result
            }
        }
    }

    async fn open(&self, path_relative: &str) -> Option<String> {
        self.files.get(path_relative).cloned()
    }

}