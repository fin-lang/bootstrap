use logos::Logos;

#[derive(Logos, Debug, PartialEq, Copy, Clone, Hash, Eq, Ord, PartialOrd)]
#[repr(u16)]
pub enum SyntaxKind {
    #[regex(r#"//[^\r\n]*"#, allow_greedy = true)]
    Comment,

    #[regex("[\\p{XID_Start}_]\\p{XID_Continue}*")]
    Ident,

    #[token("=")]
    Eq,
    #[token(";")]
    Semi,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("!")]
    Bang,
    #[token("?")]
    Question,
    #[token("#")]
    Pound,
    #[token("&")]
    Ampersand,
    #[token("&!")]
    AmpersandBang,
    #[token("*")]
    Asterisk,
    #[token("=>")]
    EqGt,
    #[token("|")]
    Pipe,
    #[token("/")]
    Slash,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("-")]
    Minus,
    #[token("->")]
    Arrow,

    #[token("let")]
    LetKw,
    #[token("pub")]
    PubKw,
    #[token("fn")]
    FnKw,
    #[token("const")]
    ConstKw,
    #[token("enum")]
    EnumKw,
    #[token("trait")]
    TraitKw,
    #[token("impl")]
    ImplKw,
    #[token("struct")]
    StructKw,
    #[token("match")]
    MatchKw,
    #[token("defer")]
    DeferKw,
    #[token("mut")]
    MutKw,
    #[token("for")]
    ForKw,
    #[token("if")]
    IfKw,
    #[token("else")]
    ElseKw,

    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,

    Unknown,

    #[regex("\\s+")]
    Whitespace,
    #[end]
    Eof,

    // Composites
    File,
    ItemConst,
    ItemExpr,
    Expr,
    ExprClosure,
    ExprRef,
    ExprPtr,
    ExprBlock,
    ExprMacro,
    ExprParen,
    ExprTrait,
    ExprStruct,
    ExprEnum,
    ExprIdent,
    ExprImpl,
    ExprIf,
    ExprBinary,
    ArgList,
    Arg,
    ExprField,
    ExprCall,
    Statement,
    StructField,
    StmtExpr,
    StmtLocal,
    Type,
    CallArgs,
    PatIdent,
    Name,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}
impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
