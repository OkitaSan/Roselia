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
pub mod lexer;
pub mod parser;
pub mod visit;
#[cfg(test)]
mod roselia_test {
    use crate::lexer::MiniDecafTokens::*;
    use crate::lexer::ScanError::*;
    use crate::lexer::*;
    #[test]
    fn test_if_lexer_works() {
        let k = "int main(){return 0;}".to_owned();
        let mut lexer = Scanner::new(k);
        assert_eq!(
            vec![
                Int,
                Identifier("main".to_owned()),
                LeftParenthesis,
                RightParenthesis,
                LeftBracket,
                Return,
                IntValue(0),
                Semicon,
                RightBracket
            ],
            lexer.to_tokens().unwrap()
        );
    }
    #[test]
    fn test_if_error_works() {
        let left_bracket_forgetted = "int main()return 0;}".to_owned();
        let mut lexer = Scanner::new(left_bracket_forgetted);
        assert_eq!(
            Err(BracketNotMatchError {
                line: 1,
                column: 20
            }),
            lexer.to_tokens()
        );

        let right_bracket_forgetted = "int main(){return 0;".to_owned();
        lexer.reset(right_bracket_forgetted);
        assert_eq!(
            Err(BracketNotMatchError {
                line: 1,
                column: 11
            }),
            lexer.to_tokens()
        );

        let left_parenthesis_forgetted = "int main){return 0;}".to_owned();
        lexer.reset(left_parenthesis_forgetted);
        assert_eq!(
            Err(ParenthesisNotMatchError { line: 1, column: 9 }),
            lexer.to_tokens()
        );

        let unrecongizeable_token = "int main({re*turn 0;}".to_owned();
        lexer.reset(unrecongizeable_token);
        assert_eq!(Err(UnrecognizableTokenError), lexer.to_tokens());
    }
}
