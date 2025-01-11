#[derive(Debug)]
pub enum Token {
    IDENTIFIER, NUMBER, FUNCTION, SEMICOLON
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
        let out: Vec<Token> = Vec::new();

        for p in parts {
            //out.push(p.to_string());
            if p == "lina" {
                println!("Ich liebe dich!");
            }
        }

        return out;
    }
}
