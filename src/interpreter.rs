use std::io::BufRead;
use std::process::exit;
use crate::tokenizer::Token;
use crate::tokenizer::Token::{*};

pub struct Interpreter {
    index: usize,
    tokens: Vec<Token>
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>) -> Interpreter {
        Interpreter {
            tokens,
            index: 0,
        }
    }

    pub fn parse_function(&mut self) {
        if let IDENTIFIER(s) = self.consume() {
            let name = s.clone();
            if let LEFTPARANTHESIS = self.consume() {
                if let RIGHTPARANTHESIS = self.consume() {
                    if let LEFTBRACKETS = self.consume() {
                        println!("Defining function with name {}", name.clone());
                        let mut function_tokens: Vec<Token> = vec![];
                        let mut bracket_count: i32 = -1_i32;
                        while bracket_count != 0 {
                            if let LEFTBRACKETS = self.peek() {
                                bracket_count -= 1;
                            } else if let RIGHTBRACKETS = self.peek() {
                                bracket_count += 1;
                                if bracket_count == 0 {
                                    break;
                                }
                            }
                            function_tokens.push(self.consume().clone());
                        }
                        println!("{:?}", function_tokens);
                    }
                }
            }
        }
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.index]
    }

    pub fn increase_index(&mut self) {
        self.index += 1;
    }

    pub fn consume_expected(&mut self, expected: Token) -> Token {
        let out = self.peek().clone();
        if out == expected {
            self.increase_index();
            return out;
        }
        exit(-1);
    }

    pub fn consume(&mut self) -> Token {
        let out = self.peek().clone();
        self.index += 1;
        out
    }

    pub fn interpret(&mut self) {
        //println!("{:?}", self.peek().unwrap());
        while self.index < self.tokens.len() {
            println!("Current: {:?}", self.peek());
            match self.consume() {
                IDENTIFIER(_) => {}
                NUMBER(_) => {}
                FUNCTION => {
                    self.parse_function();

                }
                RETURN => {}
                SEMICOLON => {}
                LEFTPARANTHESIS => {}
                RIGHTPARANTHESIS => {}
                LEFTBRACKETS => {}
                RIGHTBRACKETS => {}
            }
        }
    }
}
