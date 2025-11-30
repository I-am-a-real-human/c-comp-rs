use crate::lexer::{Token, TokenType};
use core::fmt;
use std::error::Error;
use std::fmt::Write;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub(crate) enum ParserError {
    UnclosedParen,
    UnknownPrimaryToken {
        line: usize,
        token_type: TokenType,
    },
    UnknownError,
    NoPreviousToken,
    ExpectedToken {
        expected: TokenType,
        found: Option<TokenType>,
        message: String,
    },
    UnexpectedEOF,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnclosedParen => write!(f, "Uncloses Parenthesis"),
            ParserError::UnexpectedEOF => write!(f, "Unexpected end of input"),
            ParserError::UnknownPrimaryToken { line, token_type } => {
                write!(
                    f,
                    "On line {}, unknown primary token '{:?}'",
                    line, token_type
                )
            }
            ParserError::ExpectedToken {
                expected, found, ..
            } => write!(f, "Expected token '{:?}', found '{:?}'", expected, found),
            ParserError::NoPreviousToken => write!(f, "No previous token"),
            ParserError::UnknownError => write!(f, "You're on your own pal"),
        }
    }
}

impl Error for ParserError {}

/// Representation of expression objects for creation of syntax tree. Contains
/// five types of expression objects:
/// * **Binary**: standard binary expression of <left> <operator> <right> (e.g.
/// 1 + 2)
/// * **Unary**: unary expression of form <operator> <right> (e.g. -1).
///
/// The remaining three are holding patterns for **Literal** (e.g. string or
/// numbers), **Identifier** (i.e. `int foo`) and **Grouping** (expressions
/// within parentheses)
pub(crate) enum Expr<'a> {
    Binary {
        left: Box<Expr<'a>>,
        operator: &'a Token<'a>,
        right: Box<Expr<'a>>,
    },
    Unary {
        operator: &'a Token<'a>,
        right: Box<Expr<'a>>,
    },
    Literal(&'a str),
    Identifier(&'a Token<'a>),
    Grouping(Box<Expr<'a>>),
}

pub(crate) enum Statement<'a> {
    Expression(Expr<'a>),
    Return { keyword: &'a Token<'a> , value: Option<Expr<'a>> },
    VarDecl { name: &'a Token<'a>, initialiser: Option<Expr<'a>> },
    Function {name: &'a Token<'a>, params: Vec<&'a Token<'a>>, body: Vec<Statement<'a>> },
}

impl<'a> Expr<'a> {
    pub fn print_tree(&self) -> String {
        let mut tree = String::new();
        Self::print_tree_unicode(self, &mut tree, 0, true);
        tree
    }

    fn print_tree_unicode(expr: &Self, output: &mut String, depth: usize, is_last: bool) {
        // TODO make indent customisable
        let indent = "  ".repeat(depth);
        let connector = if is_last { "└─ " } else { "├─ " };

        let type_name = match expr {
            Expr::Binary { .. } => "Binary",
            Expr::Unary { .. } => "Unary",
            Expr::Literal { .. } => "Literal",
            Expr::Grouping { .. } => "Grouping",
            _ => "Unknown",
        };

        let details = Self::format_node(expr);

        writeln!(
            output,
            "{}{}┌─ {} ({})",
            indent, connector, type_name, details
        )
        .unwrap();

        match expr {
            Expr::Binary {
                left,
                // operator,
                right,
                ..
            } => {
                Self::print_tree_unicode(left, output, depth + 1, false);
                // Self::print_tree_unicode(operator, output, depth + 1, false);
                Self::print_tree_unicode(right, output, depth + 1, true);
            }
            Expr::Unary { right, .. } => {
                // Self::print_tree_unicode(&**operator, output, depth + 1, false);
                Self::print_tree_unicode(right, output, depth + 1, true);
            }
            Expr::Grouping(expr) => {
                Self::print_tree_unicode(expr, output, depth + 1, true);
            }
            Expr::Literal { .. } | Expr::Identifier { .. } => (),
        }
    }

    fn format_node(expr: &Self) -> String {
        match expr {
            Expr::Binary { operator, .. } => format!("{:?}", operator.token_type),
            Expr::Unary { operator, .. } => format!("{:?}", operator.token_type),
            Expr::Literal(token) => format!("{:?}", token),
            Expr::Grouping(_) => "(...)".to_string(),
            Expr::Identifier(token) => format!("{:?}", token),
        }
    }
}

/// Simple recursive descent parser for the C language. Takes a iterable list
/// of `Token` enums and attempts to produce a AST from them.
///
/// * `tokens`: iterable list of `Token` enum objects (see Lexer.rs)
pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token<'a>>>,
    previous: Option<&'a Token<'a>>,
}

impl<'a> Default for Parser<'a> {
    fn default() -> Self {
        let empty_slice: &[Token<'a>] = &[];
        Self {
            tokens: empty_slice.iter().peekable(),
            previous: None,
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
            previous: None,
        }
    }

    fn peek(&mut self) -> Option<&Token<'a>> {
        self.tokens.peek().cloned()
    }

    fn previous(&mut self) -> Result<&'a Token<'a>, ParserError> {
        self.previous.ok_or(ParserError::NoPreviousToken)
    }

    fn expression(&mut self) -> Result<Expr<'a>, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr<'a>, ParserError> {
        let mut expr: Expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous()?;
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn consume(&mut self, expected: TokenType, message: &str) -> Result<&Token, ParserError> {
        if self.check(expected) {
            return self.advance();
        }

        let found = self.peek().map(|t| t.token_type);
        Err(ParserError::ExpectedToken {
            expected,
            found,
            message: message.to_string(),
        })
    }

    fn primary(&mut self) -> Result<Expr<'a>, ParserError> {
        if let Some(token) = self.peek() {
            match token.token_type {
                TokenType::False | TokenType::True | TokenType::Constant => {
                    let token = self.advance()?;
                    return Ok(Expr::Literal(token.literal));
                }
                TokenType::LParen | TokenType::LBrace => {
                    let _ = self.advance();
                    let expr = self.expression()?;
                    self.consume(TokenType::RParen, "Expect ')' after expression");
                    return Ok(Expr::Grouping(Box::new(expr)));
                }
                _ => {
                    return Err(ParserError::UnknownPrimaryToken {
                        line: token.line,
                        token_type: token.token_type,
                    });
                }
            }
        }

        Err(ParserError::UnknownError)
    }

    fn unary(&mut self) -> Result<Expr<'a>, ParserError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous()?;
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator: op,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn factor(&mut self) -> Result<Expr<'a>, ParserError> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous()?;
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr<'a>, ParserError> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous()?;
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr<'a>, ParserError> {
        let mut expr: Expr = self.term()?;

        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous()?;
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        if let Some(token) = self.peek() {
            if types.contains(&token.token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        // check if value matches input type, return false in all other situations
        self.peek().map_or(false, |t| t.token_type == token_type)
    }

    fn eof(&mut self) -> bool {
        if self.check(TokenType::EOF) {
            return true;
        }
        false
    }

    fn advance(&mut self) -> Result<&Token<'a>, ParserError> {
        let token = self.tokens.next().ok_or(ParserError::UnexpectedEOF)?;
        self.previous = Some(token);
        Ok(token)
    }

    pub fn parse(&mut self) -> Result<Vec<Statement<'a>>, ParserError> {
        let mut statements = vec![];
        while !self.eof() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Statement<'a>, ParserError> {
     if self.matches(&)
    }

    pub(crate) fn print(&self) {}
}
