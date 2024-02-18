use std::str::Chars;

#[derive(Debug, PartialEq)]
enum TokenType {
    Keyword,
    Identifier,
    Number,
    Symbol,
    Whitespace,
    Unknown,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
}

fn is_keyword(token: &str) -> bool {
    let keywords = vec![
        "fn", "if", "else", "while", "for", "in", "print", "true", "false", "None",
    ];
    keywords.contains(&token)
}

fn is_identifier(token: &str) -> bool {
    token.chars().next().unwrap().is_alphabetic()
}

fn is_number(token: &str) -> bool {
    token.parse::<f64>().is_ok()
}

fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = code.chars().peekable();
    let mut current_token = String::new();

    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            if !current_token.is_empty() {
                tokens.push(Token {
                    token_type: determine_token_type(&current_token),
                    lexeme: current_token.clone(),
                });
                current_token.clear();
            }
            continue;
        }

        if ch.is_ascii_punctuation() {
            if !current_token.is_empty() {
                tokens.push(Token {
                    token_type: determine_token_type(&current_token),
                    lexeme: current_token.clone(),
                });
                current_token.clear();
            }
            tokens.push(Token {
                token_type: TokenType::Symbol,
                lexeme: ch.to_string(),
            });
            continue;
        }

        if ch.is_ascii_alphabetic() || ch == '_' {
            let mut lookahead = *chars.peek().unwrap_or(&'\0');
            if lookahead.is_alphanumeric() || lookahead == '_' {
                current_token.push(ch);
                continue;
            }
        }

        current_token.push(ch);
    }

    if !current_token.is_empty() {
        tokens.push(Token {
            token_type: determine_token_type(&current_token),
            lexeme: current_token,
        });
    }

    tokens
}

fn determine_token_type(token: &str) -> TokenType {
    if is_keyword(token) {
        TokenType::Keyword
    } else if is_identifier(token) {
        TokenType::Identifier
    } else if is_number(token) {
        TokenType::Number
    } else if token.chars().all(|ch| ch.is_whitespace()) {
        TokenType::Whitespace
    } else {
        TokenType::Unknown
    }
}

fn main() {
    let code = r#"
       a = 5
       b: int = -43
       def hello_world():
            print('Hello, World!')

            hello_world()
    "#;

    let tokens = tokenize(code);
    for token in tokens {
        println!("{:?}: {}", token.token_type, token.lexeme);
    }
}