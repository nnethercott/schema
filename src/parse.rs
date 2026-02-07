//! Uses [tree-sitter][1] to parse files using [S-expressions][2].
//!
//! [1]: https://tree-sitter.github.io/tree-sitter/using-parsers/queries/2-operators.html
//! [2]: https://en.wikipedia.org/wiki/S-expression

use std::fmt::Display;

use tree_sitter::{Node, Query, QueryCursor, StreamingIterator};

#[derive(Debug)]
pub struct NodeLike<'a, 'tree> {
    pub node: Node<'tree>,
    pub ctx: &'a [u8],
}

impl<'a, 'tree> NodeLike<'a, 'tree> {
    pub fn new(node: Node<'tree>, code: &'a [u8]) -> Self {
        Self { node, ctx: code }
    }

    pub fn ctx_as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.ctx) }
    }
}

impl<'a, 'tree> Display for NodeLike<'a, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // markdown-flavoured
        f.write_str(&format!("```python\n{}\n```", self.ctx_as_str()))
    }
}

pub fn parse<'a, 'tree>(
    root: NodeLike<'a, 'tree>,
    s_expr: &'static str,
) -> Vec<NodeLike<'a, 'tree>> {
    let lang = tree_sitter_python::LANGUAGE.into();
    let query = Query::new(&lang, s_expr).unwrap();

    let mut cursor = QueryCursor::new();
    let mut res = cursor.matches(&query, root.node, root.ctx);

    // parse matches into new sub-nodes for future exploration
    // each sub-node will store a slice pointing to its raw code range
    let mut nodes = vec![];

    while let Some(item) = res.next() {
        let node = item.captures[0];
        let slice = &root.ctx[node.start_byte()..node.end_byte()];
        nodes.push(NodeLike::new(node, slice));
    }

    nodes
}

// parse_decorator!(workflow.define,activity,signal,update,query)
