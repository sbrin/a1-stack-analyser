use regex::Regex;
use std::collections::HashSet;

use crate::{
    payload::payload::Payload,
    provider::base::{BaseProvider, ProviderFile},
};

#[derive(Clone, Debug)]
pub struct RuleDependency {
    pub r#type: String,
    pub name: Option<String>,
    pub example: Option<String>,
}

impl Default for RuleDependency {
    fn default() -> Self {
        RuleDependency {
            r#type: String::new(),
            name: None,
            example: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub tech: String,
    pub name: String,
    pub r#type: String,
    pub dependencies: Option<Vec<RuleDependency>>,
    pub detect: Option<Vec<ComponentMatcher>>,
    pub extensions: Option<Vec<String>>,
    pub files: Option<RuleFiles>,
    pub example: Option<String>,
}

impl Default for Rule {
    fn default() -> Self {
        Rule {
            tech: "".to_string(),
            name: "".to_string(),
            r#type: "".to_string(),
            dependencies: None,
            detect: None,
            extensions: None,
            files: None,
            example: None,
        }
    }
}

#[derive(Clone, Debug)]
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

pub type ComponentMatcher =
    fn(files: &Vec<ProviderFile>, provider: &dyn BaseProvider) -> Result<Payload, bool>;

pub type TechMatcher = Box<dyn Fn(Vec<ProviderFile>) -> Result<(Rule, String), bool> + Send>;
pub type ExtensionMatcher = Box<dyn Fn(HashSet<String>) -> Result<(Rule, String), bool> + Send>;
