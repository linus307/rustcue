pub mod lexer;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_lexer() {
        let input = "fn _#lol x let y Int αβ Int x  y ";
        let expected = vec![
            lexer::Token::Identifier("fn".to_string()),
            lexer::Token::Identifier("_#lol".to_string()),
            lexer::Token::Identifier("x".to_string()),
            lexer::Token::Let,
            lexer::Token::Identifier("y".to_string()),
            lexer::Token::Identifier("Int".to_string()),
            lexer::Token::Identifier("αβ".to_string()),
            lexer::Token::Identifier("Int".to_string()),
            lexer::Token::Identifier("x".to_string()),
            lexer::Token::Identifier("y".to_string()),
        ];
        let mut lexer = lexer::Lexer::new(input);
        let result = lexer.lex();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_numbers() {
        let input = "123 456 789";
        let expected = vec![
            lexer::Token::IntLit(lexer::IntLit::DecimalLit("123".to_string())),
            lexer::Token::IntLit(lexer::IntLit::DecimalLit("456".to_string())),
            lexer::Token::IntLit(lexer::IntLit::DecimalLit("789".to_string())),
        ];
        let mut lexer = lexer::Lexer::new(input);
        let result = lexer.lex();
        assert_eq!(result, expected);

        let input = "42 1.5G 1.3Ki 170_141_183_460_469_231_731_687_303_715_884_105_727 0xBad_Face 0o755 0b0101_0001";
        let expected = vec![
            lexer::Token::IntLit(lexer::IntLit::DecimalLit("42".to_string())),
            lexer::Token::IntLit(lexer::IntLit::SiLit("1.5".to_string(), "G".to_string())),
            lexer::Token::IntLit(lexer::IntLit::SiLit("1.3".to_string(), "Ki".to_string())),
            lexer::Token::IntLit(lexer::IntLit::DecimalLit(
                "170_141_183_460_469_231_731_687_303_715_884_105_727".to_string(),
            )),
            lexer::Token::IntLit(lexer::IntLit::HexLit("0xBad_Face".to_string())),
            lexer::Token::IntLit(lexer::IntLit::OctalLit("0o755".to_string())),
            lexer::Token::IntLit(lexer::IntLit::BinaryLit("0b0101_0001".to_string())),
        ];
        let mut lexer = lexer::Lexer::new(input);
        let result = lexer.lex();
        assert_eq!(result, expected);

        let input = "0. 72.40 072.40 2.71828 1.e+0 6.67428e-11 1E6 .25 .12345E+5";
        let expected = vec![
            lexer::Token::FloatLit(lexer::FloatLit::WithoutExp("0.".to_string())),
            lexer::Token::FloatLit(lexer::FloatLit::WithoutExp("72.40".to_string())),
            lexer::Token::FloatLit(lexer::FloatLit::WithoutExp("072.40".to_string())),
            lexer::Token::FloatLit(lexer::FloatLit::WithoutExp("2.71828".to_string())),
            lexer::Token::FloatLit(lexer::FloatLit::WithExp(
                "1.".to_string(),
                "e+0".to_string(),
            )),
            lexer::Token::FloatLit(lexer::FloatLit::WithExp(
                "6.67428".to_string(),
                "e-11".to_string(),
            )),
            lexer::Token::FloatLit(lexer::FloatLit::WithExp("1".to_string(), "E6".to_string())),
            lexer::Token::FloatLit(lexer::FloatLit::WithoutExp(".25".to_string())),
            lexer::Token::FloatLit(lexer::FloatLit::WithExp(
                ".12345".to_string(),
                "E+5".to_string(),
            )),
        ];
        let mut lexer = lexer::Lexer::new(input);
        let result = lexer.lex();
        assert_eq!(result, expected);
    }
}
