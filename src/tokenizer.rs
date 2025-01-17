use std::process::exit;
use std::str::FromStr;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Token {
    IDENTIFIER(String), NUMBER(f64), FUNCTION, RETURN, SEMICOLON, LEFTPARANTHESIS, RIGHTPARANTHESIS, LEFTBRACKETS, RIGHTBRACKETS
}

pub struct Tokenizer {

}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        println!("Creating a new Tokenizer");
        Tokenizer {

        }
    }

    pub fn check_for_keyword(&mut self, input: &str) -> Token {
        let out: Token;
        match input {
            "fn" => {
                out = Token::FUNCTION;
            }
            "return" => {
                out = Token::RETURN;
            }
            _ => {
                out = Token::IDENTIFIER(input.to_string());
            }
        }
        out
    }

    pub fn check_for_number(&mut self, input: &Token) -> Token {
        let out: Token;
        match input {
            Token::IDENTIFIER(value) => {
                let mut sub: String = "".to_string();
                for c in value.chars() {
                    if c.is_ascii_digit() {
                        sub.push(c);
                    }
                    else {
                        println!("ERROR letter in number detected!");
                        exit(-1);
                    }
                }
                out = Token::NUMBER(f64::from_str(&sub).unwrap());
            }
            _ => {
                println!("ERROR expected IDENTIFIER");
                exit(-1);
            }
        }
        out
    }

    pub fn tokenize(&mut self, input: &str) -> Vec<Token> {
        let parts = input.split(" ");
        let mut out: Vec<Token> = Vec::new();
        for p in parts {
            let mut substr: String = "".to_string();
            for c in p.chars() {
                if c.is_alphanumeric() {
                    substr = substr + format!("{}", c).as_str();
                }
                else {
                    if !substr.is_empty() {
                        out.push(Token::IDENTIFIER(substr.clone()));
                        substr = "".to_string();
                    }
                    match c {
                        '(' => out.push(Token::LEFTPARANTHESIS),
                        ')' => out.push(Token::RIGHTPARANTHESIS),
                        ';' => out.push(Token::SEMICOLON),
                        '{' => out.push(Token::LEFTBRACKETS),
                        '}' => out.push(Token::RIGHTBRACKETS),
                        _   => println!("BLA")
                    }
                }
            }
            if !substr.is_empty() {
                out.push(Token::IDENTIFIER(substr.clone()));
            }

        }

        for i in 0..out.len() {
            match &mut out[i] {
                Token::IDENTIFIER(value) => {
                    out[i] = self.check_for_keyword(value);
                }
                _ => {}
            }
        }

        for i in 0..out.len() {
            match &mut out[i] {
                Token::IDENTIFIER(value) => {
                    if value.as_bytes()[0].is_ascii_digit() {
                        out[i] = self.check_for_number(&out[i]);
                    }
                }
                _ => {}
            }
        }
        out
    }
}
