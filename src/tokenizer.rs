#[derive(Debug)]
pub enum Token {
    PRETOKEN(String), IDENTIFIER, NUMBER, FUNCTION, SEMICOLON
}

pub struct Tokenizer {

}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        println!("Creating a new Tokenizer");
        Tokenizer {

        }
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
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
                       out.push(Token::PRETOKEN(substr.clone()));
                       substr = "".to_string();
                    }
                    out.push(Token::PRETOKEN(c.to_string()))
                }
                println!("{}", c);
            } 
            if !substr.is_empty() {
                out.push(Token::PRETOKEN(substr.clone()));
            }
           
           
        }

        return out;
    }
}
