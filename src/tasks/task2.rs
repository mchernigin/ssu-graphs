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
        
        let mut connections: Vec<String> = al[cur_node].iter().map(|(k, _)| k.to_owned()).collect();
        connections.sort();
        
        for node in &connections {
            if !visited.contains(node) {
                find_component(al, lead, component, visited, node);
            }
            if node == lead {
                component.extend(visited.iter().cloned());
            }
        }
        
        component.to_owned()
    }
    
    let al = gr.get_adjacency_list();
    let mut components = HashSet::<BTreeSet<String>>::new();
    let mut used_nodes = HashSet::<String>::new();
    
    for lead in gr.get_nodes() {
        if !used_nodes.contains(&lead) {
            let component = find_component(
                &al,
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