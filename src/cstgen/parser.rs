use crate::cstgen::{ControlFlow, SyntaxKind, trivia};
use logos::{Logos, Span, SpannedIter};
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder};
use std::iter::Peekable;

pub struct Cursor<'a> {
    src: &'a str,
    lexer: Peekable<SpannedIter<'a, SyntaxKind>>,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            lexer: SyntaxKind::lexer(src).spanned().peekable(),
        }
    }

    pub fn peek(&mut self) -> SyntaxKind {
        match self.lexer.peek() {
            Some((Ok(kind), _)) => *kind,
            Some((Err(_), _)) => SyntaxKind::Unknown,
            None => SyntaxKind::Eof,
        }
    }

    pub fn at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == kind
    }

    pub fn advance(&mut self) -> Option<Span> {
        match self.lexer.next() {
            Some((_, span)) => Some(span),
            _ => None,
        }
    }

    pub fn eat(&mut self, kind: SyntaxKind) -> Option<Span> {
        self.at(kind).then(|| self.advance())?
    }
}

pub struct Parser<'a> {
    builder: GreenNodeBuilder<'static>,
    cursor: Cursor<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            cursor: Cursor::new(src),
        }
    }

    pub fn node(&mut self, kind: SyntaxKind, func: impl FnOnce(&mut Self)) {
        self.builder.start_node(kind.into());
        func(self);
        self.builder.finish_node();
    }

    pub fn node_at(
        &mut self,
        kind: SyntaxKind,
        checkpoint: Checkpoint,
        func: impl FnOnce(&mut Self),
    ) {
        self.builder.start_node_at(checkpoint, kind.into());
        func(self);
        self.builder.finish_node();
    }

    pub fn expect(&mut self, kind: SyntaxKind) {
        match self.eat(kind) {
            Some(span) => {
                self.builder.token(kind.into(), &self.cursor.src[span]);
                trivia(self);
            }
            None => {
                self.eat(kind);
            }
        }
    }

    pub fn bump(&mut self) {
        let kind = self.cursor.peek();
        let Some(span) = self.eat(kind) else { return };
        self.builder.token(kind.into(), &self.cursor.src[span]);
        trivia(self);
    }

    pub fn expect_optional(&mut self, kind: SyntaxKind) -> ControlFlow {
        if self.cursor.at(kind) {
            self.expect(kind);
            ControlFlow::Continue
        } else {
            ControlFlow::Break
        }
    }

    pub fn at(&mut self, kind: SyntaxKind) -> bool {
        self.cursor.at(kind)
    }

    pub fn peek(&mut self) -> SyntaxKind {
        self.cursor.peek()
    }

    pub fn checkpoint(&mut self) -> Checkpoint {
        self.builder.checkpoint()
    }

    pub fn finish(self) -> GreenNode {
        self.builder.finish()
    }

    fn eat(&mut self, kind: SyntaxKind) -> Option<Span> {
        let result = self.cursor.eat(kind);
        // TODO: Error handling
        result
    }
}
