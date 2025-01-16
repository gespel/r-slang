mod tokenizer;

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_test() {
        let mut t = tokenizer::Tokenizer::new();
        println!("{:?}", t.tokenize("fn asd(){}asd sten return lina fn"));
        println!("Testtest called!");
    }
}
