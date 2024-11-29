pub enum FileType {
    Dir,
    File,
}

pub struct ProviderFile<'a> {
    pub name: &'a str,
    pub file_type: FileType,
    pub fp: String,
}

pub trait BaseProvider {
    fn base_path(&self) -> &str;
    fn list_dir(&self, path_relative: &str) -> Vec<ProviderFile>;
    fn open(&self, path: &str) -> Option<String>;
}

pub const IGNORED_DIVE_PATHS: [&str; 41] = [
  "node_modules",
  "dist",
  "build",
  "bin",
  "static",
  "public",
  "vendor",
  "terraform.tfstate.d",
  "migrations",
  "tests",
  "e2e",
  "__fixtures__",
  "__snapshots__",
  "tmp",

  // -- Dot folder
  ".artifacts",
  ".assets",
  ".azure",
  ".azure-pipelines",
  ".bundle",
  ".cache",
  ".changelog",
  ".devcontainer",
  ".docker",
  ".dynamodb",
  ".fusebox",
  ".git",
  // needed to detect github actions
  // ".github",
  ".gitlab",
  ".gradle",
  ".log",
  ".metadata",
  ".npm",
  ".nuxt",
  ".react-email",
  ".release",
  ".semgrep",
  ".serverless",
  ".svn",
  ".terraform",
  ".vercel",
  ".vscode",
  ".vuepress",
];