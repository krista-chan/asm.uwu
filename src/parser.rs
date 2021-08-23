use crate::ast::Spanned;
use logos::{Lexer as LogosLexer, Logos};

pub struct Lexer<'a> {
    tokens: LogosLexer<'a, Tokens>,
    position: (usize, usize),
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Tokens {
    // Punctuation
    #[token("*")]
    Star,

    #[regex(r">\.<.*")]
    LineComment,

    #[token("\n")]
    Newline,

    #[token(" ")]
    Whitespace,

    #[error]
    Error,
}

impl<'a> Lexer<'a> {
    pub fn new(lexer: LogosLexer<'a, Tokens>) -> Self {
        Lexer {
            tokens: lexer,
            position: (0, 0),
        }
    }

    pub fn next_token(&mut self) -> Option<Spanned<Tokens>> {
        let next = self.tokens.next();

        if let Some(token) = next {
            if token == Tokens::Newline {
                self.position.0 += 1;
                self.tokens.next();
            } else if token == Tokens::LineComment {
                self.position.1 += self.tokens.span().len();
                self.tokens.next();
            } else {
                self.position.1 += self.tokens.span().len();
            }

            Some(Spanned {
                data: token,
                span: self.tokens.span().into(),
            })
        } else {
            None
        }
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }
}
