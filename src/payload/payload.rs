use crate::payload::match_all_files::match_all_files;
use crate::provider::base::{BaseProvider, FileType, IGNORED_DIVE_PATHS};
use crate::types::rule::ComponentMatcher;
use crate::types::{Dependency, GraphEdge};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Payload {
    pub id: &'static str,
    pub name: &'static str,
    pub path: HashSet<String>,
    pub tech: Option<String>,
    pub languages: HashMap<String, i32>,
    pub childs: Vec<Payload>,
    pub techs: HashSet<String>,
    pub in_component: Option<Box<Payload>>,
    pub dependencies: Vec<Dependency>,
    pub edges: Vec<GraphEdge>,
    pub parent: Option<Box<Payload>>,
    pub reason: HashSet<String>,
}

impl Payload {
    pub fn new(id: &'static str, name: &'static str) -> Self {
        Payload {
            id,
            name,
            path: HashSet::new(),
            tech: None,
            languages: HashMap::new(),
            childs: Vec::new(),
            techs: HashSet::new(),
            in_component: None,
            dependencies: Vec::new(),
            edges: Vec::new(),
            parent: None,
            reason: HashSet::new(),
        }
    }

    pub async fn recurse(&mut self, provider: &dyn BaseProvider, file_path: &'static str) {
        let files = provider.list_dir(file_path.clone());
        // let rules_components = [];
        pub const RULES_COMPONENTS: Vec<ComponentMatcher> = Vec::new();

        let mut ctx: &mut Payload = self;
        for rule in RULES_COMPONENTS.iter() {
            let res = rule(&files, provider);
            if res.is_err() {
                continue;
            }

            impl IntoIterator for Payload {
                type Item = Payload;
                type IntoIter = std::vec::IntoIter<Self::Item>;

                fn into_iter(self) -> Self::IntoIter {
                    self.childs.into_iter()
                }
            }

            // let res_array = res.unwrap();
            // for pl in res_array {
            //     if pl.name != "virtual" {
            //         ctx = &mut pl;
            //         self.add_child(pl);
            //     } else {
            //         for child in pl.childs {
            //             self.add_child(child);
            //         }
            //         self.combine_dependencies(pl);
            //     }
            // }

            // for pl in res_array {
            //     if pl.name != "virtual" {
            //         *ctx = pl;
            //         self.add_child(pl.clone()); // Assuming Payload implements Clone
            //     } else {
            //         // Process virtual payload children
            //         for child in pl.childs.iter() {
            //             self.add_child(child.clone());
            //         }
            //         self.combine_dependencies(pl);
            //     }
            // }
        }

        // Detect Tech
        // let matched = match_all_files(files, &provider.base_path());
        // ctx.add_techs(matched);

        // // Recursively dive in folders
        // for file in files {
        //     if matches!(file.file_type, FileType::File) {
        //         ctx.detect_lang(&file.name);
        //         continue;
        //     }
        //     if IGNORED_DIVE_PATHS.contains(&file.name) {
        //         continue;
        //     }

        //     let fp = PathBuf::from(file_path).join(&file.name);
        //     ctx.recurse(provider, fp.to_str().unwrap_or(file_path));
        // }
    }

    // pub fn add_child(&mut self, service: Payload) -> Payload {
    //     if let Some(exist) = self.childs.iter_mut().find(|s| {
    //         s.name == service.name
    //             || (s.tech.is_some() && service.tech.is_some() && s.tech == service.tech)
    //     }) {
    //         for p in service.path.iter() {
    //             exist.path.insert(p.clone());
    //         }

    //         if let Some(parent) = &service.parent {
    //             for edge in parent.edges.iter_mut() {
    //                 if edge.target.id.as_ref() != service.id.as_ref() {
    //                     continue;
    //                 }
    //                 edge.target = Box::new(exist.to_owned());
    //             }
    //         }

    //         exist.dependencies.extend(service.dependencies);
    //         return exist.to_owned();
    //     }

    //     service.set_parent(Some(Box::new(self.to_owned())));
    //     self.childs.push(service);
    //     self.childs.last().unwrap().to_owned()
    // }

    // pub fn add_techs(&mut self, tech: HashMap<String, Vec<String>>) {
    //     for (key, reason) in tech {
    //         self.add_tech(key, reason);
    //     }
    // }

    // pub fn add_tech(&mut self, tech: String, reason: Vec<String>) {
    //     self.techs.insert(tech.clone());
    //     for r in reason {
    //         self.reason.insert(r);
    //     }

    //     find_implicit_component(self, &tech, &reason);
    //     find_hosting(self, &tech);
    // }

    // pub fn add_edges(&mut self, pl: Payload) {
    //     self.edges.push(Edge {
    //         target: Box::new(pl),
    //         read: true,
    //         write: true,
    //     });
    // }

    // pub fn add_lang(&mut self, name: String, count: usize) {
    //     *self.languages.entry(name.clone()).or_insert(0) += count;

    //     if let Some(key) = name_to_key.get(&name) {
    //         if !self.techs.contains(key) {
    //             self.add_tech(key.clone(), vec![]);
    //         }
    //     }
    // }

    // pub fn set_parent(&mut self, pl: Option<Box<Payload>>) {
    //     self.parent = pl;
    // }

    // pub fn detect_lang(&mut self, filename: &str) {
    //     if let Some(lang) = detect_lang(filename) {
    //         self.add_lang(lang.group.unwrap_or(lang.name), 1);
    //     }
    // }

    // pub fn combine_dependencies(&mut self, pl: Payload) {
    //     let mut dedup = HashMap::new();
    //     for dep in &self.dependencies {
    //         dedup.insert(dep.join("_"), dep.clone());
    //     }
    //     for dep in pl.dependencies {
    //         dedup.insert(dep.join("_"), dep);
    //     }
    //     self.dependencies = dedup.values().cloned().collect();
    // }

    // pub fn combine(&mut self, pl: Payload) {
    //     self.path.extend(pl.path);
    //     self.combine_dependencies(pl);

    //     for (lang, count) in pl.languages {
    //         self.add_lang(lang, count);
    //     }

    //     for tech in pl.techs {
    //         self.techs.insert(tech);
    //     }
    //     if let Some(tech) = pl.tech {
    //         self.techs.insert(tech);
    //     }
    // }

    // pub fn copy(&self) -> Payload {
    //     let mut cp = Payload::new(
    //         self.id.clone(),
    //         self.name.clone(),
    //         self.path.clone(),
    //         self.parent.clone(),
    //         self.tech.clone(),
    //         Some(self.dependencies.clone()),
    //         None,
    //     );
    //     cp.techs = self.techs.clone();
    //     cp.in_component = self.in_component.clone();
    //     cp.edges = self.edges.clone();
    //     cp.languages = self.languages.clone();
    //     cp.childs = self.childs.clone();

    //     cp
    // }

    // pub fn to_json(&self, root: &str) -> AnalyserJson {
    //     AnalyserJson {
    //         id: self.id.clone(),
    //         name: self.name.clone(),
    //         path: clean_path(self.path.iter().cloned().collect(), root),
    //         tech: self.tech.clone(),
    //         edges: self
    //             .edges
    //             .iter()
    //             .map(|edge| EdgeJson {
    //                 target: edge.target.id.clone(),
    //                 read: edge.read,
    //                 write: edge.write,
    //             })
    //             .collect(),
    //         in_component: self.in_component.as_ref().map(|ic| ic.clone()),
    //         childs: self
    //             .childs
    //             .iter()
    //             .map(|service| service.to_json(root))
    //             .collect(),
    //         techs: self.techs.iter().cloned().collect(),
    //         languages: self.languages.clone(),
    //         dependencies: self.dependencies.clone(),
    //         reason: self.reason.iter().cloned().collect(),
    //     }
    // }
}
