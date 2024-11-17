#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(i64),
    Operator(char),
    Punctuation(char),
    Whitespace,
    Unknown(char),
}

impl Token {
    fn is_keyword(&self) -> bool {
        matches!(self, Token::Keyword(_))
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    // Constructor for creating a new lexer instance
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    // Peek at the next character without advancing the position
    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    // Advance the position by one character
    fn advance(&mut self) -> Option<char> {
        let result = self.peek();
        if let Some(c) = result {
            self.position += c.len_utf8();
        }
        result
    }

    // Tokenize the entire input string
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.peek() {
            match c {
                ' ' | '\n' | '\t' | '\r' => {
                    // Skip whitespace and treat as a token (optional, depending on use case)
                    self.advance();
                    tokens.push(Token::Whitespace);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    // Identifier or Keyword
                    let ident = self.read_identifier();
                    if ident == "let" || ident == "fn" {
                        tokens.push(Token::Keyword(ident));
                    } else {
                        tokens.push(Token::Identifier(ident));
                    }
                }
                '0'..='9' => {
                    // Number literal
                    let num = self.read_number();
                    tokens.push(Token::Number(num));
                }
                '+' | '-' | '*' | '/' | '=' => {
                    // Operators
                    tokens.push(Token::Operator(c));
                    self.advance();
                }
                '(' | ')' | '{' | '}' | ',' | ';' => {
                    // Punctuation
                    tokens.push(Token::Punctuation(c));
                    self.advance();
                }
                _ => {
                    // Unknown characters
                    tokens.push(Token::Unknown(c));
                    self.advance();
                }
            }
        }
        tokens
    }

    // Read an identifier (variable or function name)
    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    // Read a number literal
    fn read_number(&mut self) -> i64 {
        let start = self.position;
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].parse().unwrap()
    }
}

// The public function to tokenize input
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize()
}
