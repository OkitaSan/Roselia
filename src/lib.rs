use core::fmt;

use fmt::write;

/// Tokens of MiniDecaf
/// Int -> type int(only positive numbers for now)
///
/// Identifier -> variable/function name
///
/// Return -> return keyword
///
/// LeftBracket -> {
///
/// RightBracket -> }
///
///
/// LeftParenthesis -> (
///
/// RightParenthesis -> )
///
/// Semicon -> ;
#[derive(Debug,PartialEq, Eq)]
pub enum MiniDecafTokens {
    Int,
    IntValue(i32),
    Identifier(String),
    Return,
    LeftBracket,
    RightBracket,
    LeftParenthesis,
    RightParenthesis,
    Semicon,
}
#[derive(Debug,PartialEq, Eq)]
pub enum ScanError {
    BracketNotMatchError { line: usize, column: usize },
    ParenthesisNotMatchError { line: usize, column: usize },
    UnrecognizableTokenError,
}
impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &ScanError::BracketNotMatchError { line, column } => {
                write!(f, "Expected bracket in line {},column {}", line, column)
            }
            &ScanError::ParenthesisNotMatchError { line , column} => {
                write!(f, "Expected parenthesis in line{},column {}",line,column)
            }
            &ScanError::UnrecognizableTokenError => write!(f, "Unrecognizeable token",),
        }
    }
}
impl std::error::Error for ScanError {}
type ScannerResult<T> = Result<T, ScanError>;
pub struct Scanner {
    input: Vec<char>,
    cursor: usize,
}
impl Scanner {
    pub fn new(input: String) -> Scanner {
        Scanner {
            input: input.chars().map(|x| x).collect(),
            cursor: 0,
        }
    }
    pub fn to_tokens(&mut self) -> ScannerResult<Vec<MiniDecafTokens>> {
        let mut tokens = vec![];
        let (mut line, mut column) = (1usize, 1usize);
        let (mut bracket_stack,mut parenthesis_stack) = (vec![],vec![]);
        while self.cursor != self.input.len() {
            match &self.input[self.cursor] {
                // ignore whitespaces and newline , update cursor if current cursor is '\n'
                w@' '|w @'\t'|w@'\n'|w@'\r' =>{
                    self.cursor += 1;
                    if *w == '\n'{
                        line += 1;column = 1;
                    }else{
                        column += 1;
                    }
                },
                // LeftBracket
                // do the bracket match
                '{' => {
                    tokens.push(MiniDecafTokens::LeftBracket);
                    self.cursor += 1;
                    bracket_stack.push(('{',line,column));
                    column += 1;
                },
                // RightBracket
                '}' => {
                    tokens.push(MiniDecafTokens::RightBracket);
                    if !bracket_stack.is_empty(){
                        bracket_stack.pop();
                        self.cursor += 1;
                        column += 1;
                    }else{
                        return Err(ScanError::BracketNotMatchError{line,column});
                    }
                },
                // LeftParentsis
                // do the match
                '(' => {
                    tokens.push(MiniDecafTokens::LeftParenthesis);
                    self.cursor += 1;
                    parenthesis_stack.push(('(',line,column));
                    column += 1;
                },
                // RightParentsis
                ')' => {
                    tokens.push(MiniDecafTokens::RightParenthesis);
                    if !parenthesis_stack.is_empty(){
                        self.cursor += 1;
                        parenthesis_stack.pop();
                        column += 1;
                    }else{
                        return Err(ScanError::ParenthesisNotMatchError{line,column});
                    }
                },
                // Semicon
                ';' => {
                    tokens.push(MiniDecafTokens::Semicon);
                    self.cursor += 1;
                    column += 1;
                },
                // Integer
                digit@'0'..='9' => {
                    let mut number = vec![*digit];
                    if self.cursor != self.input.len() {
                        self.cursor += 1;
                        column += 1;
                    }
                    while self.cursor != self.input.len() && self.input[self.cursor].is_ascii_digit(){
                        number.push(self.input[self.cursor]);
                        self.cursor += 1;
                        column += 1;
                    }
                    let number = i32::from_str_radix(&number.into_iter().collect::<String>(),10).unwrap();
                    tokens.push(MiniDecafTokens::IntValue(number));
                },
                // Identifier and Return and Type Identifier
                begin_word@'a'..='z' => {
                    let mut identifier = vec![*begin_word];
                    if self.cursor != self.input.len(){
                        self.cursor += 1;
                        column += 1;
                    }
                    while self.cursor != self.input.len() && self.input[self.cursor].is_ascii_alphanumeric(){
                        identifier.push(self.input[self.cursor]);
                        self.cursor += 1;
                        column += 1;
                    }
                    let identifier:String = identifier.into_iter().collect();
                    if identifier == "int".to_owned(){
                        tokens.push(MiniDecafTokens::Int);
                    }else if identifier == "return".to_owned(){
                        tokens.push(MiniDecafTokens::Return);
                    }else{
                        tokens.push(MiniDecafTokens::Identifier(identifier));
                    }
                }
                _ => return Err(ScanError::UnrecognizableTokenError),
            }
        }
        if !bracket_stack.is_empty(){
            let (_,line,column) = bracket_stack.pop().unwrap();
            return Err(ScanError::BracketNotMatchError{line,column});
        }
        if !parenthesis_stack.is_empty(){
            let (_,line,column) = parenthesis_stack.pop().unwrap();
            return Err(ScanError::ParenthesisNotMatchError{line,column});
        }
        Ok(tokens)
    }
    ///debug-only methods
    fn reset(&mut self,input:String){
        self.cursor = 0;
        self.input = input.chars().collect();
    }
}
/// Parser for the MiniDecaf
/// BNF:
///     program    := function
///     function   := type Identifier Lparen Rparen Lbrace statement Rbrace
///     type       := Int
///     statement  := Return expression Semicolon
///     expression := Integer
#[derive(Debug)]
pub struct Program{
    pub func:Function
}
#[derive(Debug)]
pub struct Function{
    pub ty:MiniDecafTokens,
    pub identifier:String,
    pub stmt:Statement
}
#[derive(Debug)]
pub struct Statement{
    pub expr:Expression
}
#[derive(Debug)]
pub struct Expression{
    pub content:i32
}
type ParseResult<T> = Result<T,ParserError>;
#[derive(Debug)]
pub enum ParserError{
    UnexpectedExpressionError,
    SemicolonGoneError,
    UndefinedTypeError,
}
impl fmt::Display for ParserError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            &Self::UndefinedTypeError => {
                write!(f,"A unexpected expression occured!")
            },
            &Self::SemicolonGoneError => {
                write!(f,"Where is the end semicon?")
            },
            &Self::UndefinedTypeError => {
                write!(f,"What type is this?")
            }
        }
    }
}
impl std::error::Error for ParserError{}
pub struct MiniDecafParser{
    cursor:usize,
    tokens:Vec<MiniDecafTokens>
}
impl MiniDecafParser{
    pub fn new(tokens:Vec<MiniDecafTokens>) -> MiniDecafParser{
        MiniDecafParser{
            cursor:0,
            tokens
        }
    }
    pub fn parse_program(&mut self) -> Program{
        let func = self.parse_function();
        Program{func}
    }
    fn parse_function(&mut self) -> Function{
        let mut t = MiniDecafTokens::Int;
        if self.tokens[self.cursor] == MiniDecafTokens::Int{
            t = self.parse_type();
        }
        let mut name = "".to_owned();
        if let MiniDecafTokens::Identifier(id) = &self.tokens[self.cursor]{
            name = id.clone();
            self.cursor += 1;
        }
        if self.tokens[self.cursor] == MiniDecafTokens::LeftParenthesis{
            self.cursor += 1;
        }
        if self.tokens[self.cursor] == MiniDecafTokens::RightParenthesis{
            self.cursor += 1;
        }
        if self.tokens[self.cursor] == MiniDecafTokens::LeftBracket{
            self.cursor += 1;
        }
        let mut stmt = Statement{expr:Expression{content:2}};
        if self.tokens[self.cursor] == MiniDecafTokens::Return{
            stmt = self.parse_statement();
        }
        if self.tokens[self.cursor] == MiniDecafTokens::RightBracket{
            self.cursor += 1;
        }
        return Function{
            ty:t,
            identifier:name,
            stmt
        };
    }
    fn parse_type(&mut self) -> ParseResult<MiniDecafTokens>{
        if self.tokens[self.cursor] == MiniDecafTokens::Int{
            self.cursor += 1;
            return Ok(MiniDecafTokens::Int);
        }else{
            return Err(ParserError::UndefinedTypeError);
        }
    }
    fn parse_statement(&mut self) -> ParseResult<Statement>{
        if self.tokens[self.cursor] == MiniDecafTokens::Return{
            self.cursor += 1;
        }else{
            return Err(ParserError::UnexpectedExpressionError);
        }
        let expr = self.parse_expression()?;
        if self.tokens[self.cursor] == MiniDecafTokens::Semicon{
            self.cursor += 1;
        }else{
            return Err(ParserError::SemicolonGoneError);
        }
        return Ok(Statement{expr});
    }
    fn parse_expression(&mut self) -> ParseResult<Expression>{
        if let MiniDecafTokens::IntValue(val) = self.tokens[self.cursor]{
            self.cursor += 1;
            return Ok(Expression{content:val});
        }else{
            Err(ParserError::UnexpectedExpressionError)
        }
    }
}
#[cfg(test)]
mod roselia_test{
    use super::*;
    use MiniDecafTokens::*;
    use ScanError::*;
    #[test]
    fn test_if_lexer_works(){
        let k = "int main(){return 0;}".to_owned();
        let mut lexer = Scanner::new(k);
        assert_eq!(vec![Int,Identifier("main".to_owned()),LeftParenthesis,RightParenthesis,LeftBracket,Return,IntValue(0),Semicon,RightBracket],lexer.to_tokens().unwrap());
    }
    #[test]
    fn test_if_error_works(){
        let left_bracket_forgetted = "int main()return 0;}".to_owned();
        let mut lexer = Scanner::new(left_bracket_forgetted);
        assert_eq!(Err(BracketNotMatchError{line:1,column:20}),lexer.to_tokens());

        let right_bracket_forgetted = "int main(){return 0;".to_owned();
        lexer.reset(right_bracket_forgetted);
        assert_eq!(Err(BracketNotMatchError{line:1,column:11}),lexer.to_tokens());

        let left_parenthesis_forgetted = "int main){return 0;}".to_owned();
        lexer.reset(left_parenthesis_forgetted);
        assert_eq!(Err(ParenthesisNotMatchError{line:1,column:9}),lexer.to_tokens());

        let unrecongizeable_token = "int main({re*turn 0;}".to_owned();
        lexer.reset(unrecongizeable_token);
        assert_eq!(Err(UnrecognizableTokenError),lexer.to_tokens());
    }
    
}