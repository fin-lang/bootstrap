macro_rules! ast_type {
    (
        $(#[$doc:meta])*
        $vis:vis struct $ty:ident($kind:path);
    ) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        $vis struct $ty {
            syntax: super::ast::SyntaxNode,
        }

        impl $ty {
            const KIND: super::ast::SyntaxKind = $kind;
        }

        impl rowan::ast::AstNode for $ty {
            type Language = super::ast::Lang;

            #[inline]
            fn can_cast(kind: SyntaxKind) -> bool {
                kind == $kind
            }

            #[inline]
            fn cast(syntax: SyntaxNode) -> Option<Self> {
                if Self::can_cast(syntax.kind()) {
                    Some(Self { syntax })
                } else {
                    None
                }
            }

            #[inline]
            fn syntax(&self) -> &super::ast::SyntaxNode {
                &self.syntax
            }
        }
    };

    (
        $(#[$doc:meta])*
        $vis:vis enum $name:ident { $($key:ident($ty:ident)),* }
    ) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        $vis enum $name {
            $(
                $key($ty),
            )*
        }

        impl rowan::ast::AstNode for $name {
            type Language = super::ast::Lang;

            fn can_cast(kind: SyntaxKind) -> bool {
                match kind {
                    $(
                        $ty::KIND => true,
                    )*
                    _ => false,
                }
            }

            fn cast(syntax: SyntaxNode) -> Option<Self> {
                match syntax.kind() {
                    $(
                        $ty::KIND => Some($name::$key($ty::cast(syntax)?)),
                    )*
                    _ => None,
                }
            }

            fn syntax(&self) -> &super::ast::SyntaxNode {
                match self {
                    $(
                        $name::$key(node) => &node.syntax,
                    )*
                }
            }
        }
    };
}

/// Matches an AST.
#[macro_export]
macro_rules! match_ast {
    ($node:expr, {
        $( $( $path:ident )::+ ($it:pat) => $res:expr, )*
        _ => $else:expr $(,)?
    }) => {{
        $( if let Some($it) = $($path::)+cast($node.clone()) { $res } else )*
        { $else }
    }};
}

pub(crate) use ast_type;
pub use match_ast;
