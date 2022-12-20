use crate::tasks::task2::solve22;
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
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for Weighted {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(
    gr: &Graph,
    start: String,
) -> GraphResult<(
    HashMap<String, Option<u32>>,
    HashMap<String, Option<String>>,
)> {
    if !gr.is_weighted() {
        return Err(GraphError {
            msg: "Graph has to be weighted".to_string(),
        });
    }

    let al = gr.get_adjacency_list();
    let nodes = gr.get_nodes();
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();

    dist.insert(start.clone(), Some(0));

    let mut front = BinaryHeap::new();
    front.push(Weighted {
        node: start.clone(),
        cost: 0,
    });

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
                front.push(Weighted {
                    node: neighbor.clone(),
                    cost: alt,
                });
            }
        }
    }

    Ok((dist, prev))
}

pub fn dijkstra_convenient(
    gr: &Graph,
    start: String,
) -> GraphResult<HashMap<String, (Option<u32>, Vec<String>)>> {
    let mut dijkstra_result = HashMap::new();
    let nodes = gr.get_nodes();
    let (dist, prev) = algorithms::weighted::dijkstra(gr, start)?;
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

pub fn floyd(gr: &Graph) -> GraphResult<HashMap<String, HashMap<String, Option<EdgeWeight>>>> {
    if !gr.is_weighted() {
        return Err(GraphError {
            msg: "Graph has to be weighted".to_string(),
        });
    }

    let nodes = gr.get_nodes();
    let mut am = HashMap::new();
    for from in &nodes {
        let mut paths_from = HashMap::new();
        for to in &nodes {
            paths_from.insert(to.to_string(), None);
        }
        am.insert(from.to_string(), paths_from);
    }
    let edges = gr.get_edges();
    for (from, to, weight) in edges {
        am.get_mut(&from).unwrap().insert(to, weight);
    }

    for k in &nodes {
        for i in &nodes {
            for j in &nodes {
                if am[i][k].is_none() || am[k][j].is_none() {
                    continue;
                }

                let maybe_new_weight = am[i][k].unwrap() + am[k][j].unwrap();
                if am[i][j].is_none() || am[i][j].is_some() && maybe_new_weight < am[i][j].unwrap()
                {
                    am.get_mut(i)
                        .unwrap()
                        .insert(j.to_string(), Some(maybe_new_weight));
                }
            }
        }
    }

    Ok(am)
}

// Bellman Ford indeed
pub fn find_negative_cycle(gr: &Graph, u: String) -> GraphResult<Vec<String>> {
    if !gr.is_weighted() {
        return Err(GraphError {
            msg: "Graph has to be weighted".to_string(),
        });
    }

    let mut path_weights = HashMap::new();
    let mut parent = HashMap::new();
    parent.insert(u.to_string(), u.to_string());
    let nodes = gr.get_nodes();
    for node in &nodes {
        path_weights.insert(node.to_string(), None);
    }
    path_weights.insert(u, Some(0));

    let edges = gr.get_edges();
    let n = nodes.len();
    for _ in 1..n - 1 {
        for (from, to, weight) in &edges {
            if path_weights[to].is_none() || weight < &path_weights[to] {
                path_weights.insert(to.to_string(), *weight);
                parent.insert(to.to_string(), from.to_string());
            }
        }
    }
    for (from, to, weight) in &edges {
        if path_weights[from].is_none()
            || weight.is_some()
                && path_weights[to].unwrap() > path_weights[from].unwrap() + weight.unwrap()
        {
            let mut negative_cycle = Vec::new();
            negative_cycle.push(from.to_string());
            if !parent.contains_key(from) {
                continue;
            }
            let mut p = &parent[from];
            while p != from {
                negative_cycle.push(p.to_string());
                p = &parent[p];
            }
            return Ok(negative_cycle);
        }
    }

    Ok(Vec::new())
}

pub fn edmonds_karp(gr: &Graph, source: String, sink: String) -> GraphResult<i32> {
    if !gr.is_weighted() {
        return Err(GraphError {
            msg: "Graph has to be weighted".to_string(),
        });
    }

    let mut alg_gr = gr.clone();
    let mut max_flow = 0;
    while let Some(path) = solve22(&alg_gr, source.clone()).get(&sink) {
        if path.is_empty() {
            break;
        }
        let al = alg_gr.get_adjacency_list();
        let mut path_flow = i32::MAX;
        let mut weakest = ("".to_string(), "".to_string());
        for i in 1..path.len() {
            if al[&path[i - 1]][&path[i]].unwrap() < path_flow {
                path_flow = al[&path[i - 1]][&path[i]].unwrap();
                weakest = (path[i - 1].clone(), path[i].clone());
            }
        }
        alg_gr.pop_edge(weakest.0, weakest.1)?;
        max_flow += path_flow;
    }

    Ok(max_flow)
}
