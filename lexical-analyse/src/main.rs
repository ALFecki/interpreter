use std::fmt::Debug;
use std::str::Chars;
use crate::Token::Variable;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Keyword(String),
    Identifier(String),
    Variable((Type, String)),
    Operator(String),
    Constant(Type),
    Indent,
    Dedent,
    Newline,
    LexicalError(String),
}

#[derive(Debug, PartialEq, Clone)]
enum Type {
    Integer(i32),
    Float(f64),
    String(String),
    Char(char),
}

struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
    current_index: i32,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        let mut lexer = Lexer {
            input: input.chars(),
            current_char: None,
            current_index: 0,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
        self.current_index += 1;
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut indentation_stack = Vec::new();
        let mut current_indentation = 0;

        loop {
            let current_index = self.current_index;
            match self.current_char {
                Some(' ') => {
                    self.advance();
                }
                Some('\n') => {
                    self.current_index = 0;
                    self.advance();
                    tokens.push((Token::Newline, self.current_index));
                    let mut spaces_count = 0;
                    while let Some(' ') = self.current_char {
                        self.advance();
                        spaces_count += 1;
                    }
                    if spaces_count > current_indentation {
                        tokens.push((Token::Indent, self.current_index));
                        indentation_stack.push(current_indentation);
                        current_indentation = spaces_count;
                    } else {
                        while spaces_count < current_indentation {
                            tokens.push((Token::Dedent, self.current_index));
                            current_indentation = indentation_stack.pop().unwrap();
                        }
                    }
                }
                Some(ch) => {
                    if ch.is_ascii_digit() || ch == '-' {
                        tokens.push((self.consume_number(), current_index));
                    } else if ch.is_alphabetic() {
                        tokens.push((self.consume_identifier(), current_index));
                    } else {
                        match ch {
                            '#' => {
                                self.advance();
                                self.consume_comment();
                            }
                            '"' => {
                                self.advance();
                                tokens.push((self.consume_string_literal(), current_index));
                            }
                            _ => {
                                tokens.push((self.consume_operator(), current_index));
                            }
                        }
                    }
                }
                None => break,
            }
        }

        while current_indentation > 0 {
            tokens.push((Token::Dedent, self.current_index));
            current_indentation = indentation_stack.pop().unwrap();
        }
        let mut verified_tokens = self.verify_output(tokens);
        self.add_data_types(verified_tokens.clone().len(), &mut verified_tokens, );
        Ok(verified_tokens)
    }

    fn consume_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut is_float = false;
        let mut is_negative = false;

        if let Some(ch) = self.current_char {
            if ch == '-' {
                is_negative = true;
                self.advance();
            }
        }

        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else if ch == '.' && !is_float {
                num_str.push(ch);
                self.advance();
                is_float = true;
            } else {
                break;
            }
        }

        if is_float {
            if let Ok(f) = num_str.parse::<f64>() {
                if is_negative {
                    Token::Constant(Type::Float(-f))
                } else {
                    Token::Constant(Type::Float(f))
                }
            } else {
                panic!("Invalid float number: {}", num_str);
            }
        } else {
            if let Ok(i) = num_str.parse::<i32>() {
                if is_negative {
                    Token::Constant(Type::Integer(-i))
                } else {
                    Token::Constant(Type::Integer(i))
                }
            } else {
                panic!("Invalid integer: {}", num_str);
            }
        }
    }

    fn consume_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "if" | "else" | "for" | "while" | "def" | "class" | "and" | "or" | "is" | "not" => Token::Keyword(identifier),
            _ => Token::Identifier(identifier),
        }
    }

    fn consume_comment(&mut self) {
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn consume_string_literal(&mut self) -> Token {
        let mut literal = String::new();
        let mut is_escaped = false;

        while let Some(ch) = self.current_char {
            if ch == '\n' {
                panic!("Unterminated string literal");
            }

            if is_escaped {
                match ch {
                    'n' => literal.push('\n'),
                    't' => literal.push('\t'),
                    '\\' => literal.push('\\'),
                    '"' => literal.push('"'),
                    _ => literal.push(ch),
                }
                is_escaped = false;
            } else if ch == '\\' {
                is_escaped = true;
            } else if ch == '"' {
                self.advance();
                break;
            } else {
                literal.push(ch);
            }
            self.advance();
        }

        Token::Constant(Type::String(literal))
    }

    fn consume_operator(&mut self) -> Token {
        let mut operator = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_ascii_whitespace() || ch.is_alphanumeric() || ch == '"' {
                break;
            }
            operator.push(ch);
            self.advance();
        }
        if operator == "\"" {
            self.advance();
            return self.consume_string_literal();
        }
        Token::Operator(operator)
    }

    fn verify_output(&mut self, tokens: Vec<(Token, i32)>) -> Vec<Token> {
        let mut line_number = 1;
        let mut new_tokens: Vec<Token> = Vec::new();
        for (index, (token, _)) in tokens.iter().enumerate() {
            // for ((token_prev, _), (token_next, ind_next)) in tokens.iter().zip(tokens.iter().skip(1)) {
            if index == tokens.len() - 1 {
                break;
            }
            if index == 0 {
                continue;
            }
            match tokens[index + 1].clone() {
                (Token::Operator(a), column_num) => {
                    if a != ":" && a != ")" && a != "]" {
                        match token {
                            Token::Identifier(_) => {}
                            _ => {
                                new_tokens.push(Token::LexicalError(format!("Invalid order {:?}:{:?}: {:?} cannot be before {:?}", line_number, column_num, *token, tokens[index + 1].0)));
                            }
                        }
                    } else if a == ":" {
                        // let (new_line, indent) = (new_tokens[new_tokens.len() - 1].clone(), new_tokens[new_tokens.len() - 2].clone());
                        let (new_line, column_number) = tokens[index - 1].clone();
                        if new_line != Token::Newline || *token != Token::Indent {
                            new_tokens.push(Token::LexicalError(format!("Invalid intent {:?}:{:?}", line_number + 1, column_number)))
                        }
                    }
                }
                (Token::Newline, _) => {
                    line_number += 1;
                }
                (Token::Indent, _) => {}
                _ => {}
            }
            if *token != Token::Indent && *token != Token::Newline && *token != Token::Dedent {
                new_tokens.push(token.clone())
            }
        }

        return new_tokens;
    }

    fn add_data_types(&self, size: usize, tokens: &mut [Token]) {
        for i in 0..size {
            match tokens[i].clone() {
                Token::Identifier(name) => {
                    if i < size - 3 && tokens[i + 1] == Token::Operator("=".to_string()) {
                        match tokens[i + 2].clone() {
                            Token::Constant(t) => {
                                tokens[i] = Token::Variable((t, name))
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let input = r#"
        x = -5
        k = 10

        e = 1e10
        if x < 10:
        print("Hello, world!")
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    match tokens {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}