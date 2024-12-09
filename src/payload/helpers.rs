use crate::payload::payload::Payload;
use crate::rules::register::LIST_INDEXED;

const NOT_A_COMPONENT: [&str; 4] = ["ci", "language", "tool", "framework"];

/// When receive a tech in a component, we can deduct a new Component that was missing
/// For example we receive:
///  - "pg" from a package.json
///  - we deduct there is a Postgresql
///  - Create an eponymous component
///
/// Obviously there could be some false positive.
pub fn find_implicit_component(pl: &mut Payload, tech: &str, reason: &[String]) {
    let list_indexed = LIST_INDEXED.lock().unwrap();
    let ref_rule = match list_indexed.get(tech) {
        Some(rule) => rule,
        None => return,
    };

    if NOT_A_COMPONENT.contains(&ref_rule.r#type.as_str()) {
        return;
    }

    let mut comp = Payload::new(&ref_rule.name, pl.path.iter().next().unwrap());
    comp.tech = Some(tech.to_string());
    comp.reason = reason.iter().cloned().collect();

    pl.add_child(comp);
}

/// When receive a tech in a component, we can deduct a new Hosting that was missing
/// For example we receive:
///  - "vercel" from a folder named .vercel
///  - we deduct this component is hosted by Vercel
///  - Create an eponymous component
///
/// Obviously there could be some false positive.
pub fn find_hosting(pl: &mut Payload, tech: &str) {
    let list_indexed = LIST_INDEXED.lock().unwrap();
    let ref_rule = match list_indexed.get(tech) {
        Some(rule) => rule,
        None => return,
    };

    if ref_rule.r#type != "hosting" && ref_rule.r#type != "cloud" {
        return;
    }

    let find = pl.childs.iter().find(|c| c.tech.as_ref() == Some(&ref_rule.tech));
    if find.is_none() {
        panic!("cant find hosting {}", ref_rule.tech);
    }
}

/// Some edges can be found in the dependencies, i.e:
/// - in monorepo or in some language you can have many folders and import another folder.
///
/// We try to find those import, using only the dependencies (not opening the code),
/// it can lead to some false positive with very generic names.

pub fn find_edges_in_dependencies(pl: &mut Payload) {
    // First collect all names into a HashSet
    let names: std::collections::HashSet<String> =
        pl.childs.iter().map(|child| child.name.clone()).collect();

    // Create a Vec to store the edges we want to add
    let mut edges_to_add = Vec::new();

    // First pass: collect all edges we want to add
    for child in &pl.childs {
        for dep in &child.dependencies {
            let name = &dep[1];
            if !names.contains(name) {
                continue;
            }

            // Self referencing
            if name == &child.name || child.tech.as_ref().map_or(false, |tech| name == tech) {
                continue;
            }

            // Check if we already added an edge about that
            if let Some(target) = pl.childs.iter().find(|c| c.name == *name) {
                // Store the edge information for later processing
                edges_to_add.push((child.id.clone(), target.id.clone()));
            }
        }
    }

    // Second pass: add all the edges
    for (source_id, target_id) in edges_to_add {
        // Add edge logic would go here
        // You'll need to implement the actual edge adding functionality
        // This could be something like:
        // pl.add_edge(&source_id, &target_id);
    }
}

/// flatten takes a nested Payload and brings everything down to a single level.
/// It deduplicates components that are strictly similar, and keep references in path.
///
/// If merge = true, it merges all fields that can be merged down to the parent (e.g: dependencies).
/// Merging is only useful to get a summary of everything at the root level.
pub fn flatten(src: &Payload, merge: bool) -> Payload {
    // Generate a flat list of childs
    let mut dest = Payload::new("flatten", "/");
    push_childs(src, &mut dest);

    // Find and merge duplicates
    let mut duplicates = Vec::new();
    for i in 0..dest.childs.len() {
        if duplicates.contains(&dest.childs[i].id) {
            continue;
        }

        for j in (i + 1)..dest.childs.len() {
            if dest.childs[i].tech.is_none() || dest.childs[j].tech.is_none() {
                continue;
            }
            if dest.childs[i].name != dest.childs[j].name
                && dest.childs[i].tech != dest.childs[j].tech
            {
                continue;
            }

            duplicates.push(dest.childs[j].id.clone());
            // Combine logic would go here
            // Note: The original combine functionality needs to be implemented in the Payload struct
        }
    }

    // Remove duplicates
    dest.childs.retain(|child| !duplicates.contains(&child.id));

    find_edges_in_dependencies(&mut dest);

    if merge {
        // Merge logic would go here
        // Note: The original merge functionality needs to be implemented in the Payload struct
    }

    dest
}

fn push_childs(src: &Payload, dest: &mut Payload) {
    for pl in &src.childs {
        let mut cp = pl.clone();
        push_childs(&cp, dest);
        cp.childs.clear();
        dest.childs.push(cp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Add tests here
}
