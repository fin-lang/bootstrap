pub mod ast;
mod macros;
pub mod traits;

use ast::SyntaxNode;
use rowan::{GreenNode, ast::AstNode};

use crate::cstgen::{Lang, SyntaxKind};

/// Lowers a CST into an AST.
pub fn lower(root: GreenNode) -> Option<ast::File> {
    ast::File::cast(SyntaxNode::new_root(root))
}

pub(crate) fn find_child<T: AstNode<Language = Lang>>(node: &SyntaxNode) -> Option<T> {
    node.children().find_map(T::cast)
}

pub(crate) fn find_token(node: &SyntaxNode, kind: SyntaxKind) -> Option<ast::SyntaxToken> {
    node.children_with_tokens()
        .filter_map(|node| node.into_token())
        .find(|token| token.kind() == kind)
}
