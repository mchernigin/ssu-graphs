use crate::*;

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

    let mut visited = HashSet::<String>::new();
    let mut path = Vec::<String>::new();
    dfs_inner(&gr.get_adjacency_list(), &mut visited, &mut path, &start)
}

/// Get vector of nodes in BFS (Breadth-First Search) order.
pub fn bfs(gr: &Graph, start: String) -> Vec<String> {
    fn bfs_inner(
        al: &HashMap<String, HashMap<String, Option<EdgeWeight>>>,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        cur_node: &String,
    ) -> Vec<String> {
        visited.insert(cur_node.to_string());

        let mut connections = al[cur_node].keys().collect::<Vec<_>>();
        connections.sort_unstable();

        for node in connections {
            if !visited.contains(node) {
                bfs_inner(al, visited, path, node);
            }
        }

        path.push(cur_node.to_string());
        path.to_owned()
    }

    let mut visited = HashSet::<String>::new();
    let mut path = Vec::<String>::new();
    bfs_inner(&gr.get_adjacency_list(), &mut visited, &mut path, &start)
}

pub fn bfs_to(
    gr: &Graph,
    start: String,
    end: String,
    parent: &mut HashMap<String, String>,
) -> Vec<String> {
    fn bfs_inner(
        al: &HashMap<String, HashMap<String, Option<EdgeWeight>>>,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        cur_node: &String,
        end: &String,
        parent: &mut HashMap<String, String>,
    ) -> Vec<String> {
        visited.insert(cur_node.to_string());

        if cur_node == end {
            path.push(cur_node.to_string());
            return path.to_owned();
        }

        let mut connections = al[cur_node].keys().collect::<Vec<_>>();
        connections.sort_unstable();

        for node in connections {
            if !visited.contains(node) {
                parent.insert(node.to_string(), cur_node.to_string());
                let r = bfs_inner(al, visited, path, node, end, parent);
                if !r.is_empty() {
                    return  r;
                }
            }
        }

        path.push(cur_node.to_string());
        Vec::new()
    }

    let mut visited = HashSet::<String>::new();
    let mut path = Vec::<String>::new();
    bfs_inner(
        &gr.get_adjacency_list(),
        &mut visited,
        &mut path,
        &start,
        &end,
        parent,
    )
}
