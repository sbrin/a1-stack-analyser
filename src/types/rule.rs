use regex::Regex;
use std::collections::HashSet;

use crate::{
    payload::payload::Payload,
    provider::base::{BaseProvider, ProviderFile},
};

use super::techs::{AllowedKeys, TechType};

#[derive(Clone)]
pub enum SupportedDeps {
    Deno,
    Docker,
    GithubAction,
    Golang,
    Npm,
    Php,
    Python,
    Ruby,
    Rust,
    TerraformResource,
    Terraform,
}

pub struct RuleDependency {
    pub r#type: SupportedDeps,
    pub name: Option<String>,
    pub example: Option<String>,
}

pub struct Rule {
    pub tech: AllowedKeys,
    pub name: String,
    pub r#type: TechType,
    pub dependencies: Option<Vec<RuleDependency>>,
    pub detect: Option<Vec<ComponentMatcher>>,
    pub extensions: Option<Vec<String>>,
}

pub enum RuleFiles {
    FilesRegex {
        files: Regex,
        example: String,
    },
    FilesArray {
        files: Vec<String>,
    },
    NoFiles,
    MatchFullPath {
        match_full_path: bool,
        files: Regex,
        example: String,
    },
}

pub struct RuleWithFile {
    pub tech: AllowedKeys,
    pub files: RuleFiles,
}

pub type ComponentMatcher =
    fn(files: &Vec<ProviderFile>, provider: &dyn BaseProvider) -> Result<Payload, bool>;

pub type TechMatcher = fn(files: Vec<ProviderFile>) -> Result<(Rule, String), bool>;
pub type ExtensionMatcher = fn(list: HashSet<String>) -> Result<(Rule, String), bool>;
