use std::{
    collections::HashMap,
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
};

use serde::Deserialize;

use crate::Result;
pub(crate) type Attributes = HashMap<String, Value>;
pub(crate) type NodeId = usize;

static ATOMIC_UID: AtomicUsize = AtomicUsize::new(0);

//FIXME: change serialize into flattened

#[derive(Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Value {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "bool")]
    Boolean { bool: bool },
    #[serde(rename = "int")]
    Integer { int: u32 },
    #[serde(rename = "string")]
    String { string: String },
    #[serde(rename = "list")]
    List { list: Vec<Value> },
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Boolean { bool } => write!(f, "{bool}"),
            Self::Integer { int } => write!(f, "{int}"),
            Self::String { string } => write!(f, "{string}"),
            Self::List { list } => write!(f, "{:?}", list),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, PartialEq)]
pub struct Node {
    id: NodeId,
    edges: Vec<Edge>,
    attrs: Attributes,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, PartialEq)]
pub struct Edge {
    sink: NodeId,
    attrs: Attributes,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Graph(Vec<Node>);

//NOTE: i'm making the assumption that graphs are already serialized with nodes in order
impl Graph {
    pub fn deser(value: serde_json::Value)->Result<Self>{
        let mut graph = serde_json::from_value::<Graph>(value)?;
        graph.init();
        Ok(graph)
    }

    /// ensure subgraphs each have unique node ids
    fn init(&mut self) {
        for node in self.iter_mut() {
            node.id = ATOMIC_UID.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Node> {
        // we want to iterate over the graph in order from start to finish
        self.0.iter_mut()
    }
}
