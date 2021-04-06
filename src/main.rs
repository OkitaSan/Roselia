use roselia::*;
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let left_bracket_forgetted = "int main()return 0;}".to_owned();
    let mut lexer = Scanner::new(left_bracket_forgetted);
    let result = lexer.to_tokens()?;
    let mut parser = MiniDecafParser::new(result);
    println!("{:?}",parser.parse_program());
    println!("Hello, world!");
    Ok(())
}
