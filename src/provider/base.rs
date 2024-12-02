use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum FileType {
    Dir,
    File,
}

#[derive(Debug, Clone)]
pub struct ProviderFile {
    pub name: String,
    pub file_type: FileType,
    pub fp: String,
}

#[async_trait]
pub trait BaseProvider {
    fn base_path(&self) -> String;
    async fn list_dir(&self, path_relative: &str) -> Vec<ProviderFile>;
    async fn open(&self, path: &str) -> Option<String>;
}

pub const IGNORED_DIVE_PATHS: &[&str] = &[
    "node_modules",
    "dist",
    "build",
    // ... other paths can be added as needed
];
