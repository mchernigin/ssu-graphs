use crate::*;

use std::collections::BTreeSet;

/// Find strongly connected component in oriented graph
pub fn solve21(gr: &Graph) -> GraphResult<HashSet<BTreeSet<String>>> {
    if !gr.is_oriented() {
        return Err(GraphError {
            msg: "Graph has to be oriented".to_string(),
        });
    }

    fn find_component(
        al: &HashMap<String, HashMap<String, Option<EdgeWeight>>>,
        lead: &String,
        component: &mut BTreeSet<String>,
        visited: &mut HashSet<String>,
        cur_node: &String,
    ) -> BTreeSet<String> {
        visited.insert(cur_node.to_string());
        if component.is_empty() {
            component.insert(cur_node.to_string());
        }

        let mut connections = al[cur_node].keys().collect::<Vec<_>>();
        connections.sort_unstable();

        for node in connections {
            if !visited.contains(node) && node != cur_node {
                find_component(al, lead, component, visited, node);
            }
            if node == lead && cur_node != lead {
                component.extend(visited.iter().cloned());
            }
        }

        component.to_owned()
    }

    let mut components = HashSet::<BTreeSet<String>>::new();
    let mut used_nodes = HashSet::<String>::new();

    for lead in gr.get_nodes() {
        if !used_nodes.contains(&lead) {
            let component = find_component(
                &gr.get_adjacency_list(),
                &lead,
                &mut BTreeSet::<String>::new(),
                &mut HashSet::<String>::new(),
                &lead,
            );
            components.insert(component.clone());
            used_nodes.extend(component);
        }
    }

    Ok(components)
}

/// Find shortest in terms of number of edges paths to each node from given one
pub fn solve22(gr: &Graph, start: String) -> HashMap<String, Vec<String>> {
    fn find_shortest_paths(
        al: &HashMap<String, HashMap<String, Option<EdgeWeight>>>,
        cur_node: &String,
        mut cur_path: Vec<String>,
        paths: &mut HashMap<String, Vec<String>>,
    ) -> HashMap<String, Vec<String>> {
        cur_path.push(cur_node.to_string());

        if paths[cur_node].is_empty() || cur_path.len() < paths[cur_node].len() {
            paths.insert(cur_node.to_string(), cur_path.to_owned());
        }

        let mut connections = al[cur_node].keys().collect::<Vec<_>>();
        connections.sort_unstable();

        for node in connections {
            if node != cur_node && !cur_path.contains(node) {
                find_shortest_paths(al, node, cur_path.clone(), paths);
            }
        }

        paths.to_owned()
    }

    let mut paths = HashMap::<String, Vec<String>>::new();
    gr.get_nodes().into_iter().for_each(|node| {
        paths.insert(node, Vec::new());
    });
    find_shortest_paths(
        &gr.get_adjacency_list(),
        &start,
        Vec::<String>::new(),
        &mut paths,
    )
}
