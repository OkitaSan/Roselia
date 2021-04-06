use core::fmt;

    use super::lexer::*;
    use fmt::write;
    /// Parser for the MiniDecaf
    /// BNF:
    ///     program    := function
    ///     function   := type Identifier Lparen Rparen Lbrace statement Rbrace
    ///     type       := Int
    ///     statement  := Return expression Semicolon
    ///     expression := Integer
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
        pub content: i32,
    }
    type ParseResult<T> = Result<T, ParserError>;
    #[derive(Debug)]
    pub enum ParserError {
        UnexpectedTokenError,
        SemicolonGoneError,
        UndefinedTypeError,
        MissingIdentifierError,
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
            }
        }
    }
    impl std::error::Error for ParserError {}
    pub struct MiniDecafParser {
        cursor: usize,
        tokens: Vec<MiniDecafTokens>,
    }
    impl MiniDecafParser {
        pub fn new(tokens: Vec<MiniDecafTokens>) -> MiniDecafParser {
            MiniDecafParser { cursor: 0, tokens }
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
            if let MiniDecafTokens::IntValue(val) = self.tokens[self.cursor] {
                self.cursor += 1;
                return Ok(Expression { content: val });
            } else {
                Err(ParserError::UnexpectedTokenError)
            }
        }
    }