use core::fmt;
use std::collections::HashMap;

use super::lexer::*;
/// Parser for the MiniDecaf
/// BNF:
///     program    := function
///     function   := type Identifier Lparen Rparen Lbrace statement Rbrace
///     type       := Int
///     statement  := Return expression Semicolon
///     expression := unary
///     unary      := Integer | ('-'|'!'|'~') unary
#[derive(Debug)]
pub struct Program {
    pub func: Function,
}
#[derive(Debug)]
pub struct Function {
    pub ty: MiniDecafTokens,
    pub identifier: String,
    pub stmt: Statement,
}
#[derive(Debug)]
pub struct Statement {
    pub expr: Expression,
}
#[derive(Debug)]
pub struct Expression {
    pub unary:Unary
}
#[derive(Debug)]
pub enum Unary{
    Integer(i32),
    SubUnary{
        operator:MiniDecafTokens,
        sub_unary:Box<Unary>
    }
}
macro_rules! hashmap {
    ($($x:expr=>$y:expr),*) => {
        {
        let mut res = ::std::collections::HashMap::new();
        $(
            res.insert($x,$y);
        )*
        res
        }
    };
}
type ParseResult<T> = Result<T, ParserError>;
#[derive(Debug)]
pub enum ParserError {
    UnexpectedTokenError,
    SemicolonGoneError,
    UndefinedTypeError,
    MissingIdentifierError,
    ExpectUnaryExpressionError,
    InvaildIdentifierError,
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::UndefinedTypeError => {
                write!(f, "A unexpected expression occured!")
            }
            &Self::SemicolonGoneError => {
                write!(f, "Where is the end semicon?")
            }
            &Self::UnexpectedTokenError => {
                write!(f, "Wrong token!")
            }
            &Self::MissingIdentifierError => {
                write!(f, "Where is the identifier?")
            }
            &Self::ExpectUnaryExpressionError =>{
                write!(f,"lossing operand")
            }
            &Self::InvaildIdentifierError => {
                write!(f,"Invaild identifier error")
            }
        }
    }
}
impl std::error::Error for ParserError {}
pub struct MiniDecafParser {
    cursor: usize,
    tokens: Vec<MiniDecafTokens>,
    vaild_identifier : HashMap<String,()>
}

impl MiniDecafParser {
    pub fn new(tokens: Vec<MiniDecafTokens>) -> MiniDecafParser {
        MiniDecafParser { cursor: 0, tokens,vaild_identifier:hashmap!("main".to_owned() => ()) }
    }
    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let func = self.parse_function()?;
        Ok(Program { func })
    }
    fn parse_function(&mut self) -> ParseResult<Function> {
        let t = self.parse_type()?;
        let name;
        if let MiniDecafTokens::Identifier(id) = &self.tokens[self.cursor] {
            name = id.clone();
            if !self.vaild_identifier.contains_key(id){
                return Err(ParserError::InvaildIdentifierError);
            }
            self.cursor += 1;
        } else {
            return Err(ParserError::MissingIdentifierError);
        }
        if self.tokens[self.cursor] == MiniDecafTokens::LeftParenthesis {
            self.cursor += 1;
        } else {
            return Err(ParserError::UnexpectedTokenError);
        }
        if self.tokens[self.cursor] == MiniDecafTokens::RightParenthesis {
            self.cursor += 1;
        } else {
            return Err(ParserError::UnexpectedTokenError);
        }
        if self.tokens[self.cursor] == MiniDecafTokens::LeftBracket {
            self.cursor += 1;
        } else {
            return Err(ParserError::UnexpectedTokenError);
        }
        let stmt = self.parse_statement()?;
        if self.tokens[self.cursor] == MiniDecafTokens::RightBracket {
            self.cursor += 1;
        } else {
            return Err(ParserError::UnexpectedTokenError);
        }
        Ok(Function {
            ty: t,
            identifier: name,
            stmt,
        })
    }
    fn parse_type(&mut self) -> ParseResult<MiniDecafTokens> {
        if self.tokens[self.cursor] == MiniDecafTokens::Int {
            self.cursor += 1;
            return Ok(MiniDecafTokens::Int);
        } else {
            return Err(ParserError::UndefinedTypeError);
        }
    }
    fn parse_statement(&mut self) -> ParseResult<Statement> {
        // 顶层已经帮我们检查了return了。
        self.cursor += 1;
        let expr = self.parse_expression()?;
        if self.tokens[self.cursor] == MiniDecafTokens::Semicon {
            self.cursor += 1;
        } else {
            return Err(ParserError::SemicolonGoneError);
        }
        return Ok(Statement { expr });
    }
    fn parse_expression(&mut self) -> ParseResult<Expression> {
            let result = self.parse_unary()?;
            return Ok(Expression{unary:result})
    }
    fn parse_unary(&mut self) -> ParseResult<Unary>{
        if let MiniDecafTokens::IntValue(val) = self.tokens[self.cursor]{
            self.cursor += 1;
            return Ok(Unary::Integer(val));
        }else if let MiniDecafTokens::BitwiseNot = self.tokens[self.cursor] {
            let operator = MiniDecafTokens::BitwiseNot;
            self.cursor += 1;
            let sub_unary = self.parse_unary()?;
            return Ok(Unary::SubUnary{operator,sub_unary:Box::new(sub_unary)})
        }else if let MiniDecafTokens::LogicalNot = self.tokens[self.cursor] {
            let operator = MiniDecafTokens::LogicalNot;
            self.cursor += 1;
            let sub_unary = self.parse_unary()?;
            return Ok(Unary::SubUnary{operator,sub_unary:Box::new(sub_unary)})
        }else if let MiniDecafTokens::Minus = self.tokens[self.cursor] {
            let operator = MiniDecafTokens::Minus;
            self.cursor += 1;
            let sub_unary = self.parse_unary()?;
            return Ok(Unary::SubUnary{operator,sub_unary:Box::new(sub_unary)})
        }else{
            return Err(ParserError::UnexpectedTokenError);
        }
    }
}
