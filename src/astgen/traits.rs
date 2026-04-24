use rowan::ast::AstNode;

use crate::astgen::ast;
use crate::astgen::find_child;
use crate::cstgen::Lang;

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
