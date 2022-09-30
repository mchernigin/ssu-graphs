use crate::*;

use std::collections::HashSet;

pub fn kruskal(gr: &Graph) -> Result<Vec<(String, String, EdgeWeight)>, GraphError> {
    check_if_applicable(gr)?;

    let mut edges: Vec<(String, String, EdgeWeight)> = gr
        .get_edges()
        .iter()
        .map(|(n1, n2, w)| (n1.to_owned(), n2.to_owned(), w.unwrap()))
        .collect();
    edges.sort_by_key(|x| x.2);

    let mut mst = Vec::<(String, String, EdgeWeight)>::new();
    let mut tree_id = HashMap::<String, usize>::new();
    for (i, node) in gr.get_nodes().iter().enumerate() {
        tree_id.insert(node.to_owned(), i);
    }

    for edge in edges {
        if tree_id[&edge.0] != tree_id[&edge.1] {
            let old_id = tree_id[&edge.0];
            let new_id = tree_id[&edge.1];
            for (node, parent) in tree_id.to_owned() {
                if parent == old_id {
                    tree_id.insert(node, new_id);
                }
            }

            mst.push(edge);
        }
    }

    Ok(mst)
}

pub fn prim(gr: &Graph) -> Result<Vec<(String, String, EdgeWeight)>, GraphError> {
    check_if_applicable(gr)?;

    let mut mst = Vec::<(String, String, EdgeWeight)>::new();
    let mut used_nodes = HashSet::<String>::new();
    let mut available_edges = HashSet::<(String, String, EdgeWeight)>::new();

    let mut not_used_nodes = HashSet::<String>::from_iter(gr.get_nodes());
    if let Some(start_element) = not_used_nodes.clone().iter().next() {
        not_used_nodes.remove(start_element);
        used_nodes.insert(start_element.to_owned());
        for connection in &gr.get_adjacency_list()[start_element] {
            available_edges.insert((
                start_element.to_owned(),
                connection.0.to_owned(),
                connection.1.unwrap(),
            ));
        }
    } else {
        return Err(GraphError {
            msg: "Graph is empty".to_string(),
        });
    };

    while !not_used_nodes.is_empty() {
        let mut next_connection: Option<(String, String, EdgeWeight)> = None;
        for edge in available_edges.iter() {
            if !used_nodes.contains(&edge.1)
                && (next_connection.is_none()
                    || next_connection.is_some() && next_connection.clone().unwrap().2 > edge.2)
            {
                next_connection = Some(edge.to_owned());
            }
        }
        let next_connection = next_connection.ok_or(GraphError {
            msg: "Found isolated node!".to_string(),
        })?;
        let new_node = next_connection.1.clone();
        if !used_nodes.contains(&new_node) {
            used_nodes.insert(new_node.clone());
            not_used_nodes.remove(&new_node);
            mst.push(next_connection.clone());
        }
        for connection in &gr.get_adjacency_list()[&new_node] {
            available_edges.insert((
                next_connection.1.to_owned(),
                connection.0.to_owned(),
                connection.1.unwrap(),
            ));
        }
        available_edges.remove(&next_connection);
    }

    Ok(mst)
}

fn check_if_applicable(gr: &Graph) -> Result<(), GraphError> {
    if !gr.is_weighted() {
        Err(GraphError {
            msg: "Graph has to be weighted".to_string(),
        })
    } else if gr.is_oriented() {
        Err(GraphError {
            msg: "Graph has to be not oriented".to_string(),
        })
    } else {
        Ok(())
    }
}
