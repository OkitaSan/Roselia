use roselia::lexer::*;
use roselia::parser::*;
use roselia::ir::*;
use std::fs;
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let program = "int main(){return 0;}".to_owned();
    let mut lexer = Scanner::new(program);
    let tokens = lexer.to_tokens()?;
    println!("{:?}",tokens);
    let mut parser = MiniDecafParser::new(tokens);
    let prog = parser.parse_program()?;
    let ir_generator = MiniDecafIRGenerator::new();
    let ir:Vec<_> = ir_generator.to_ir(&prog).into_iter().map(|x| x.to_riscv32()).collect();
    let elf_head = r#"    .text
    .globl main
main:
"#.to_owned();
    let rv32 = format!("{}{}",elf_head,ir.join(""));
    fs::write("main.s", &rv32)?;
    Ok(())
}
