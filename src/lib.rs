use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub mod algorithms;
pub mod tasks;

/// Print an error message to the terminal without exiting the program.
#[macro_export]
macro_rules! safe_err {
    ($($arg:tt)*) => {{
        eprint!("\x1b[1;31mError\x1b[0m: \x1b[1m");
        eprint!($($arg)*);
        eprintln!("\x1b[0m");
    }};
}

/// Print an error message to the terminal and exiting the program.
#[macro_export]
macro_rules! or_err {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                safe_err!("{e}");
                process::exit(1);
            }
        }
    };
}

/// Check if result is Err. If it is, continue to next iteration.
#[macro_export]
macro_rules! or_escape {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                eprintln!("\x1b[1;33mGoing back\x1b[0m: \x1b[1m{e}\x1b[0m\n");
                continue;
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct GraphError {
    pub msg: String,
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<std::io::Error> for GraphError {
    fn from(e: std::io::Error) -> Self {
        GraphError {
            msg: format!("File error: {e}"),
        }
    }
}

impl From<std::num::ParseIntError> for GraphError {
    fn from(e: std::num::ParseIntError) -> Self {
        GraphError {
            msg: format!("Cannot parse connection weight: {e}"),
        }
    }
}

pub type GraphResult<T> = Result<T, GraphError>;
pub type EdgeWeight = i32;
pub type Edge = (String, String, Option<EdgeWeight>);
pub type EdgeWeighted = (String, String, EdgeWeight);

#[derive(Debug, Clone)]
pub struct Graph {
    adjacency_list: HashMap<String, HashMap<String, Option<EdgeWeight>>>,
    is_weighted: bool,
    is_oriented: bool,
}

impl Graph {
    /// Create new graph.
    pub fn new(is_weighted: bool, is_oriented: bool) -> Self {
        Self {
            adjacency_list: HashMap::new(),
            is_weighted,
            is_oriented,
        }
    }

    /// Returns the adjacency list of this Graph.
    pub fn get_adjacency_list(&self) -> HashMap<String, HashMap<String, Option<EdgeWeight>>> {
        self.adjacency_list.clone()
    }

    pub fn set_adjacency_list(&mut self, al: HashMap<String, HashMap<String, Option<EdgeWeight>>>) {
        self.adjacency_list = al;
    }

    /// Check if graph is weighted.
    pub fn is_weighted(&self) -> bool {
        self.is_weighted
    }

    /// Check if graph is oriented.
    pub fn is_oriented(&self) -> bool {
        self.is_oriented
    }

    /// Create new graph from given file.
    pub fn from_file(path: String) -> GraphResult<Self> {
        let in_file = File::open(path)?;
        let mut buf_reader = BufReader::new(in_file);

        let mut graph_description = String::new();
        buf_reader.read_line(&mut graph_description)?;
        let mut not = false;
        let mut is_weighted = None;
        let mut is_oriented = None;
        for word in graph_description.split_whitespace() {
            match word {
                "not" => not = !not,
                "oriented" => {
                    is_oriented = Some(!not);
                    not = false
                }
                "weighted" => {
                    is_weighted = Some(!not);
                    not = false
                }
                _ => {
                    return Err(GraphError {
                        msg: format!("Unknown word in graph description: {word:?}"),
                    })
                }
            }
        }
        if is_weighted.is_none() || is_oriented.is_none() {
            return Err(GraphError {
                msg: format!(
                    "Cannot determine if graph is {} or not",
                    if is_weighted.is_none() {
                        "weighted"
                    } else {
                        "oriented"
                    }
                ),
            });
        }
        let is_weighted = is_weighted.unwrap();
        let is_oriented = is_oriented.unwrap();

        let mut adjacency_list: HashMap<String, HashMap<String, Option<EdgeWeight>>> =
            HashMap::new();
        for line in buf_reader.lines() {
            let line = line?;
            let (node_name, connections_str) = line.split_once(':').ok_or_else(|| GraphError {
                msg: "Invalid syntax".to_string(),
            })?;

            let mut connections: HashMap<String, Option<EdgeWeight>> = HashMap::new();
            for c in connections_str
                .split(',')
                .map(|c| c.trim())
                .filter(|c| !c.is_empty())
            {
                if !is_weighted {
                    connections.insert(c.to_string(), None);
                    continue;
                }

                let (con_node, rest) = c.split_once('(').ok_or_else(|| GraphError {
                    msg: "Weight of connection was not provided in weighted graph".to_string(),
                })?;

                let weight = rest.trim_end_matches(')').parse::<EdgeWeight>()?;
                connections.insert(con_node.to_string(), Some(weight));
            }
            adjacency_list.insert(node_name.to_string(), connections);
        }

        let gr = Self {
            adjacency_list,
            is_weighted,
            is_oriented,
        };
        gr.validate()?;

        Ok(gr)
    }

    /// Save graph to file.
    pub fn save_to_file(&self, path: &String) -> std::io::Result<()> {
        let mut out_file = File::create(path)?;
        out_file.write_all(self.pretty_view().as_bytes())?;
        out_file.sync_all()?;
        Ok(())
    }

    /// Get a multiline string representing graph using adjacency list.
    pub fn pretty_view(&self) -> String {
        let mut al = String::new();

        if !self.is_weighted {
            al.push_str("not ");
        }
        al.push_str("weighted ");
        if !self.is_oriented {
            al.push_str("not ");
        }
        al.push_str("oriented");

        for node in self.get_nodes() {
            al.push_str(&format!("\n{}: ", &node));
            for (connection, weight) in &self.adjacency_list[&node] {
                al.push_str(connection);
                if let Some(w) = weight {
                    al.push_str(&format!("({})", &w.to_string()));
                }
                al.push_str(", ");
            }
            al = al.trim_end_matches(", ").to_string();
        }

        al
    }

    /// Checks if graph is valid or not
    fn validate(&self) -> GraphResult<()> {
        // Check that all nodes in right exists
        for (key, connections) in &self.adjacency_list {
            for node in connections.keys() {
                if !self.adjacency_list.contains_key(node) {
                    let mut msg = format!("{key:?} connects with not existing node {node:?}");
                    if node.contains(' ') {
                        msg.push_str(": Maybe missing a comma?");
                    }
                    return Err(GraphError { msg });
                }
            }
        }

        // Check if not oriented
        if self.is_oriented {
            return Ok(());
        }
        for (key, connections) in &self.adjacency_list {
            for node in connections.keys() {
                if !self.adjacency_list[node].contains_key(key)
                    || self.adjacency_list[node][key] != self.adjacency_list[key][node]
                {
                    return Err(GraphError {
                        msg: format!(
                            "Graph is not oriented, but connection between \
                             {key:?} and {node:?} is not symmetric"
                        ),
                    });
                }
            }
        }
        // Check for propper weights
        for (key, connections) in &self.adjacency_list {
            for (node, weight) in connections {
                if self.is_weighted && weight.is_none() {
                    return Err(GraphError {
                        msg: format!(
                            "Graph is weighted, but weight of connection between \
                             {key:?} and {node:?} is `None`"
                        ),
                    });
                } else if !self.is_weighted && weight.is_some() {
                    return Err(GraphError {
                        msg: format!(
                            "Graph is not weighted, but weight of connection between \
                            {key:?} and {node:?} was specified"
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    /// Get a vec of all nodes stored in graph.
    pub fn get_nodes(&self) -> Vec<String> {
        let mut nodes = self
            .adjacency_list
            .iter()
            .map(|(node, _)| node.to_string())
            .collect::<Vec<_>>();
        nodes.sort();

        nodes
    }

    /// Get a vec of all edges stored in graph.
    pub fn get_edges(&self) -> Vec<Edge> {
        let mut edges: Vec<Edge> = vec![];
        for (node1, connections) in &self.adjacency_list {
            for (node2, weight) in connections {
                edges.push((node1.to_owned(), node2.to_owned(), *weight));
            }
        }

        edges
    }

    /// Add new node to the graph.
    pub fn push_node(&mut self, name: String) -> GraphResult<String> {
        match self.adjacency_list.insert(name.clone(), HashMap::new()) {
            Some(_) => Err(GraphError {
                msg: format!("Node {name:?} already exists"),
            }),
            None => Ok(name),
        }
    }

    /// Remove node from the graph.
    pub fn pop_node(&mut self, node: String) -> GraphResult<HashMap<String, Option<EdgeWeight>>> {
        let rv = self
            .adjacency_list
            .remove(&node)
            .ok_or_else(|| GraphError {
                msg: format!("Node {node:?} does not exist"),
            })?;

        let mut dead_connections: Vec<String> = Vec::new();
        for (it_node, connections) in &self.adjacency_list {
            if connections.contains_key(&node) {
                dead_connections.push(it_node.to_string());
            }
        }

        for it_node in dead_connections {
            self.adjacency_list.get_mut(&it_node).unwrap().remove(&node);
        }

        Ok(rv)
    }

    /// Add new edge to the graph.
    pub fn push_edge(
        &mut self,
        node1: String,
        node2: String,
        weight: Option<EdgeWeight>,
    ) -> GraphResult<()> {
        if self.is_weighted && weight.is_none() {
            return Err(GraphError {
                msg: "Weight was not specified in weighted graph".to_string(),
            });
        }
        if !self.is_weighted && weight.is_some() {
            return Err(GraphError {
                msg: "For some reason weight was provided in weighted graph".to_string(),
            });
        }
        if !self.adjacency_list.contains_key(&node2) {
            return Err(GraphError {
                msg: format!("Node {node2:?} does not exist"),
            });
        }

        let node1_connections = self
            .adjacency_list
            .get_mut(&node1)
            .ok_or_else(|| GraphError {
                msg: format!("Node {node1:?} does not exist"),
            })?;

        node1_connections.insert(node2.clone(), weight);

        if !self.is_oriented {
            self.adjacency_list
                .get_mut(&node2)
                .unwrap()
                .insert(node1.to_owned(), weight);
        }

        Ok(())
    }

    /// Remove an edge from graph.
    pub fn pop_edge(&mut self, node1: String, node2: String) -> GraphResult<Option<EdgeWeight>> {
        if !self.adjacency_list.contains_key(&node2) {
            return Err(GraphError {
                msg: format!("Node {node2:?} does not exist"),
            });
        }

        let node1_connection = self
            .adjacency_list
            .get_mut(&node1)
            .ok_or_else(|| GraphError {
                msg: format!("Node {node1:?} does not exist"),
            })?;

        let rv = node1_connection.remove(&node2).ok_or_else(|| GraphError {
            msg: "There is no such connection".to_string(),
        })?;

        if !self.is_oriented {
            self.adjacency_list.get_mut(&node2).unwrap().remove(&node1);
        }

        Ok(rv)
    }
}
