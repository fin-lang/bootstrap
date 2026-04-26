mod parser;
mod syntax;

use parser::Parser;
use rowan::GreenNode;
pub use syntax::{Lang, SyntaxKind};

#[derive(PartialEq, Copy, Clone)]
enum ControlFlow {
    Break,
    Continue,
}

/// Lower fin source into a CST.
pub fn lower(src: &str) -> GreenNode {
    let mut parser = Parser::new(src);
    file(&mut parser);
    parser.finish()
}

fn file(p: &mut Parser) {
    p.node(SyntaxKind::File, |p| {
        trivia(p);
        while !p.at(SyntaxKind::Eof) {
            item(p);
        }
    });
}

fn punctuated(p: &mut Parser, func: impl Fn(&mut Parser) -> ControlFlow) {
    while func(p) == ControlFlow::Continue {}
}

fn item(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::PubKw | SyntaxKind::ConstKw => p.node(SyntaxKind::ItemConst, |p| {
            p.expect_optional(SyntaxKind::PubKw);
            p.expect(SyntaxKind::ConstKw);
            pattern(p);
            if p.at(SyntaxKind::Colon) {
                p.node(SyntaxKind::Type, |p| {
                    p.bump();
                    expr(p);
                });
            }
            if p.expect_optional(SyntaxKind::Eq) == ControlFlow::Continue {
                expr(p);
            }
            p.expect(SyntaxKind::Semi);
        }),
        _ => p.node(SyntaxKind::ItemExpr, |p| {
            expr(p);
            p.expect_optional(SyntaxKind::Semi);
        }),
    }
}

fn arg(p: &mut Parser, allow_closures: bool) {
    p.node(SyntaxKind::Arg, |p| {
        p.expect_optional(SyntaxKind::ConstKw);
        expr_prefix(p, allow_closures);
        if p.expect_optional(SyntaxKind::Colon) == ControlFlow::Continue {
            expr(p);
        }
    });
}

fn arg_list(p: &mut Parser, allow_closures: bool) {
    p.node(SyntaxKind::ArgList, |p| {
        punctuated(p, |p| {
            arg(p, allow_closures);
            p.expect_optional(SyntaxKind::Comma)
        });
    });
}

fn struct_field(p: &mut Parser) {
    name(p);
    p.expect(SyntaxKind::Colon);
    expr(p);
}

fn name(p: &mut Parser) {
    p.node(SyntaxKind::Name, |p| {
        p.expect(SyntaxKind::Ident);
    });
}

fn pattern(p: &mut Parser) {
    match p.peek() {
        SyntaxKind::Ident => p.node(SyntaxKind::PatIdent, |p| name(p)),
        _ => panic!("unexpected token in pattern"),
    }
}

fn statement(p: &mut Parser) -> ControlFlow {
    match p.peek() {
        SyntaxKind::LetKw => p.node(SyntaxKind::StmtLocal, |p| {
            p.bump();
            pattern(p);
            if p.expect_optional(SyntaxKind::Eq) == ControlFlow::Continue {
                expr(p);
            }
            p.expect(SyntaxKind::Semi);
        }),
        _ => {
            let checkpoint = p.checkpoint();
            if expr_inner(p, SyntaxKind::Eof) == ControlFlow::Break {
                return ControlFlow::Break;
            }
            p.node_at(SyntaxKind::StmtExpr, checkpoint, |p| {});
        }
    }
    ControlFlow::Continue
}

fn expr_block(p: &mut Parser) {
    p.node(SyntaxKind::ExprBlock, |p| {
        p.expect(SyntaxKind::BraceOpen);
        while statement(p) == ControlFlow::Continue {}
        p.expect(SyntaxKind::BraceClose);
    });
}

fn expr_inner(p: &mut Parser, left: SyntaxKind) -> ControlFlow {
    let checkpoint = p.checkpoint();
    if expr_prefix(p, true) == ControlFlow::Break {
        return ControlFlow::Break;
    }

    loop {
        let right = p.peek();
        if !right_binds_tighter(left, right) {
            break;
        }

        match right {
            SyntaxKind::ParenOpen => p.node_at(SyntaxKind::ExprCall, checkpoint, |p| {
                p.bump();
                p.node(SyntaxKind::CallArgs, |p| {
                    punctuated(p, |p| {
                        expr(p);
                        p.expect_optional(SyntaxKind::Comma)
                    });
                });
                p.expect(SyntaxKind::ParenClose);
            }),
            SyntaxKind::Dot => p.node_at(SyntaxKind::ExprField, checkpoint, |p| {
                p.bump();
                expr(p);
            }),
            _ => panic!("unexpected operator {:?}", right),
        }
    }
    ControlFlow::Continue
}

fn expr(p: &mut Parser) {
    expr_inner(p, SyntaxKind::Eof);
}

fn expr_prefix(p: &mut Parser, allow_closures: bool) -> ControlFlow {
    match p.peek() {
        SyntaxKind::Ampersand => p.node(SyntaxKind::ExprRef, |p| {
            p.bump();
            expr(p);
        }),
        SyntaxKind::Asterisk => p.node(SyntaxKind::ExprPtr, |p| {
            p.bump();
            expr(p);
        }),
        SyntaxKind::Ident => p.node(SyntaxKind::ExprIdent, |p| {
            p.bump();
        }),
        SyntaxKind::BraceOpen => expr_block(p),
        SyntaxKind::ParenOpen => p.node(SyntaxKind::ExprParen, |p| {
            p.bump();
            expr(p);
            p.expect(SyntaxKind::ParenClose);
        }),

        SyntaxKind::Pipe if allow_closures => p.node(SyntaxKind::ExprClosure, |p| {
            p.bump();
            arg_list(p, false);
            p.expect(SyntaxKind::Pipe);
            if p.expect_optional(SyntaxKind::Arrow) == ControlFlow::Continue {
                expr(p);
                if p.at(SyntaxKind::BraceOpen) {
                    expr(p);
                }
            } else {
                expr(p);
            }
        }),
        SyntaxKind::EnumKw => p.node(SyntaxKind::ExprEnum, |p| {
            p.bump();
            p.expect(SyntaxKind::BraceOpen);
            punctuated(p, |p| {
                expr(p);
                p.expect_optional(SyntaxKind::Comma)
            });
            p.expect(SyntaxKind::BraceClose);
        }),
        SyntaxKind::StructKw => p.node(SyntaxKind::ExprStruct, |p| {
            p.bump();
            p.expect(SyntaxKind::BraceOpen);
            punctuated(p, |p| {
                struct_field(p);
                p.expect_optional(SyntaxKind::Comma)
            });
            p.expect(SyntaxKind::BraceClose);
        }),
        SyntaxKind::TraitKw => p.node(SyntaxKind::ExprTrait, |p| {
            p.bump();
            p.expect(SyntaxKind::BraceOpen);
            while !p.at(SyntaxKind::BraceClose) {
                item(p);
            }
            p.expect(SyntaxKind::BraceClose);
        }),
        SyntaxKind::ImplKw => p.node(SyntaxKind::ExprImpl, |p| {
            p.bump();
            expr(p);
            p.expect(SyntaxKind::BraceOpen);
            while !p.at(SyntaxKind::BraceClose) {
                item(p);
            }
            p.expect(SyntaxKind::BraceClose);
        }),
        SyntaxKind::IfKw => p.node(SyntaxKind::ExprIf, |p| {
            p.bump();
            expr(p);
            expr_block(p);
        }),

        _ => {
            return ControlFlow::Break;
        }
    }
    ControlFlow::Continue
}

fn right_binds_tighter(left: SyntaxKind, right: SyntaxKind) -> bool {
    fn tightness(kind: SyntaxKind) -> Option<usize> {
        [
            [SyntaxKind::Dot],
            [SyntaxKind::Bang],
            [SyntaxKind::ParenOpen],
        ]
        .iter()
        .position(|level| level.contains(&kind))
    }

    let left_tightness = tightness(left);
    let right_tightness = tightness(right);

    match (left_tightness, right_tightness) {
        (_, None) => false,
        (None, _) => {
            assert!(left == SyntaxKind::Eof);
            true
        }
        _ => right_tightness > left_tightness,
    }
}

fn trivia(p: &mut Parser) {
    loop {
        match p.peek() {
            SyntaxKind::Comment => p.bump(),
            SyntaxKind::Whitespace => p.bump(),
            _ => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::{assert_snapshot, glob};
    use std::fs;

    #[test]
    fn test_insta() {
        glob!("snapshots/validation/*.fin", |path| {
            let input = fs::read_to_string(path).unwrap();
            let mut p = Parser::new(&input);
            file(&mut p);
            let tree = format!("{:#?}", rowan::SyntaxNode::<Lang>::new_root(p.finish()));
            assert_snapshot!(tree);
        });
    }
}
