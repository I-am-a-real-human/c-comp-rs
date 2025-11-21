use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
enum TokenType {
    IDENTIFIER,
    CONSTANT,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    STAR,
    SEMICOLON,
    RETURN,
    IF,
    ELSE,
    INT,
    FLOAT,
    CHAR,
    STRUCT,
    VOID,
    BANG,
    BANG_EQUAL,
    EQUAL_EQUAL,
    GREATER_EQUAL,
    GREATER,
    LESS_EQUAL,
    LESS,
    EQUAL,
    SLASH,
    EOF,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.token_type)
    }
}

#[derive(Debug)]
struct Lexer<'a> {
    source: &'a str,
    chars: std::str::Chars<'a>,
    tokens: Vec<Token>,
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
                ("return", TokenType::RETURN),
                ("if", TokenType::IF),
                ("else", TokenType::ELSE),
                ("struct", TokenType::STRUCT),
                ("void", TokenType::VOID),
                ("int", TokenType::INT),
                ("float", TokenType::FLOAT),
                ("char", TokenType::CHAR),
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

    fn add_token(&mut self, token_type: TokenType, literal: String) {
        let text = &self.source[self.start_byte..self.curr_byte];
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text.to_string(),
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
            self.add_token(match_token, "".to_string())
        } else {
            self.add_token(original_token, "".to_string())
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
            self.add_token(TokenType::SLASH, "".to_string());
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
        let constant: String = self.source[self.start_byte + 1..self.curr_byte - 1].to_string();
        self.add_token(TokenType::CONSTANT, constant);
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
            self.add_token(token_type, text.to_string());
        } else {
            self.add_token(TokenType::IDENTIFIER, text.to_string());
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
            TokenType::CONSTANT,
            self.source[self.start_byte..self.curr_byte].to_string(),
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
            Some('(') => self.add_token(TokenType::LPAREN, "".to_string()),
            Some(')') => self.add_token(TokenType::RPAREN, "".to_string()),
            Some('{') => self.add_token(TokenType::LBRACE, "".to_string()),
            Some('}') => self.add_token(TokenType::RBRACE, "".to_string()),
            Some(',') => self.add_token(TokenType::COMMA, "".to_string()),
            Some(';') => self.add_token(TokenType::SEMICOLON, "".to_string()),
            Some('*') => self.add_token(TokenType::STAR, "".to_string()),
            // conditional tokens
            Some('!') => self.conditional_token('=', TokenType::BANG_EQUAL, TokenType::BANG),
            Some('=') => self.conditional_token('=', TokenType::EQUAL_EQUAL, TokenType::EQUAL),
            Some('>') => self.conditional_token('=', TokenType::GREATER_EQUAL, TokenType::GREATER),
            Some('<') => self.conditional_token('=', TokenType::LESS_EQUAL, TokenType::LESS),
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
            lexeme: "".to_string(),
            literal: "".to_string(),
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
