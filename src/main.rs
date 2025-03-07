
use std::io::{self, Write};
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
    chars : Peekable<Chars<'a>>
}


impl<'a> Lexer<'a> {
    fn new(input : &'a str) -> Self {
        Lexer {
            chars : input.chars().peekable()
        }
    }

}


fn main() {}