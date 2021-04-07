use core::fmt;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
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
    /// !
    LogicalNot,
    /// ~
    BitwiseNot,
    /// -
    Minus
}
impl From<ParseIntError> for ScanError{
    fn from(_: ParseIntError) -> Self {
        Self::NumberOverflowError
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum ScanError {
    BracketNotMatchError { line: usize, column: usize },
    ParenthesisNotMatchError { line: usize, column: usize },
    UnrecognizableTokenError,
    NumberOverflowError,
}
impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &ScanError::BracketNotMatchError { line, column } => {
                write!(f, "Expected bracket in line {},column {}", line, column)
            }
            &ScanError::ParenthesisNotMatchError { line, column } => {
                write!(f, "Expected parenthesis in line{},column {}", line, column)
            }
            &ScanError::UnrecognizableTokenError => write!(f, "Unrecognizeable token",),
            &ScanError::NumberOverflowError => write!(f,"number overflow")
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
        let (mut bracket_stack, mut parenthesis_stack) = (vec![], vec![]);
        while self.cursor != self.input.len() {
            match &self.input[self.cursor] {
                // ignore whitespaces and newline , update cursor if current cursor is '\n'
                w @ ' ' | w @ '\t' | w @ '\n' | w @ '\r' => {
                    self.cursor += 1;
                    if *w == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                }
                // LeftBracket
                // do the bracket match
                '{' => {
                    tokens.push(MiniDecafTokens::LeftBracket);
                    self.cursor += 1;
                    bracket_stack.push(('{', line, column));
                    column += 1;
                }
                // RightBracket
                '}' => {
                    tokens.push(MiniDecafTokens::RightBracket);
                    if !bracket_stack.is_empty() {
                        bracket_stack.pop();
                        self.cursor += 1;
                        column += 1;
                    } else {
                        return Err(ScanError::BracketNotMatchError { line, column });
                    }
                }
                // LeftParentsis
                // do the match
                '(' => {
                    tokens.push(MiniDecafTokens::LeftParenthesis);
                    self.cursor += 1;
                    parenthesis_stack.push(('(', line, column));
                    column += 1;
                }
                // RightParentsis
                ')' => {
                    tokens.push(MiniDecafTokens::RightParenthesis);
                    if !parenthesis_stack.is_empty() {
                        self.cursor += 1;
                        parenthesis_stack.pop();
                        column += 1;
                    } else {
                        return Err(ScanError::ParenthesisNotMatchError { line, column });
                    }
                }
                // Semicon
                ';' => {
                    tokens.push(MiniDecafTokens::Semicon);
                    self.cursor += 1;
                    column += 1;
                }
                // Integer
                digit @ '0'..='9' => {
                    let mut number = vec![*digit];
                    if self.cursor != self.input.len() {
                        self.cursor += 1;
                        column += 1;
                    }
                    while self.cursor != self.input.len()
                        && self.input[self.cursor].is_ascii_digit()
                    {
                        number.push(self.input[self.cursor]);
                        self.cursor += 1;
                        column += 1;
                    }
                    let number =
                        i32::from_str_radix(&number.into_iter().collect::<String>(), 10)?;
                    tokens.push(MiniDecafTokens::IntValue(number));
                }
                // Identifier and Return and Type Identifier
                begin_word @ 'a'..='z' => {
                    let mut identifier = vec![*begin_word];
                    if self.cursor != self.input.len() {
                        self.cursor += 1;
                        column += 1;
                    }
                    while self.cursor != self.input.len()
                        && self.input[self.cursor].is_ascii_alphanumeric()
                    {
                        identifier.push(self.input[self.cursor]);
                        self.cursor += 1;
                        column += 1;
                    }
                    let identifier: String = identifier.into_iter().collect();
                    if identifier == "int".to_owned() {
                        tokens.push(MiniDecafTokens::Int);
                    } else if identifier == "return".to_owned() {
                        tokens.push(MiniDecafTokens::Return);
                    } else {
                        tokens.push(MiniDecafTokens::Identifier(identifier));
                    }
                },
                '-' => {
                    tokens.push(MiniDecafTokens::Minus);
                    self.cursor += 1;
                    column += 1;
                },
                '!' => {
                    tokens.push(MiniDecafTokens::LogicalNot);
                    self.cursor += 1;
                    column += 1;
                },
                '~' => {
                    tokens.push(MiniDecafTokens::BitwiseNot);
                    self.cursor += 1;
                    column += 1;
                }
                _ => return Err(ScanError::UnrecognizableTokenError),
            }
        }
        if !bracket_stack.is_empty() {
            let (_, line, column) = bracket_stack.pop().unwrap();
            return Err(ScanError::BracketNotMatchError { line, column });
        }
        if !parenthesis_stack.is_empty() {
            let (_, line, column) = parenthesis_stack.pop().unwrap();
            return Err(ScanError::ParenthesisNotMatchError { line, column });
        }
        Ok(tokens)
    }
    ///debug-only methods
    pub fn reset(&mut self, input: String) {
        self.cursor = 0;
        self.input = input.chars().collect();
    }
}
