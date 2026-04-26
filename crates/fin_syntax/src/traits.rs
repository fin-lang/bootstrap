use crate::{Lang, ast, find_child};
use rowan::ast::AstNode;

pub trait HasName: AstNode<Language = Lang> {
    fn name(&self) -> Option<ast::Name> {
        find_child(&self.syntax())
    }
}

pub trait HasExpr: AstNode<Language = Lang> {
    fn expr(&self) -> Option<ast::Expr> {
        find_child(&self.syntax())
    }
}
