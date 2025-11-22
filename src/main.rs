use std::{collections::HashMap, fmt, fs};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug)]
enum LexerError {
    UnterminatedString { line: usize, col: usize },
    UnexpectedChar { line: usize, col: usize, char: char },
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
    col: usize,
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
            col: 1,
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

    fn add_token(&mut self, token_type: TokenType, literal: &'a str) -> Result<(), LexerError> {
        let text = &self.source[self.start_byte..self.curr_byte];
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line: self.line,
        });
        Ok(())
    }

    fn at(&self, pos: usize) -> Option<char> {
        self.source.get(pos..)?.chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.curr_byte += c.len_utf8();
        self.col += 1;
        Some(c)
    }

    fn conditional_token(
        &mut self,
        token: char,
        match_token: TokenType,
        original_token: TokenType,
    ) -> TokenType {
        if self.matches(token) {
            match_token
        } else {
            original_token
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

    fn parse_slash(&mut self) -> Result<(), LexerError> {
        // single line comment
        if self.matches('/') {
            // consume until end of line or EOF
            while self.peek() != Some('\n') && !self.eof() {
                self.advance();
            }
        } else {
            self.add_token(TokenType::Slash, "");
        }

        Ok(())
    }

    fn consume_string(&mut self) -> Result<(), LexerError> {
        while self.peek() != Some('"') && !self.eof() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.eof() {
            return Err(LexerError::UnterminatedString {
                line: self.line,
                col: self.col,
            });
        }

        // need to call this to consume the closing '"'
        self.advance();

        // FIXME we need to capture the value of the quotation mark instead
        // of +1/-1 below. Either that or at least double-check the value
        let constant = &self.source[self.start_byte + 1..self.curr_byte - 1];
        self.add_token(TokenType::Constant, constant);
        Ok(())
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
            // keep going until number has been consumed
            while self.is_digit(self.peek()) {
                self.advance();
            }
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

    fn scan_token(&mut self) -> Result<(), LexerError> {
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
            Some('!') => {
                let token_type = self.conditional_token('=', TokenType::BangEqual, TokenType::Bang);
                self.add_token(token_type, "")
            }
            Some('=') => {
                let token_type =
                    self.conditional_token('=', TokenType::EqualEqual, TokenType::Equal);
                self.add_token(token_type, "")
            }
            Some('>') => {
                let token_type =
                    self.conditional_token('=', TokenType::GreaterEqual, TokenType::Greater);
                self.add_token(token_type, "")
            }
            Some('<') => {
                let token_type = self.conditional_token('=', TokenType::LessEqual, TokenType::Less);
                self.add_token(token_type, "")
            }
            Some('/') => self.parse_slash(),
            Some('\n') => {
                self.line += 1;
                self.col = 0;
                Ok(())
            }
            Some('"') => self.consume_string(),
            Some('\'') => Ok(self.consume_char()),
            Some(' ') | Some('\r') | Some('\t') => Ok(()),
            _ => {
                if self.is_digit(c) {
                    self.consume_number();
                    Ok(())
                } else if self.is_alpha(c) {
                    self.consume_identifier();
                    Ok(())
                } else {
                    Err(LexerError::UnexpectedChar {
                        line: self.line,
                        col: self.col,
                        char: c.expect("Error reporting UnexpectedChar"),
                    })
                }
            }
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        if !self.eof() {
            if self.at(self.curr_byte) == Some(expected) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn tokenise(&mut self) -> Result<&Vec<Token<'a>>, Vec<LexerError>> {
        let mut errors = Vec::new();

        // scan file
        while !self.eof() {
            self.start_byte = self.curr_byte;

            if let Err(e) = self.scan_token() {
                errors.push(e);
            }
        }

        // add EOF token before finishing
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "",
            literal: "",
            line: self.line,
        });

        if errors.is_empty() {
            return Ok(&self.tokens);
        }
        Err(errors)
        // }
    }

    fn eof(&self) -> bool {
        self.curr_byte >= self.source.len()
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn lex_catch_unterminated_string() {
        let source = "int main(void) {\nchar* str = \"string here\nreturn 0=0;\n}";
        let mut lexer = Lexer::from_string(&source);
        let result = lexer.tokenise();
        match result {
            Ok(_) => {
                assert!(false);
            }
            Err(mut errors) => {
                assert_eq!(errors.len(), 1);
                let error = errors.pop().unwrap();
                assert!(matches!(error, LexerError::UnterminatedString { .. }));
            }
        }
    }
    #[test]
    fn lex_tokenizes_simple_return_statement() {
        let source = "return 42;";
        let mut lexer = Lexer::from_string(source);
        let tokens = lexer.tokenise().expect("Should tokenise without errors");

        let token_types: Vec<_> = tokens.iter().map(|t| t.token_type.clone()).collect();

        assert!(token_types.contains(&TokenType::Return));
        assert!(token_types.contains(&TokenType::Constant));
        assert!(token_types.contains(&TokenType::Semicolon));
    }

    #[test]
    fn lex_identifiers_and_keywords() {
        let source = "int var = 100;";
        let mut lexer = Lexer::from_string(source);
        let tokens = lexer.tokenise().expect("Should tokenise without errors");

        let mut found_int = false;
        let mut found_identifier = false;
        for t in tokens {
            match t.token_type {
                TokenType::Int => found_int = true,
                TokenType::Identifier => found_identifier = true,
                _ => {}
            }
        }
        assert!(found_int);
        assert!(found_identifier);
    }

    #[test]
    fn lex_catches_unexpected_character() {
        let source = "int @";
        let mut lexer = Lexer::from_string(source);
        let result = lexer.tokenise();
        match result {
            Ok(_) => panic!("Expected error on unexpected character"),
            Err(mut errors) => {
                assert_eq!(errors.len(), 1);
                let err = errors.pop().unwrap();
                assert!(matches!(err, LexerError::UnexpectedChar { .. }));
            }
        }
    }

    #[test]
    fn handles_empty_input() {
        let source = "";
        let mut lexer = Lexer::from_string(source);
        let tokens = lexer.tokenise().expect("Should handle empty input");
        // Expect only EOF token
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::EOF);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("fcc help TODO");
    }

    let source = fs::read_to_string(&args[1]).expect("Couldn't read file '{}'");
    let mut lexer = Lexer::from_string(&source);

    let result = lexer.tokenise();

    match result {
        Ok(tokens) => {
            for token in tokens.iter() {
                println!("{} | ", token);
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("Syntax error: {:?}", error);
            }
            std::process::exit(1);
        }
    }

    for token in lexer.tokens.iter() {
        print!("{} | ", token);
    }
    println!("");
}
