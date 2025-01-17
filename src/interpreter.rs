use std::io::BufRead;
use crate::tokenizer::Token;
use crate::tokenizer::Token::IDENTIFIER;

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
            if let Token::LEFTPARANTHESIS = self.consume() {
                if let Token::RIGHTPARANTHESIS = self.consume() {
                    if let Token::LEFTBRACKETS = self.consume() {
                        println!("Defining function with name {}", name.clone());
                        let mut function_tokens: Vec<Token> = vec![];
                        let mut bracket_count: i32 = -1_i32;
                        while bracket_count != 0 {
                            if let Token::LEFTBRACKETS = self.peek() {
                                bracket_count -= 1;
                            } else if let Token::RIGHTBRACKETS = self.peek() {
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
                Token::IDENTIFIER(_) => {}
                Token::NUMBER(_) => {}
                Token::FUNCTION => {
                    self.parse_function();

                }
                Token::RETURN => {}
                Token::SEMICOLON => {}
                Token::LEFTPARANTHESIS => {}
                Token::RIGHTPARANTHESIS => {}
                Token::LEFTBRACKETS => {}
                Token::RIGHTBRACKETS => {}
            }
        }
    }
}
