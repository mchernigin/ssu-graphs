use crate::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Clone, Eq, PartialEq)]
struct Weighted {
    node: String,
    cost: u32,
}

impl Ord for Weighted {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for Weighted {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(gr: &Graph, start: String) -> GraphResult<(HashMap<String, Option<u32>>, HashMap<String, Option<String>>)> {
    if !gr.is_weighted() {
        return Err(GraphError { msg: "Graph has to be weighted".to_string() });
    }

    let al = gr.get_adjacency_list();
    let nodes = gr.get_nodes();
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();

    dist.insert(start.clone(), Some(0));

    let mut front = BinaryHeap::new();
    front.push(Weighted {node: start.clone(), cost: 0});

    for node in nodes {
        prev.insert(node.clone(), None);
        if node != start {
            dist.insert(node.clone(), None);
        }
    }
    *dist.get_mut(&start).unwrap() = Some(0u32);

    while !front.is_empty() {
        let u = front.pop().unwrap().node;

        let mut connections = al[&u].keys().collect::<Vec<_>>();
        connections.sort_unstable();

        for neighbor in connections {
            let alt = dist[&u].unwrap() + al[&u][neighbor].unwrap() as u32;
            if dist[neighbor].is_none() || alt < dist[neighbor].unwrap() {
                *dist.get_mut(neighbor).unwrap() = Some(alt);
                *prev.get_mut(neighbor).unwrap() = Some(u.clone());
                front.push(Weighted {node: neighbor.clone(), cost: alt});
            }
        }
    }

    Ok((dist, prev))
}

pub fn dijkstra_convenient(gr: &Graph, start: String) -> GraphResult<HashMap<String, (Option<u32>, Vec<String>)>> {
    let mut dijkstra_result = HashMap::new();
    let nodes = gr.get_nodes();
    let (dist, prev) = algorithms::weighted::dijkstra(&gr, start.clone())?;
    for node in nodes {
        let mut path = VecDeque::new();
        let mut n = Some(node.clone());
        while n.is_some() {
            path.push_front(n.clone().unwrap());
            n = prev[&n.unwrap()].clone();
        }
        dijkstra_result.insert(node.clone(), (dist[&node], Vec::from(path)));
    }
    Ok(dijkstra_result)
}

