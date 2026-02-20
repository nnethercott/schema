use std::{
    collections::HashMap,
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
};

use roaring::RoaringBitmap;
use serde::Deserialize;

use crate::Result;
pub(crate) type Attributes = HashMap<String, Value>;
pub(crate) type NodeId = usize;

static ATOMIC_UID: AtomicUsize = AtomicUsize::new(0);

#[derive(Deserialize, PartialEq, Clone)]
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

impl From<bool> for Value {
    fn from(bool: bool) -> Self {
        Value::Boolean { bool }
    }
}

impl From<u32> for Value {
    fn from(int: u32) -> Self {
        Value::Integer { int }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String { string: value }
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(value: Vec<T>) -> Self {
        Value::List {
            list: value.into_iter().map(Into::into).collect(),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Edge {
    sink: NodeId,
    attrs: Attributes,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Node {
    id: NodeId,
    edges: Vec<Edge>,
    attrs: Attributes,
}

impl Node {
    pub fn is_leaf(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn get(&self, k: &str) -> Option<&Value> {
        self.attrs.get(k)
    }
}

//NOTE: i'm making the assumption that graphs are already serialized with nodes in order
#[derive(Deserialize, Debug)]
pub struct Graph(Vec<Node>);

impl Graph {
    pub fn deser(value: serde_json::Value) -> Result<Self> {
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

    pub fn ids(&self) -> RoaringBitmap {
        RoaringBitmap::from_iter(self.iter().map(|node| node.id as u32))
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Node> {
        self.0.iter()
    }

    pub fn root(&self) -> Option<&Node> {
        self.iter().next()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Node> {
        self.0.iter_mut()
    }

    pub fn iter_leafs(&self) -> impl Iterator<Item = &Node> {
        self.iter().filter(|n| n.is_leaf())
    }

    pub fn iter_leafs_mut(&mut self) -> impl Iterator<Item = &mut Node> {
        self.iter_mut().filter(|n| n.is_leaf())
    }
}

macro_rules! edge {
    ($sink:expr => $(($label:literal,$value:expr)),+ ) => {
        {
            let mut attrs: HashMap<String, Value> = HashMap::new();
            $(
                attrs.insert(format!("{}", $label), Value::from($value));
            )*
            Edge {
                sink: $sink,
                attrs,
            }
        }
    };
}

pub fn merge<R>(graphs: &mut [Graph], refers_to: R)
where
    R: Fn(&Node, &Node) -> bool,
{
    // lookup table
    let root_nodes: Vec<Node> = graphs
        .iter()
        .filter_map(|g| g.root().map(|n| n.clone()))
        .collect();

    // FIXME: O(n^3) :// 
    // observe: each graph can be processed independently
    for g in graphs.iter_mut() {
        for leaf in g.iter_leafs_mut() {
            for node in &root_nodes {
                if refers_to(leaf, node) {
                    // add some check on if node.id in g.ids()
                    leaf.edges.push(edge!(node.id => ("foreign", true)));
                }
            }
        }
    }
}
