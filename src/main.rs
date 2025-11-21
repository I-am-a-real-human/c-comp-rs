use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
enum TokenType {
    Identifier,
    Constant,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Star,
    Semicolon,
    Return,
    If,
    Else,
    Int,
    Float,
    Char,
    Struct,
    Void,
    Bang,
    BangEqual,
    EqualEqual,
    GreaterEqual,
    Greater,
    LessEqual,
    Less,
    Equal,
    Slash,
    EOF,
}

#[derive(Debug)]
struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    literal: &'a str,
    line: usize,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.token_type)
    }
}

#[derive(Debug)]
struct Lexer<'a> {
    source: &'a str,
    chars: std::str::Chars<'a>,
    tokens: Vec<Token<'a>>,
    start_byte: usize,
    curr_byte: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl<'a> Default for Lexer<'a> {
    fn default() -> Self {
        Lexer {
            source: "",
            chars: "".chars(),
            tokens: vec![],
            start_byte: 0,
            curr_byte: 0,
            line: 1,
            keywords: HashMap::from([
                ("return", TokenType::Return),
                ("if", TokenType::If),
                ("else", TokenType::Else),
                ("struct", TokenType::Struct),
                ("void", TokenType::Void),
                ("int", TokenType::Int),
                ("float", TokenType::Float),
                ("char", TokenType::Char),
            ]),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn from_string(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.chars(),
            ..Default::default()
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: &'a str) {
        let text = &self.source[self.start_byte..self.curr_byte];
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line: self.line,
        });
    }

    fn at(&self, pos: usize) -> Option<char> {
        self.source.get(pos..)?.chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.curr_byte += c.len_utf8();
        Some(c)
    }

    fn conditional_token(
        &mut self,
        token: char,
        match_token: TokenType,
        original_token: TokenType,
    ) {
        if self.matches(token) {
            self.add_token(match_token, "")
        } else {
            self.add_token(original_token, "")
        }
    }

    fn peek(&self) -> Option<char> {
        if self.eof() {
            return Some('\0');
        } else {
            return self.at(self.curr_byte);
        }
    }

    fn peek_after(&self) -> Option<char> {
        if self.eof() {
            return Some('\0');
        } else {
            return self.at(self.curr_byte + 1);
        }
    }

    fn parse_slash(&mut self) {
        // single line comment
        if self.matches('/') {
            // consume until end of line or EOF
            while self.peek() != Some('\n') && !self.eof() {
                self.advance();
            }
        } else {
            self.add_token(TokenType::Slash, "");
        }
    }

    fn consume_string(&mut self) {
        while self.peek() != Some('"') && !self.eof() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.eof() {
            panic!("Unterminated string at line {}", self.line);
        }

        // need to call this to consume the closing '"'
        self.advance();

        // FIXME we need to capture the value of the quotation mark instead
        // of +1/-1 below. Either that or at least double-check the value
        let constant = &self.source[self.start_byte + 1..self.curr_byte - 1];
        self.add_token(TokenType::Constant, constant);
    }

    fn consume_char(&mut self) {
        // peek(2) == '\''? if not then report unterminated/invalid char
        todo!();
    }

    fn consume_identifier(&mut self) {
        while matches!(self.peek(), c if self.is_alphanumeric(c)) {
            self.advance();
        }

        let text = &self.source[self.start_byte..self.curr_byte];

        if let Some(token_type) = self.keywords.get(text).cloned() {
            self.add_token(token_type, text);
        } else {
            self.add_token(TokenType::Identifier, text);
        }
    }

    fn consume_number(&mut self) {
        // similar to string implementation
        while self.is_digit(self.peek()) && !self.eof() {
            self.advance();
        }

        if self.peek() == Some('.') && self.is_digit(self.peek_after()) {
            self.advance();
        } // TODO - a syntax error could be caught here

        self.add_token(
            TokenType::Constant,
            &self.source[self.start_byte..self.curr_byte],
        );
    }

    fn is_alphanumeric(&self, c: Option<char>) -> bool {
        // TODO move this and other generic functions outside of impl scope
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_alpha(&self, c: Option<char>) -> bool {
        // TODO move this and other generic functions outside of impl scope
        // just check if we're within the bounds of each class of alphabetical char
        (c >= Some('a') && c <= Some('z')) || (c >= Some('A') && c <= Some('Z')) || c == Some('_')
    }

    fn is_digit(&self, c: Option<char>) -> bool {
        // TODO move this and other generic functions outside of impl scope
        // rely on ASCII value comparison
        if c >= Some('0') && c <= Some('9') {
            return true;
        }
        false
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // single value token
            Some('(') => self.add_token(TokenType::LParen, ""),
            Some(')') => self.add_token(TokenType::RParen, ""),
            Some('{') => self.add_token(TokenType::LBrace, ""),
            Some('}') => self.add_token(TokenType::RBrace, ""),
            Some(',') => self.add_token(TokenType::Comma, ""),
            Some(';') => self.add_token(TokenType::Semicolon, ""),
            Some('*') => self.add_token(TokenType::Star, ""),
            // conditional tokens
            Some('!') => self.conditional_token('=', TokenType::BangEqual, TokenType::Bang),
            Some('=') => self.conditional_token('=', TokenType::EqualEqual, TokenType::Equal),
            Some('>') => self.conditional_token('=', TokenType::GreaterEqual, TokenType::Greater),
            Some('<') => self.conditional_token('=', TokenType::LessEqual, TokenType::Less),
            Some('/') => self.parse_slash(),
            Some('\n') => self.line += 1,
            Some('"') => self.consume_string(),
            Some('\'') => self.consume_char(),
            Some(' ') | Some('\r') | Some('\t') => (),
            _ => {
                if self.is_digit(c) {
                    self.consume_number();
                } else if self.is_alpha(c) {
                    self.consume_identifier();
                } else {
                    panic!("Unexpected character {c:?}")
                }
            }
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        if !self.eof() {
            if self.at(self.curr_byte) == Some(expected) {
                self.curr_byte += 1;
                return true;
            }
        }
        return false;
    }

    fn tokenise(&mut self) {
        // scan file
        while !self.eof() {
            self.start_byte = self.curr_byte;
            self.scan_token();
        }

        // add EOF token before finishing
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "",
            literal: "",
            line: self.line,
        });
    }

    fn eof(&self) -> bool {
        self.curr_byte >= self.source.len()
    }
}

fn main() {
    let program = "int main(void) {
        return 0;
    }";

    let mut lexer = Lexer::from_string(program);

    lexer.tokenise();

    for token in lexer.tokens.iter() {
        print!("{} | ", token);
    }
    println!("");
}
