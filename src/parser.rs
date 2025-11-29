use core::panic;

use crate::lexer::{Token, TokenType};

enum ParserError {
    UnclosedParen,
}

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

// TODO look into generating this via macro

///
enum Expr<'a> {
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Literal(Token<'a>),
    Identifier(Token<'a>),
    Grouping(Box<Expr<'a>>),
}

impl<'a> Default for Parser<'a> {
    fn default() -> Self {
        Parser {
            tokens: vec![],
            current: 0,
        }
    }
}

impl<'a> Parser<'a> {
    fn parse(&self) -> Expr {
        // TODO update return types to Result<Expr, ParserError>
        let result = self.expression();
        result
    }

    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            // TODO
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        expr
    }
    fn consume(&self, token_type: TokenType, message: &str) -> Result<Token, ParserError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        panic!("{}", message);
    }

    fn primary(&self) -> Option<Expr> {
        if self.matches(&[TokenType::False]) {
            return Some(Expr::Literal(self.previous()));
        } else if self.matches(&[TokenType::True]) {
            return Some(Expr::Literal(self.previous()));
        } else if self.matches(&[TokenType::Constant]) {
            return Some(Expr::Literal(self.previous()));
        } else if self.matches(&[TokenType::LParen]) {
            let expr = self.expression();
            self.consume(TokenType::RParen, "Expect ')' after expression");
            return Some(Expr::Grouping(Box::new(expr)));
        }

        None
    }

    fn unary(&self) -> Expr {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary();
            return Expr::Unary {
                operator: op,
                right: Box::new(right),
            };
        }

        self.primary().expect("something went wrong here?")
    }

    fn factor(&self) -> Expr {
        let mut expr = self.unary();
        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            }
        }

        expr
    }

    fn term(&self) -> Expr {
        let mut expr = self.factor();

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn comparison(&self) -> Expr {
        let expr: Expr = self.term();

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for &ttype in types {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.eof() {
            return false;
        }
        return self.peek().token_type == token_type;
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn eof(&self) -> bool {
        if self.peek().token_type == TokenType::EOF {
            return true;
        }
        false
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> Token {
        if !self.eof() {
            self.current += 1;
        }
        self.previous()
    }
}
