use super::macros::ast_type;
use crate::{
    astgen::{
        find_child, find_token,
        traits::{HasExpr, HasName},
    },
    cstgen::{Lang, SyntaxKind},
};
use rowan::ast::AstNode;

pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;
pub type SyntaxNodePtr = rowan::ast::SyntaxNodePtr<Lang>;

#[derive(Clone, Debug, PartialEq)]
pub enum Visibility {
    Default,
    Public,
}

ast_type! {
    /// A name.
    pub struct Name(SyntaxKind::Name);
}

impl Name {
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        find_token(&self.syntax, SyntaxKind::Ident)
    }
}

ast_type! {
    /// An ident pattern.
    pub struct PatIdent(SyntaxKind::PatIdent);
}

impl HasName for PatIdent {}

ast_type! {
    /// A pattern.
    pub enum Pat {
        Ident(PatIdent)
    }
}

ast_type! {
    /// A `let` statement.
    pub struct StmtLocal(SyntaxKind::StmtLocal);
}

ast_type! {
    /// An expression statement.
    pub struct StmtExpr(SyntaxKind::StmtExpr);
}

ast_type! {
    /// A code statement.
    pub enum Stmt {
        Local(StmtLocal),
        Expr(StmtExpr)
    }
}

ast_type! {
    /// A binary expression.
    pub struct ExprBinary(SyntaxKind::ExprBinary);
}

impl ExprBinary {
    pub fn lhs(&self) -> Option<Expr> {
        self.syntax.children().filter_map(Expr::cast).nth(0)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.syntax.children().filter_map(Expr::cast).nth(1)
    }
}

ast_type! {
    /// A block expression.
    pub struct ExprBlock(SyntaxKind::ExprBlock);
}

impl ExprBlock {
    pub fn stmts(&self) -> impl Iterator<Item = Stmt> {
        self.syntax.children().filter_map(Stmt::cast)
    }
}

ast_type! {
    /// A call expression.
    pub struct ExprCall(SyntaxKind::ExprCall);
}

impl ExprCall {
    pub fn callee(&self) -> Option<Expr> {
        self.syntax.children().find_map(Expr::cast)
    }

    pub fn args(&self) -> Option<impl Iterator<Item = Expr>> {
        self.syntax.children().find_map(|node| match node.kind() {
            SyntaxKind::CallArgs => Some(node.children().filter_map(Expr::cast)),
            _ => None,
        })
    }
}

ast_type! {
    /// A closure expression.
    pub struct ExprClosure(SyntaxKind::ExprClosure);
}

impl ExprClosure {}

ast_type! {
    /// An `enum` expression.
    pub struct ExprEnum(SyntaxKind::ExprEnum);
}

impl ExprEnum {}

ast_type! {
    /// A field expression.
    pub struct ExprField(SyntaxKind::ExprField);
}

impl ExprField {}

ast_type! {
    /// An ident expression.
    pub struct ExprIdent(SyntaxKind::ExprIdent);
}

impl ExprIdent {}

ast_type! {
    /// An `if` expression.
    pub struct ExprIf(SyntaxKind::ExprIf);
}

impl ExprIf {
    pub fn condition(&self) -> Option<Expr> {
        self.syntax.children().find_map(Expr::cast)
    }

    pub fn then_block(&self) -> Option<ExprBlock> {
        self.syntax.children().filter_map(ExprBlock::cast).nth(0)
    }

    pub fn else_block(&self) -> Option<ExprBlock> {
        self.syntax.children().filter_map(ExprBlock::cast).nth(1)
    }
}

ast_type! {
    /// A macro call expression.
    pub struct ExprMacro(SyntaxKind::ExprMacro);
}

impl ExprMacro {}

ast_type! {
    /// A parenthesized expression.
    pub struct ExprParen(SyntaxKind::ExprParen);
}

impl ExprParen {
    pub fn body(&self) -> Option<Expr> {
        self.syntax.children().find_map(Expr::cast)
    }
}

ast_type! {
    /// A `struct` expression.
    pub struct ExprStruct(SyntaxKind::ExprStruct);
}

impl ExprStruct {}

ast_type! {
    /// A `trait` expression.
    pub struct ExprTrait(SyntaxKind::ExprTrait);
}

impl ExprTrait {}

ast_type! {
    /// A fin expression.
    pub enum Expr {
        Binary(ExprBinary),
        Block(ExprBlock),
        Call(ExprCall),
        Closure(ExprClosure),
        Enum(ExprEnum),
        Field(ExprField),
        Ident(ExprIdent),
        If(ExprIf),
        Macro(ExprMacro),
        Paren(ExprParen),
        Struct(ExprStruct),
        Trait(ExprTrait)
    }
}

ast_type! {
    /// A constant item.
    pub struct ItemConst(SyntaxKind::ItemConst);
}

impl ItemConst {
    pub fn pat(&self) -> Option<Pat> {
        find_child(&self.syntax)
    }
}

impl HasExpr for ItemConst {}

ast_type! {
    /// An expression item.
    pub struct ItemExpr(SyntaxKind::ItemExpr);
}

impl HasExpr for ItemExpr {}

ast_type! {
    /// Items inside a namespace.
    pub enum Item {
        Const(ItemConst),
        Expr(ItemExpr)
    }
}

ast_type! {
    /// A fin source file.
    pub struct File(SyntaxKind::File);
}

impl File {
    pub fn items(&self) -> impl Iterator<Item = Item> {
        self.syntax.children().filter_map(Item::cast)
    }
}
