//!

use harper_core::parsers::{self, Parser, PlainEnglish};
use harper_core::{Token, TokenKind};
use harper_tree_sitter::TreeSitterMasker;
use tree_sitter::Node;

/// A parser for AsciiDoc files.
pub struct AsciiDocParser {
    inner: parsers::Mask<TreeSitterMasker, PlainEnglish>,
}

impl AsciiDocParser {
    /// Determines which tree-sitter nodes should be parsed for linting.
    fn node_condition(n: &Node) -> bool {
        match n.kind() {
            "line" | "body" | "table_cell_content" => true,
            _ => false,
        }
    }
}

impl Default for AsciiDocParser {
    fn default() -> Self {
        Self {
            inner: parsers::Mask::new(
                TreeSitterMasker::new(tree_sitter_asciidoc::language(), Self::node_condition),
                PlainEnglish,
            ),
        }
    }
}

impl Parser for AsciiDocParser {
    fn parse(&self, source: &[char]) -> Vec<Token> {
        let mut tokens = self.inner.parse(source);

        for token in &mut tokens {
            if let TokenKind::Space(v) = &mut token.kind {
                *v = (*v).clamp(0, 1);
            }
        }

        tokens
    }
}
