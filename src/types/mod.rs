pub mod rule;
pub mod techs;

use crate::payload::payload::Payload;

#[derive(Clone)]
pub struct GraphEdge {
    pub target: Payload,
    read: bool,
    write: bool,
}

pub type Dependency = (String, String, String);

pub struct Analyser {
    /// Unique random id for this payload
    pub id: String,

    /// Best-effort name of this payload
    pub name: String,

    /// Where this payload was found.
    /// When flatten() it will contain all path that were deduplicated
    pub path: std::collections::HashSet<String>,

    /// If this payload is a specific Technology.
    /// e.g: if it's a Postgresql database, the tech will be: "postgresql"
    pub tech: Option<String>,

    /// List matched tech from the rules.
    /// Computed with the dependencies and languages.
    pub techs: std::collections::HashSet<String>,

    /// If this payload is hosted by another payload.
    /// e.g: we found a vercel dependency at the same level, this payload will be considered in the component Vercel.
    pub in_component: Option<Payload>,

    /// List all childs of this payload
    pub childs: Vec<Payload>,

    /// List all languages found in this folder.
    /// This list is computed using file extensions.
    pub languages: std::collections::HashMap<String, usize>,

    /// List all relationship from this Payload to another one.
    /// e.g:
    /// this is a package.json and we found a Postgresql at the same level,
    /// we will add an edge from this payload to the Postgresql's one.
    pub edges: Vec<GraphEdge>,

    /// List all dependencies wether or not they matched a rule.
    pub dependencies: Vec<Dependency>,
}
