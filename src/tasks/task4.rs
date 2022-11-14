use crate::*;

pub fn solve4a(
    gr: &Graph,
) -> GraphResult<HashMap<String, HashMap<String, (Option<u32>, Vec<String>)>>> {
    let mut res = HashMap::new();
    let nodes = gr.get_nodes();
    for start in nodes {
        res.insert(
            start.clone(),
            algorithms::weighted::dijkstra_convenient(gr, start)?,
        );
    }

    Ok(res)
}
