use std::collections::HashMap;
use std::path::Path;

use super::base::{BaseProvider, FileType, ProviderFile};

#[derive(Debug)]
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

impl BaseProvider for FakeProvider {
    fn base_path(&self) -> String {
        self.base_path.clone()
    }

    fn list_dir(&self, path_relative: &str) -> Vec<ProviderFile> {
        match self.paths.get(path_relative) {
            None => vec![],
            Some(files) => {
                let mut result = files
                    .iter()
                    .map(|file| {
                        let is_dir = file.ends_with('/');
                        let name = if is_dir {
                            file[..file.len() - 1].to_string()
                        } else {
                            file.clone()
                        };

                        ProviderFile {
                            name,
                            file_type: if is_dir {
                                FileType::Dir
                            } else {
                                FileType::File
                            },
                            fp: Path::new(path_relative).join(file).to_string_lossy().to_string(),
                        }
                    })
                    .collect::<Vec<_>>();
                result.sort_by(|a, b| a.name.cmp(&b.name));
                result
            }
        }
    }

    fn open(&self, path_relative: &str) -> Option<String> {
        self.files.get(path_relative).cloned()
    }
}

// ... existing code ...

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_provider() -> FakeProvider {
        let mut paths = HashMap::new();
        paths.insert(
            "/".to_string(),
            vec!["dir1/".to_string(), "file1.txt".to_string()],
        );
        paths.insert(
            "/dir1".to_string(),
            vec!["file2.txt".to_string(), "subdir/".to_string()],
        );

        let mut files = HashMap::new();
        files.insert("/file1.txt".to_string(), "content1".to_string());
        files.insert("/dir1/file2.txt".to_string(), "content2".to_string());

        FakeProvider::new(paths, files)
    }

    #[test]
    fn test_base_path() {
        let provider = setup_test_provider();
        assert_eq!(provider.base_path(), "/");
    }

    #[test]
    fn test_list_dir() {
        let provider = setup_test_provider();

        // Test root directory
        let root_files = provider.list_dir("/");
        assert_eq!(root_files.len(), 2);
        assert_eq!(root_files[0].name, "dir1");
        assert_eq!(root_files[0].file_type, FileType::Dir);
        assert_eq!(root_files[1].name, "file1.txt");
        assert_eq!(root_files[1].file_type, FileType::File);

        // Test subdirectory
        let dir1_files = provider.list_dir("/dir1");
        assert_eq!(dir1_files.len(), 2);
        assert_eq!(dir1_files[0].name, "file2.txt");
        assert_eq!(dir1_files[0].file_type, FileType::File);
        assert_eq!(dir1_files[1].name, "subdir");
        assert_eq!(dir1_files[1].file_type, FileType::Dir);

        // Test non-existent directory
        let empty_dir = provider.list_dir("/nonexistent");
        assert!(empty_dir.is_empty());
    }

    #[test]
    fn test_open() {
        let provider = setup_test_provider();

        // Test existing files
        assert_eq!(provider.open("/file1.txt"), Some("content1".to_string()));
        assert_eq!(
            provider.open("/dir1/file2.txt"),
            Some("content2".to_string())
        );

        // Test non-existent file
        assert_eq!(provider.open("/nonexistent.txt"), None);
    }
}
