#[derive(Debug, Clone, PartialEq, Eq)]
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

pub trait BaseProvider: std::fmt::Debug {
    fn list_dir(&self, path: &str) -> Vec<ProviderFile>;
    fn base_path(&self) -> String;
    fn open(&self, path: &str) -> Option<String>;
}

pub const IGNORED_DIVE_PATHS: &[&str] = &[
    "node_modules",
    "dist",
    "build",
    // ... other paths can be added as needed
];
