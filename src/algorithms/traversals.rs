use crate::*;

use std::collections::VecDeque;

/// Get vector of nodes in DFS (Depth-First Search) order.
pub fn dfs(gr: &Graph, start: String) -> Vec<String> {
    fn dfs_inner(
        al: &HashMap<String, HashMap<String, Option<EdgeWeight>>>,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        cur_node: &String,
    ) -> Vec<String> {
        visited.insert(cur_node.to_string());
        path.push(cur_node.to_string());

        let mut connections = al[cur_node].keys().collect::<Vec<_>>();
        connections.sort_unstable();

        for node in connections {
            if !visited.contains(node) {
                dfs_inner(al, visited, path, node);
            }
        }

        path.to_owned()
    }

    dfs_inner(
        &gr.get_adjacency_list(),
        &mut HashSet::<String>::new(),
        &mut Vec::<String>::new(),
        &start,
    )
}

/// Get vector of nodes in BFS (Breadth-First Search) order.
pub fn bfs(gr: &Graph, start: String) -> Vec<String> {
    let al = gr.get_adjacency_list();
    let mut visited = HashSet::<String>::new();
    let mut path = Vec::<String>::new();
    let mut queue = VecDeque::<String>::new();

    queue.push_back(start.clone());
    visited.insert(start);
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        path.push(node.clone());

        let mut connections = al[&node].keys().collect::<Vec<_>>();
        connections.sort_unstable();

        for node in connections {
            if !visited.contains(node) {
                visited.insert(node.clone());
                queue.push_back(node.clone());
            }
        }
    }

    path
}
