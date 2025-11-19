use std::fmt;

#[derive(Debug)]
enum TokenType {
    KEYWORD(String),
    CONSTANT(String), // TODO figure out whether we want to store the string here or as part of the Lexer
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    STAR,
    SEMICOLON,
    RETURN,
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
        write!(f, "{}, {}", self.lexeme, self.literal)
    }
}

#[derive(Debug)]
struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    curr_pos: usize,
    line: usize,
}

impl Default for Lexer {
    fn default() -> Self {
        Lexer {
            source: "".to_string(),
            tokens: vec![],
            start: 0,
            curr_pos: 0,
            line: 1,
        }
    }
}

impl Lexer {
    fn add_token(&mut self, token_type: TokenType, literal: String) {
        let text = &self.source[self.start..self.curr_pos];
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text.to_string(),
            literal: literal,
            line: self.line,
        });
    }

    fn at(&self, pos: usize) -> char {
        return self.source.as_bytes()[pos] as char;
    }

    fn advance(&mut self) -> char {
        self.curr_pos += 1;
        return self.at(self.curr_pos);
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LPAREN, "".to_string()),
            ')' => self.add_token(TokenType::RPAREN, "".to_string()),
            '{' => self.add_token(TokenType::LBRACE, "".to_string()),
            '}' => self.add_token(TokenType::RBRACE, "".to_string()),
            ',' => self.add_token(TokenType::COMMA, "".to_string()),
            ';' => self.add_token(TokenType::SEMICOLON, "".to_string()),
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenType::BANG_EQUALS, "".to_string())
                } else {
                    self.add_token(TokenType::BANG, "".to_string())
                }
            } // TODO add matches check here for BANG_EQUALS
            _ => panic!("Unexpected character {}", c),
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        if !self.eof() {
            if self.at(self.curr_pos) == expected {
                self.curr_pos += 1;
                return true;
            }
        }
        return false;
    }

    fn tokenise(&mut self) {
        // scan file
        while !self.eof() {
            self.start = self.curr_pos;
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
        self.curr_pos >= self.source.len()
    }
}

fn main() {
    let program = "int main(void) {
        return 0;
    }"
    .to_string();
    let mut lexer = Lexer {
        source: program,
        ..Default::default()
    };
    lexer.tokenise();

    for token in lexer.tokens.iter() {
        print!("{}", token);
    }
}
