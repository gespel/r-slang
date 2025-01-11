mod tokenizer;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("Test running now!");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_test() {
        let t = tokenizer::Tokenizer::new();
        println!("{:?}", t.tokenize("asd sten lina"));
        println!("Testtest called!");
    }
}
