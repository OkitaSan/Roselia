use super::parser::*;
#[derive(Debug,PartialEq, Eq)]
pub enum MiniDecafIR{
    UnaryMinus,
    LogicalNot,
    BitwiseNot,
    Push(i32),
    Ret,
}
impl MiniDecafIR{
    pub fn to_riscv32(&self) -> String{
        match self{
            &Self::Push(val) => {
                format!("\taddi sp,sp,-4;\n\tli t1,{};\n\tsw t1,0(sp);\n",val)
            }
            &Self::Ret => {
                format!("\tlw a0,0(sp);\n\taddi sp,sp, 4;\n\tjr ra;")
            }
            &Self::LogicalNot => {
                format!("\tlw t1,0(sp);\n\tseqz t1,t1;\n\tsw t1, 0(sp);\n")
            }
            &Self::BitwiseNot => {
                format!("\tlw t1,0(sp);\n\tnot t1,t1;\n\tsw t1,0(sp);\n")
            }
            &Self::UnaryMinus => {
                format!("\tlw t1,0(sp);\n\tneg t1,t1\n\tsw t1,0(sp);\n")
            }
        }
    }
}
pub struct MiniDecafIRGenerator{
    ir:Vec<MiniDecafIR>
}
impl MiniDecafIRGenerator{
    pub fn new() -> MiniDecafIRGenerator{
        MiniDecafIRGenerator{
            ir:vec![]
        }
    }
    pub fn to_ir(mut self,prog:&Program) -> Vec<MiniDecafIR>{
        self.parse_program(prog);
        self.ir
    }
    fn parse_program(&mut self,prog:&Program){
        self.parse_function(&prog.func)
    }   
    fn parse_function(&mut self,func:&Function){
        self.parse_statement(&func.stmt)
    }
    fn parse_statement(&mut self,stmt:&Statement){
        self.parse_expression(&stmt.expr);
        self.ir.push(MiniDecafIR::Ret);
    }
    fn parse_expression(&mut self,expr:&Expression){
        self.parse_unary(&expr.unary)
    }
    fn parse_unary(&mut self,expr:&Unary){
        match expr{
            &Unary::Integer(val) => {
                self.ir.push(MiniDecafIR::Push(val));
            }
            &Unary::SubUnary{
                ref operator,
                ref sub_unary
            } => {
                match operator{
                    &crate::lexer::MiniDecafTokens::BitwiseNot => {
                        self.parse_unary(&sub_unary);
                        self.ir.push(MiniDecafIR::BitwiseNot)
                    },
                    &crate::lexer::MiniDecafTokens::LogicalNot => {
                        self.parse_unary(&sub_unary);
                        self.ir.push(MiniDecafIR::LogicalNot)
                    },
                    &crate::lexer::MiniDecafTokens::Minus => {
                        self.parse_unary(&sub_unary);
                        self.ir.push(MiniDecafIR::UnaryMinus)
                    },
                    _ => unimplemented!()
                };
            }
        }
    }
}