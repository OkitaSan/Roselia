use roselia::lexer::*;
use roselia::parser::*;
use roselia::visit::*;
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let left_bracket_forgetted = "int main(){return 0;}".to_owned();
    let mut lexer = Scanner::new(left_bracket_forgetted);
    let result = lexer.to_tokens()?;
    let mut parser = MiniDecafParser::new(result);
    let ast = parser.parse_program()?;
    println!("{:?}",ast);
    println!("The result of the ast is {}",program_visitor(&ast));
    println!("Hello, world!");
    Ok(())
}
