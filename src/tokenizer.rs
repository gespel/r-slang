use std::process::exit;

#[derive(Debug)]
pub enum Token {
    IDENTIFIER(String), NUMBER(f32), FUNCTION, RETURN, SEMICOLON, LEFTPARANTHESIS, RIGHTPARANTHESIS, UNKOWN
}

pub struct Tokenizer {

}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        println!("Creating a new Tokenizer");
        Tokenizer {

        }
    }

    pub fn check_for_keyword(&mut self, input: &Token) -> Token {
        let out: Token;
        match input {
            Token::IDENTIFIER(value) => {
                match value.clone().as_str() {
                    "fn" => {
                        out = Token::FUNCTION;
                    }
                    "return" => {
                        out = Token::RETURN;
                    }
                    _ => {
                        out = Token::IDENTIFIER(value.clone());
                    }
                }
            }
            _ => {
                println!("ERROR expected IDENTIFIER");
                exit(-1);
            }
        }
        out
    }

    pub fn tokenize(&mut self, input: &str) -> Vec<Token> {
        let mut string_parts: Vec<String> = Vec::new();
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
                        _   => println!("BLA")
                    } 
                }
            } 
            if !substr.is_empty() {
                out.push(Token::IDENTIFIER(substr.clone()));
            }
           
           
        }

        for i in 0..out.len() {
            match &out[i] {
                Token::IDENTIFIER(value) => {
                    out[i] = self.check_for_keyword(&out[i]);
                }
                _ => {

                }
            }
        } 

        return out;
    }
}
