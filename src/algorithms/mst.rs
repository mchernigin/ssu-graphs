use crate::*;

use std::collections::HashSet;

pub fn kruskal(gr: &Graph) -> GraphResult<Vec<EdgeWeighted>> {
    check_if_applicable(gr)?;

    let mut edges = gr
        .get_edges()
        .iter()
        .map(|(n1, n2, w)| (n1.to_string(), n2.to_string(), w.unwrap()))
        .collect::<Vec<_>>();
    edges.sort_unstable_by_key(|x| x.2);

    let mut tree_id = HashMap::<String, usize>::new();
    for (i, node) in gr.get_nodes().iter().enumerate() {
        tree_id.insert(node.to_owned(), i);
    }

    let mut mst = Vec::<EdgeWeighted>::new();
    for edge in edges {
        if tree_id[&edge.0] != tree_id[&edge.1] {
            let old_id = tree_id[&edge.0];
            let new_id = tree_id[&edge.1];
            for (node, parent) in tree_id.clone().iter() {
                if *parent == old_id {
                    tree_id.insert(node.to_string(), new_id);
                }
            }

            mst.push(edge);
        }
    }

    Ok(mst)
}

pub fn prim(gr: &Graph) -> GraphResult<Vec<EdgeWeighted>> {
    check_if_applicable(gr)?;

    let mut mst = Vec::<EdgeWeighted>::new();
    let mut used_nodes = HashSet::<String>::new();
    let mut available_edges = HashSet::<EdgeWeighted>::new();

    let mut not_used_nodes = HashSet::<String>::from_iter(gr.get_nodes());
    if not_used_nodes.is_empty() {
        return Err(GraphError {
            msg: "Graph is empty".to_string(),
        });
    }

    let start_element: String = not_used_nodes.iter().next().unwrap().clone();
    for (to, weight) in gr.get_adjacency_list()[&start_element].iter() {
        available_edges.insert((start_element.to_string(), to.to_string(), weight.unwrap()));
    }
    not_used_nodes.remove(&start_element);
    used_nodes.insert(start_element);

    while !not_used_nodes.is_empty() {
        let mut next_connection: Option<EdgeWeighted> = None;
        for edge in available_edges.iter() {
            if !used_nodes.contains(&edge.1)
                && (next_connection.is_none()
                    || next_connection.is_some() && next_connection.clone().unwrap().2 > edge.2)
            {
                next_connection = Some(edge.to_owned());
            }
        }
        let next_connection = next_connection.ok_or_else(|| GraphError {
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

fn check_if_applicable(gr: &Graph) -> GraphResult<()> {
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
