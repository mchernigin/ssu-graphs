use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};

macro_rules! safe_err {
    ($($arg:tt)*) => {{
        eprint!("\x1b[1;31mError\x1b[0m: \x1b[1m");
        eprint!($($arg)*);
        eprintln!("\x1b[0m");
    }};
}

macro_rules! or_err {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                safe_err!("{}\n", e);
                process::exit(1);
            }
        }
    };
}

macro_rules! or_escape {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                eprint!("\x1b[1;33mGoing back\x1b[0m: \x1b[1m");
                eprint!("{}", e);
                eprintln!("\x1b[0m\n");
                continue;
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct GraphError {
    msg: String,
}

impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<std::io::Error> for GraphError {
    fn from(e: std::io::Error) -> Self {
        GraphError { msg: format!("File error: {}", e) }
    }
}

impl From<std::num::ParseIntError> for GraphError {
    fn from(e: std::num::ParseIntError) -> Self {
        GraphError { msg: format!("Cannot parse connection weight: {}", e) }
    }
}

#[derive(Clone)]
pub struct Graph {
    adjacency_list: HashMap<String, HashMap<String, Option<i32>>>,
    is_weighted: bool,
    is_oriented: bool,
}

impl Graph {
    pub fn new(is_weighted: bool, is_oriented: bool) -> Self {
        Self {
            adjacency_list: HashMap::new(),
            is_weighted: is_weighted,
            is_oriented: is_oriented,
        }
    }

    pub fn is_weighted(&self) -> bool {
        self.is_weighted
    }

    pub fn from_file(path: &String) -> Result<Self, GraphError> {
        let in_file = File::open(path)?;
        let mut buf_reader = BufReader::new(in_file);

        let mut graph_description = String::new();
        buf_reader.read_line(&mut graph_description)?;
        let mut not = false;
        let mut is_weighted: Option<bool> = None;
        let mut is_oriented: Option<bool> = None;
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
                        msg: format!("Unknown word in graph description {:?}", word),
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

        let mut adjacency_list: HashMap<String, HashMap<String, Option<i32>>> = HashMap::new();
        for line in buf_reader.lines() {
            let line = line?;
            let (node_name, connections_str) = line.split_once(':').ok_or(GraphError {
                msg: "Invalid syntax".to_string(),
            })?;

            let mut connections: HashMap<String, Option<i32>> = HashMap::new();
            for c in connections_str
                .split(",")
                .map(|c| c.trim())
                .filter(|c| *c != "")
            {
                if !is_weighted {
                    connections.insert(c.to_string(), None);
                    continue;
                }

                let (con_node, rest) = c.split_once('(').ok_or(GraphError {
                    msg: "Weight of connection was not provided in weighted graph".to_string(),
                })?;

                let weight = rest.trim_end_matches(')').parse::<i32>()?;
                connections.insert(con_node.to_string(), Some(weight));
            }
            adjacency_list.insert(node_name.to_string(), connections);
        }

        Ok(Self {
            adjacency_list: adjacency_list,
            is_weighted: is_weighted,
            is_oriented: is_oriented,
        })
    }

    pub fn save_to_file(&self, path: &String) -> std::io::Result<()> {
        let mut out_file = File::create(path)?;
        out_file.write_all(self.pretty_view().as_bytes())?;
        out_file.sync_all()?;
        Ok(())
    }

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
            al.push('\n');
            al.push_str(&node);
            al.push_str(": ");
            for (connection, weight) in &self.adjacency_list[&node] {
                al.push_str(&connection);
                if let Some(w) = weight {
                    al.push('(');
                    al.push_str(&w.to_string());
                    al.push(')');
                }
                al.push_str(", ");
            }
            al = al.trim_end_matches(", ").to_string();
        }

        al
    }

    pub fn get_nodes(&self) -> Vec<String> {
        let mut nodes: Vec<String> = vec![];
        for (node, _) in &self.adjacency_list {
            nodes.push(node.to_string());
        }
        nodes.sort();

        nodes
    }

    pub fn push_node(&mut self, name: String) -> Result<String, GraphError> {
        if self
            .adjacency_list
            .insert(name.clone(), HashMap::new())
            .is_some()
        {
            Err(GraphError {
                msg: format!("Node {:?} already exists", name),
            })
        } else {
            Ok(name)
        }
    }

    pub fn push_edge(
        &mut self,
        node1: &String,
        node2: &String,
        weight: Option<i32>,
    ) -> Result<(), GraphError> {
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

        if !self.adjacency_list.contains_key(node2) {
            return Err(GraphError {
                msg: format!("Node {:?} does not exist", node2),
            });
        }

        let node1_connections = self.adjacency_list.get_mut(node1).ok_or(GraphError {
            msg: format!("Node {:?} does not exist", node2),
        })?;

        if node1_connections.insert(node2.clone(), weight).is_some() {
            return Err(GraphError {
                msg: format!("Nodes are already connected"),
            });
        }

        if !self.is_oriented {
            self.adjacency_list
                .get_mut(node2)
                .unwrap()
                .insert(node1.to_owned(), weight);
        }

        Ok(())
    }

    pub fn pop_edge(&mut self, node1: &String, node2: &String) -> Result<Option<i32>, GraphError> {
        if !self.adjacency_list.contains_key(node2) {
            return Err(GraphError {
                msg: format!("Node {:?} does not exist", node2),
            });
        }

        let node1_connection = self.adjacency_list.get_mut(node1).ok_or(GraphError {
            msg: format!("Node {:?} does not exist", node1),
        })?;

        let rv = node1_connection.remove(node2).ok_or(GraphError {
            msg: "There is no such connection".to_string(),
        })?;

        if !self.is_oriented {
            self.adjacency_list.get_mut(node2).unwrap().remove(node1);
        }

        Ok(rv)
    }

    pub fn pop_node(&mut self, node: String) -> Result<HashMap<String, Option<i32>>, GraphError> {
        let rv = self.adjacency_list.remove(&node).ok_or(GraphError {
            msg: format!("Node {:?} does not exist", node),
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
}
