pub mod parser;

mod lexer;
mod syntax;

#[cfg(test)]
mod tests {
    #[test]
    fn is_ok() {
        assert_eq!(2+2, 4);
    }
}
