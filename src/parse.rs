//! Uses [tree-sitter][1] to parse files using [S-expressions][2].
//!
//! [1]: https://tree-sitter.github.io/tree-sitter/using-parsers/queries/2-operators.html
//! [2]: https://en.wikipedia.org/wiki/S-expression

use std::fmt::Display;

use ouroboros::self_referencing;
use tree_sitter::{Node, Query, QueryCursor, QueryMatches, StreamingIterator, TextProvider};

fn build_py_query(s_expr: &'static str) -> Query {
    let lang = tree_sitter_python::LANGUAGE.into();
    Query::new(&lang, s_expr).unwrap()
}

#[derive(Debug, Clone)]
pub struct Noeud<'a, 'tree>
where
    'tree: 'a,
{
    pub node: Node<'tree>,
    pub ctx: &'a [u8],
}

impl<'a, 'tree> Display for Noeud<'a, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // markdown-flavoured
        f.write_str(&format!(
            "Lines: {}-{}\n```python\n{}\n```",
            self.node.start_position().row,
            self.node.end_position().row,
            self.ctx_as_str()
        ))
    }
}

impl<'a, 'tree> Noeud<'a, 'tree> {
    pub fn new(node: Node<'tree>, code: &'a [u8]) -> Self {
        Self { node, ctx: code }
    }

    pub fn ctx_as_str(&self) -> &str {
        // Safety: string bounds parsed by TS so byte slice should always be valid utf8
        unsafe { str::from_utf8_unchecked(self.ctx) }
    }

    pub fn parse(&self, s_expr: &'static str) -> NoeudIter<'a, 'tree> {
        let query = build_py_query(s_expr);
        let (node, ctx) = (self.node, self.ctx);

        NoeudIterBuilder {
            query,
            ctx,
            builder: QueryCursor::new(),
            cursor_builder: |builder, q| builder.matches(q, node, ctx),
        }
        .build()
    }
}

#[self_referencing]
pub struct NoeudIter<'a, 'tree>
where
    'tree: 'a,
{
    ctx: &'a [u8],
    query: Query,
    builder: QueryCursor,

    // FIXME: later unpack this...
    #[borrows(mut builder, query)]
    #[not_covariant]
    cursor: QueryMatches<'this, 'tree, &'a [u8], &'a [u8]>,
}

// TODO: turn on proc macros to see types later
impl<'a, 'tree> Iterator for NoeudIter<'a, 'tree> {
    type Item = Noeud<'a, 'tree>;

    fn next(&mut self) -> Option<Self::Item> {
        let ctx = *self.borrow_ctx();

        self.with_cursor_mut(|cur| {
            if let Some(item) = cur.next() {
                let node = item.captures[0].node;
                let slice = &ctx[node.start_byte()..node.end_byte()];
                Some(Noeud::new(node, slice))
            } else {
                None
            }
        })
    }
}
