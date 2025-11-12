#[derive(Debug)]
enum TokenType {
    KEYWORD(String),
    CONSTANT(String),
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMICOLON,
    RETURN,
    EOF,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    fn from_token_type(&self, token_type : TokenType) -> Token {
        Token {
            token_type : token_type,
            lexeme: "".to_string(),
            literal: "".to_string(),
        }
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

    fn add_token(&mut self, token_type : TokenType) {
    }

    fn add_

    fn advance(&mut self) -> char {
        self.curr_pos += 1;
        return self.source.as_bytes()[self.curr_pos] as char;
    }

    fn scan_token(&self) {
        let c = self.advance();
        match c {
            "(" => add_token
        }
    }

    fn tokenise(&mut self) {
        while !self.eof() {
            self.start = self.curr_pos;
            self.scan_token();
        }

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
    let lexer = Lexer {
        source: program,
        ..Default::default()
    };
}
