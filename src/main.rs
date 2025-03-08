use std::iter::Peekable;
use std::str::Chars;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Token {
    NUMBER(i32),
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    LEFT_PAREN,
    RIGHT_PAREN,
}

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        while let Some(&ch) = self.chars.peek() {
            match ch {
                ' ' => {
                    self.chars.next();
                }
                '0'..='9' => return Some(self.number()),
                '+' => {
                    self.chars.next();
                    return Some(Token::PLUS);
                }
                '-' => {
                    self.chars.next();
                    return Some(Token::MINUS);
                }
                '*' => {
                    self.chars.next();
                    return Some(Token::MULTIPLY);
                }
                '/' => {
                    self.chars.next();
                    return Some(Token::DIVIDE);
                }
                '(' => {
                    self.chars.next();
                    return Some(Token::LEFT_PAREN);
                }
                ')' => {
                    self.chars.next();
                    return Some(Token::RIGHT_PAREN);
                }
                _ => {
                    eprintln!("Unrecognized character '{}'", ch);
                    std::process::exit(65);
                }
            }
        }
        None
    }

    fn number(&mut self) -> Token {
        let mut num = String::new();
        while let Some(&ch) = self.chars.peek() {
            if ch.is_numeric() {
                num.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }
        Token::NUMBER(num.parse::<i32>().unwrap())
    }
}

#[derive(Debug)]
enum AST {
    Num(i32),
    BinOp(Box<AST>, Token, Box<AST>),
}

struct Parser<'a> {
    tokens: Peekable<Box<dyn Iterator<Item = Token> + 'a>>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Box<dyn Iterator<Item = Token> + 'a>) -> Self {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    fn parse(&mut self) -> AST {
        self.expr()
    }

    fn expr(&mut self) -> AST {
        let mut node = self.term();

        while let Some(&token) = self.tokens.peek() {
            if token == Token::PLUS || token == Token::MINUS {
                let op = self.tokens.next().unwrap();
                let right = self.term();
                node = AST::BinOp(Box::new(node), op, Box::new(right));
            } else {
                break;
            }
        }
        node
    }

    fn term(&mut self) -> AST {
        let mut node = self.factor();

        while let Some(&token) = self.tokens.peek() {
            if token == Token::MULTIPLY || token == Token::DIVIDE {
                let op = self.tokens.next().unwrap();
                let right = self.factor();
                node = AST::BinOp(Box::new(node), op, Box::new(right));
            } else {
                break;
            }
        }
        node
    }

    fn factor(&mut self) -> AST {
        match self.tokens.next() {
            Some(Token::NUMBER(n)) => AST::Num(n),
            Some(Token::LEFT_PAREN) => {
                let node = self.expr();
                if self.tokens.next() != Some(Token::RIGHT_PAREN) {
                    eprintln!("Expected closing parenthesis");
                    std::process::exit(65);
                }
                node
            }
            _ => {
                eprintln!("Unexpected token");
                std::process::exit(65);
            }
        }
    }
}

fn evaluate(ast: &AST) -> i32 {
    match ast {
        AST::Num(n) => *n,
        AST::BinOp(left, Token::PLUS, right) => evaluate(left) + evaluate(right),
        AST::BinOp(left, Token::MINUS, right) => evaluate(left) - evaluate(right),
        AST::BinOp(left, Token::MULTIPLY, right) => evaluate(left) * evaluate(right),
        AST::BinOp(left, Token::DIVIDE, right) => evaluate(left) / evaluate(right),
        _ => panic!("Unknown operation"),
    }
}
fn main() {
    let input = "3 + 5 * (10 - 2) / 2 * 2 * 10";

    let mut lexer = Lexer::new(input);
    let tokens: Vec<Token> = std::iter::from_fn(|| lexer.next_token()).collect();

    let mut parser = Parser::new(Box::new(tokens.into_iter()));
    let ast = parser.parse();

    let result = evaluate(&ast);
    println!("AST: {:#?}", ast);
    println!("Result: {}", result);
}
