use graphs_at_ssu::*;
use inquire::{error::InquireResult, Confirm, CustomType, CustomUserError, Select, Text};
use std::process;

fn main() -> InquireResult<()> {
    let tasks = vec![
        "Find MST using Kruskal algorithm",
        "Find MST using Prim algorithm",
    ];

    print!("\x1B[2J\x1B[1;1H"); // clear the console
    let graph_creation_ans = or_err!(Select::new(
        "What do you want to start with?",
        vec!["Create new graph", "Load from file"],
    )
    .prompt());

    let mut gr = if graph_creation_ans == "Load from file" {
        let file_path = or_err!(Text::new("Select graph file:")
            .with_suggester(&suggest_file_paths)
            .with_completer(&complete_file_path)
            .with_initial_value("./")
            .prompt());
        or_err!(Graph::from_file(file_path))
    } else {
        let is_weighted_ans = or_err!(Confirm::new("Do you want weighted graph?")
            .with_default(false)
            .prompt());
        let is_oriented_ans = or_err!(Confirm::new("Do you want oriented graph?")
            .with_default(false)
            .prompt());
        Graph::new(is_weighted_ans, is_oriented_ans)
    };
    println!();

    loop {
        // TODO: Change properties from interface
        let cmd_ans = match Select::new(
            "What to do...?",
            vec![
                "Look at graph",
                "Push new node",
                "Pop existing node",
                "Connect two nodes",
                "Disconnect two nodes",
                "Save to file",
                "Tasks...",
            ],
        )
        .prompt()
        {
            Ok(cmd) => cmd,
            Err(_) => break,
        };

        match cmd_ans {
            "Look at graph" => println!("\n{}", gr.pretty_view()),
            "Push new node" => {
                let node_name = or_escape!(Text::new("Enter node name:").prompt());
                if let Err(e) = gr.push_node(node_name) {
                    safe_err!("Cannot push node: {e}");
                }
            }
            "Pop existing node" => {
                let node_name = or_escape!(Select::new("Select node:", gr.get_nodes()).prompt());
                if let Err(e) = gr.pop_node(node_name) {
                    safe_err!("Cannot pop node: {e}");
                }
            }
            "Connect two nodes" => {
                let nodes = gr.get_nodes();
                let node1 = or_escape!(Select::new("Select first node:", nodes.clone()).prompt());
                let node2 = or_escape!(Select::new("Select second node:", nodes).prompt());
                let weight = if gr.is_weighted() {
                    Some(or_escape!(CustomType::<EdgeWeight>::new(
                        "Enter connection weight:"
                    )
                    .with_error_message("Enter an integer value")
                    .prompt()))
                } else {
                    None
                };
                if let Err(e) = gr.push_edge(node1.clone(), node2.clone(), weight) {
                    safe_err!("Cannot connect node {node1:?} with {node2:?}: {e}");
                }
            }
            "Disconnect two nodes" => {
                let nodes = gr.get_nodes();
                let node1 = or_escape!(Select::new("Select first node:", nodes.clone()).prompt());
                let node2 = or_escape!(Select::new("Select second node:", nodes).prompt());
                if let Err(e) = gr.pop_edge(node1.clone(), node2.clone()) {
                    safe_err!("Cannot disconnect node {node1:?} with {node2:?}: {e}");
                }
            }
            "Save to file" => {
                let path = or_escape!(Text::new("Select graph file:")
                    .with_suggester(&suggest_file_paths)
                    .with_completer(&complete_file_path)
                    .with_initial_value("./")
                    .prompt());
                if let Err(e) = gr.save_to_file(&path) {
                    safe_err!("Cannot save to {path:?}: {e}");
                }
            }
            "Tasks..." => {
                let mst_with = |f: &dyn Fn(&Graph) -> GraphResult<Vec<EdgeWeighted>>| match f(&gr) {
                    Ok(mst) => {
                        print!(
                            "\n{}\n",
                            mst.iter()
                                .map(|t| format!("{t:?}"))
                                .collect::<Vec<String>>()
                                .join(", ")
                        );
                    }
                    Err(e) => safe_err!("Cannot find MST: {e}"),
                };
                match or_escape!(Select::new("Select task:", tasks.clone()).prompt()) {
                    "Find MST using Kruskal algorithm" => mst_with(&algorithms::mst::kruskal),
                    "Find MST using Prim algorithm" => mst_with(&algorithms::mst::prim),
                    _ => safe_err!("Unknown algorithm"),
                }
            }
            _ => safe_err!("Not implemented"),
        }
        println!();
    }

    Ok(())
}

fn suggest_file_paths(input: &str) -> Result<Vec<String>, CustomUserError> {
    Ok(list_paths(input)?)
}

fn complete_file_path(input: &str) -> Result<Option<String>, CustomUserError> {
    // Implementation from https://rosettacode.org/wiki/Longest_common_prefix#Rust
    fn longest_common_prefix<T: AsRef<[u8]>>(list: &[T]) -> Option<Vec<u8>> {
        if list.is_empty() {
            return None;
        }
        let mut ret = Vec::new();
        let mut i = 0;
        loop {
            let mut c = None;
            for word in list {
                let word = word.as_ref();
                if i == word.len() {
                    return Some(ret);
                }
                match c {
                    None => {
                        c = Some(word[i]);
                    }
                    Some(letter) if letter != word[i] => return Some(ret),
                    _ => continue,
                }
            }
            if let Some(letter) = c {
                ret.push(letter);
            }
            i += 1;
        }
    }

    Ok(longest_common_prefix(&list_paths(input)?)
        .map(|bytes| String::from_utf8_lossy(&bytes).to_string()))
}

fn list_paths(root: &str) -> std::io::Result<Vec<String>> {
    let mut suggestions = vec![];

    let mut input_path = std::path::PathBuf::from(root);
    if let Some(parent) = input_path.parent() {
        if !input_path.exists() || !input_path.is_dir() || !root.ends_with('/') {
            input_path = parent.to_path_buf();
        }
    }
    if root.is_empty() {
        input_path = std::env::current_dir()?;
    }
    if !input_path.exists() {
        return Ok(vec![]);
    }

    for entry in std::fs::read_dir(input_path)? {
        let path = entry?.path();
        let path_str = path.to_string_lossy();

        if path_str.starts_with(root) && !path_str.contains("/.") {
            let path = if path.is_dir() {
                format!("{path_str}/")
            } else {
                path_str.to_string()
            };
            suggestions.push(path);
        }
    }

    Ok(suggestions)
}
