use super::parser::*;
pub fn program_visitor(prog: &Program) -> i32 {
    function_visitor(&prog.func)
}
pub fn function_visitor(func: &Function) -> i32 {
    statement_visitor(&func.stmt)
}
pub fn statement_visitor(stmt: &Statement) -> i32 {
    expression_visitor(&stmt.expr)
}
#[inline]
pub fn expression_visitor(expr: &Expression) -> i32 {
    let Expression { content } = *expr;
    content
}