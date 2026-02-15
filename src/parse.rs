//! Uses [tree-sitter][1] to parse files using [S-expressions][2].
//!
//! [1]: https://tree-sitter.github.io/tree-sitter/using-parsers/queries/2-operators.html
//! [2]: https://en.wikipedia.org/wiki/S-expression

use std::fmt::Debug;
use streaming_iterator::StreamingIterator;

use ouroboros::self_referencing;
use tree_sitter::{Node, Query, QueryCursor, QueryMatches};

#[derive(Clone)]
pub struct Noeud<'a, 'tree>
where
    'tree: 'a,
{
    pub node: Node<'tree>,
    pub src: &'a [u8],
}

impl<'a, 'tree> Debug for Noeud<'a, 'tree> {
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
        Self { node, src: code }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.src[self.node.start_byte()..self.node.end_byte()]
    }

    pub fn ctx_as_str(&self) -> &str {
        // Safety: string bounds parsed by TS so byte slice should always be valid utf8
        unsafe { str::from_utf8_unchecked(self.bytes()) }
    }

    pub fn parse(&self, query: &'a Query) -> NoeudIter<'a, 'tree> {
        let Noeud { node, src: ctx } = self.clone();

        NoeudIterBuilder {
            query,
            src: ctx,
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
    src: &'a [u8],
    query: &'a Query,
    builder: QueryCursor,

    #[borrows(mut builder, query)]
    #[not_covariant]
    cursor: QueryMatches<'this, 'tree, &'a [u8], &'a [u8]>,
}

impl<'a, 'tree> Iterator for NoeudIter<'a, 'tree> {
    // (capture_group_id, node) pairs
    type Item = Vec<(&'a str, Noeud<'a, 'tree>)>;

    fn next(&mut self) -> Option<Self::Item> {
        let src = *self.borrow_src();
        let group_names = self.borrow_query().capture_names();

        self.with_cursor_mut(|cur| {
            if let Some(matches) = cur.next() {
                let next = matches
                    .captures
                    .iter()
                    .map(|item| {
                        let node = item.node;
                        let group = group_names[item.index as usize];
                        (group, Noeud::new(node, src))
                    })
                    .collect::<Vec<_>>();
                Some(next)
            } else {
                None
            }
        })
    }
}
