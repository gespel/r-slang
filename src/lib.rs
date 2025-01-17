mod tokenizer;
mod interpreter;

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_test() {
        let mut t = tokenizer::Tokenizer::new();
        let a = t.tokenize("fn asd(){asd asd 32}");
        let mut i = interpreter::Interpreter::new(a.clone());
        i.interpret();
        println!("{:?}", a.clone());
        println!("Testtest called!");
    }
}
