use crate::{
    errors::{LexerError, LexerResult},
    token_type::TokenKind,
};

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    loc: Location,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    fn advance(&mut self, text: &str) {
        // Newline sepeartor for each line in code
        match text.rfind("\n") {
            Some(idx) => {
                self.line += text.chars().filter(|&it| it == '\n').count();
                self.column = text[idx + 1..].chars().count();
            }
            None => self.column += text.chars().count(),
        }
    }
}

fn tokenize(mut input: &str) -> LexerResult<Vec<Token>> {
    let mut res = Vec::new();
    let mut loc = Location::default();

    while !input.is_empty() {
        let old_input = input;

        // Skip leading whitespace and comments before tokenizing input
        skip_whitespace(&mut input);
        skip_comment(&mut input);

        // If whitespace or comments were consumed, update location
        // and start the next iteration.
        if input.len() != old_input.len() {
            let consumed = old_input.len() - input.len();
            loc.advance(&old_input[..consumed]);
            continue;
        }

        let kind = advance(&mut input).map_err(|err| err.with_location(loc))?;
        res.push(Token { kind, loc });

        // Update new location as tokens are consumed
        let consumed = old_input.len() - input.len();
        loc.advance(&old_input[..consumed]);

        dbg!(&res);
    }

    // Add EOF token last
    res.push(Token {
        kind: TokenKind::Eof,
        loc,
    });

    Ok(res)
}

fn skip_whitespace(input: &mut &str) {
    *input = input.trim_start_matches(char::is_whitespace)
}

fn skip_comment(input: &mut &str) {
    if input.starts_with("//") {
        let idx = input.find('\n').map_or(input.len(), |it| it + 1);
        *input = &input[idx..]
    }
}

fn advance(input: &mut &str) -> LexerResult<TokenKind> {
    let mut chars = input.chars();
    dbg!(&chars);
    let c = chars.next().unwrap();
    dbg!(&c);
    let res = match &c {
        ',' => TokenKind::Comma,
        '.' => TokenKind::Dot,
        '{' => TokenKind::OpenBrace,
        '}' => TokenKind::CloseBrace,
        '(' => TokenKind::OpenParen,
        ')' => TokenKind::CloseParen,
        '+' => TokenKind::Plus,
        '-' => TokenKind::Minus,
        ';' => TokenKind::Semicolon,
        '*' => TokenKind::Star,
        '/' => TokenKind::Slash,
        // Operators
        '!' => {
            if chars.clone().next() == Some('=') {
                chars.next();
                TokenKind::BangEq
            } else {
                TokenKind::Bang
            }
        }
        '>' => {
            if chars.clone().next() == Some('=') {
                chars.next();
                TokenKind::GtEq
            } else {
                TokenKind::Gt
            }
        }
        '<' => {
            if chars.clone().next() == Some('=') {
                chars.next();
                TokenKind::LtEq
            } else {
                TokenKind::Lt
            }
        }
        '=' => {
            if chars.clone().next() == Some('=') {
                chars.next();
                TokenKind::EqEq
            } else {
                TokenKind::Eq
            }
        }
        // String Literals
        '"' => {
            // Look for the end " literal before the line ends
            let mut buf = String::new();
            loop {
                match chars.next() {
                    Some('"') => break,
                    Some('\n') | None => {
                        return Err(LexerError::new("Unterminated string literal"));
                    }
                    Some(c) => buf.push(c),
                }
            }
            TokenKind::StringLiteral(buf)
        }
        // Integer and Float Numbers
        // Edge cases: 1. or 1.2.3
        c if c.is_numeric() => {
            // Look for "." for float or end of numbers or ; or newline
            let mut buff = String::new();
            buff.push(*c);
            let mut is_float = false;
            loop {
                let next_char = chars.clone().next();
                dbg!(&next_char);
                match next_char {
                    Some(c) if c.is_numeric() => {
                        chars.next();
                        buff.push(c)
                    }
                    Some('.') => {
                        let mut look_ahead = chars.clone();
                        look_ahead.next();
                        dbg!(&look_ahead);
                        if look_ahead.next().is_some_and(|c| c.is_numeric()) {
                            buff.push('.');
                            chars.next();
                            is_float = true
                        } else {
                            return Err(LexerError::new("Invalid numeric literal"));
                        }
                    }
                    Some(_) | None => break,
                }
            }
            if is_float {
                TokenKind::FNumber(buff.parse::<f64>().unwrap())
            } else {
                TokenKind::INumber(buff.parse::<i64>().unwrap())
            }
        }
        // Identifer Literal
        // Exclude digits for start of the identifier
        c if c.is_ascii_alphabetic() || *c == '_' => {
            // Look for valid alphanumeric characters
            let mut buff = String::new();
            buff.push(*c);
            loop {
                match chars.clone().next() {
                    // Include digits as well along with ascii
                    Some(c) if c.is_ascii_alphanumeric() || c == '_' => {
                        chars.next();
                        buff.push(c)
                    }

                    _ => break,
                }
            }

            TokenKind::Ident(buff)
        }

        // Reserved Keywords
        _ => todo!(),
    };

    dbg!(&chars);

    *input = chars.as_str();
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_comment_only() {
        let lox_code = "// nothing here";
        let expected = vec![Token {
            kind: TokenKind::Eof,
            loc: Location {
                line: 0,
                column: 15,
            },
        }];
        assert_eq!(tokenize(lox_code).unwrap(), expected);
    }

    #[test]
    fn tokenize_trailing_whitespace() {
        let lox_code = "=   ";

        let expected = vec![
            Token {
                kind: TokenKind::Eq,
                loc: Location { line: 0, column: 0 },
            },
            Token {
                kind: TokenKind::Eof,
                loc: Location { line: 0, column: 4 },
            },
        ];

        assert_eq!(tokenize(lox_code).unwrap(), expected);
    }

    #[test]
    fn location_advance_single_line_input() {
        let mut location = Location::default();
        let test_lox_code = "x=0;\n";
        location.advance(test_lox_code);
        assert_eq!(location.line, 1);
        assert_eq!(location.column, 0);
    }

    #[test]
    fn location_advance_multi_line_input() {
        let mut location = Location::default();
        let test_lox_code = "x=0;\nx+=1;\n";
        location.advance(test_lox_code);
        assert_eq!(location.line, 2);
        assert_eq!(location.column, 0);
    }

    #[test]
    fn location_advance_no_newline_input() {
        let mut location = Location::default();
        let test_lox_code = "x=0;";
        location.advance(test_lox_code);
        assert_eq!(location.line, 0);
        assert_eq!(location.column, 4);
    }

    #[test]
    fn location_advance_multiline_with_indentation() {
        let mut location = Location::default();

        location.advance("x=0;\n    x+=1;");

        assert_eq!(location.line, 1);
        assert_eq!(location.column, 9);
    }

    #[test]
    fn tokenize_single_token() {
        let lox_code = ".";
        let res = tokenize(lox_code);
        let mut expected = Vec::new();
        let equal_token = Token {
            kind: TokenKind::Dot,
            loc: Location { line: 0, column: 0 },
        };
        let eof_token = Token {
            kind: TokenKind::Eof,
            loc: Location { line: 0, column: 1 },
        };
        expected.push(equal_token);
        expected.push(eof_token);
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn tokenize_single_token_with_comments() {
        let lox_code = "//This is a comment\n.";
        let res = tokenize(lox_code);
        let mut expected = Vec::new();
        let equal_token = Token {
            kind: TokenKind::Dot,
            loc: Location { line: 1, column: 0 },
        };
        let eof_token = Token {
            kind: TokenKind::Eof,
            loc: Location { line: 1, column: 1 },
        };
        expected.push(equal_token);
        expected.push(eof_token);
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn tokenize_multiline_locations() {
        let lox_code = "==\n  !=\n    +";
        let expected = vec![
            Token {
                kind: TokenKind::EqEq,
                loc: Location { line: 0, column: 0 },
            },
            Token {
                kind: TokenKind::BangEq,
                loc: Location { line: 1, column: 2 },
            },
            Token {
                kind: TokenKind::Plus,
                loc: Location { line: 2, column: 4 },
            },
            Token {
                kind: TokenKind::Eof,
                loc: Location { line: 2, column: 5 },
            },
        ];
        assert_eq!(tokenize(lox_code).unwrap(), expected);
    }

    #[test]
    fn tokenize_string_literal() {
        let lox_code = r#""hello""#;
        let expected = vec![
            Token {
                kind: TokenKind::StringLiteral("hello".to_string()),
                loc: Location { line: 0, column: 0 },
            },
            Token {
                kind: TokenKind::Eof,
                loc: Location { line: 0, column: 7 },
            },
        ];
        assert_eq!(tokenize(lox_code).unwrap(), expected);
    }

    #[test]
    fn tokenize_unterminated_string() {
        let lox_code = r#""hello"#;
        let result = tokenize(lox_code);
        assert!(result.is_err());
    }

    #[test]
    fn tokenize_integer_literal() {
        let lox_code = r#"1"#;
        let expected = vec![
            Token {
                kind: TokenKind::INumber(1),
                loc: Location { line: 0, column: 0 },
            },
            Token {
                kind: TokenKind::Eof,
                loc: Location { line: 0, column: 1 },
            },
        ];
        dbg!(tokenize(lox_code));
        assert_eq!(tokenize(lox_code).unwrap(), expected);
    }

    #[test]
    fn tokenize_float_literal() {
        let lox_code = r#"1.23"#;
        let expected = vec![
            Token {
                kind: TokenKind::FNumber(1.23),
                loc: Location { line: 0, column: 0 },
            },
            Token {
                kind: TokenKind::Eof,
                loc: Location { line: 0, column: 4 },
            },
        ];
        dbg!(tokenize(lox_code));
        assert_eq!(tokenize(lox_code).unwrap(), expected);
    }

    #[test]
    fn tokenize_alphanumeric_identifier() {
        let lox_code = "hello123";
        let expected = vec![
            Token {
                kind: TokenKind::Ident("hello123".to_string()),
                loc: Location { line: 0, column: 0 },
            },
            Token {
                kind: TokenKind::Eof,
                loc: Location { line: 0, column: 8 },
            },
        ];
        assert_eq!(tokenize(lox_code).unwrap(), expected);
    }

    #[test]
    fn tokenize_number_followed_by_operator() {
        let lox_code = "1+2";

        let expected = vec![
            Token {
                kind: TokenKind::INumber(1),
                loc: Location { line: 0, column: 0 },
            },
            Token {
                kind: TokenKind::Plus,
                loc: Location { line: 0, column: 1 },
            },
            Token {
                kind: TokenKind::INumber(2),
                loc: Location { line: 0, column: 2 },
            },
            Token {
                kind: TokenKind::Eof,
                loc: Location { line: 0, column: 3 },
            },
        ];

        dbg!(tokenize(lox_code));
        assert_eq!(tokenize(lox_code).unwrap(), expected);
    }

    #[test]
    fn tokenize_multiline_code() {
        let lox_code = concat!(r#"y = "hello";"#, "\n", " x != 1");
        let expected = vec![
            Token {
                kind: TokenKind::Ident("y".to_string()),
                loc: Location { line: 0, column: 0 },
            },
            Token {
                kind: TokenKind::Eq,
                loc: Location { line: 0, column: 2 },
            },
            Token {
                kind: TokenKind::StringLiteral("hello".to_string()),
                loc: Location { line: 0, column: 4 },
            },
            Token {
                kind: TokenKind::Semicolon,
                loc: Location {
                    line: 0,
                    column: 11,
                },
            },
            Token {
                kind: TokenKind::Ident("x".to_string()),
                loc: Location { line: 1, column: 1 },
            },
            Token {
                kind: TokenKind::BangEq,
                loc: Location { line: 1, column: 3 },
            },
            Token {
                kind: TokenKind::INumber(1),
                loc: Location { line: 1, column: 6 },
            },
            Token {
                kind: TokenKind::Eof,
                loc: Location { line: 1, column: 7 },
            },
        ];
        assert_eq!(tokenize(lox_code).unwrap(), expected);
    }

    // #[test]
    // fn tokenize_unexpected_character() {
    //     let lox_code = "@";

    //     let result = tokenize(lox_code);

    //     assert!(result.is_err());
    // }
}
