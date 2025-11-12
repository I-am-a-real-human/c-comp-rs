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

#[derive(Debug)]
struct Lexer {
    source: String,
    tokens: Vec<TokenType>,
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
    fn scan_token(&self) {}

    fn tokenise(&mut self) {
        while !self.eof() {
            self.start = self.curr_pos;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "",
            literal: "",
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
    let lexer = Lexer { source: program };
}
