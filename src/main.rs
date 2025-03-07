use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
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

    fn number(&mut self) -> i32 {
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

fn main() {}
