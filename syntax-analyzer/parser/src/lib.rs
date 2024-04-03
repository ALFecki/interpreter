#![doc(html_logo_url = "https://raw.githubusercontent.com/RustPython/RustPython/main/logo.png")]
#![doc(html_root_url = "https://docs.rs/rustpython-parser/")]

pub use rustpython_ast as ast;
#[cfg(feature = "location")]
pub use rustpython_parser_core::source_code;
pub use rustpython_parser_core::{text_size, Mode};

mod function;
// Skip flattening lexer to distinguish from full parser
mod context;
pub mod lexer;
mod parser;
mod soft_keywords;
mod string;
mod token;

pub use parser::{parse, parse_starts_at, parse_tokens, Parse, ParseError, ParseErrorType};
pub use string::FStringErrorType;
pub use token::{StringKind, Tok};

#[allow(deprecated)]
pub use parser::{parse_expression, parse_expression_starts_at, parse_program};

#[rustfmt::skip]
mod python {
    #![allow(clippy::all)]
    #![allow(unused)]

    #[cfg(feature = "lalrpop")]
    include!(concat!(env!("OUT_DIR"), "/src/python.rs"));

    #[cfg(not(feature = "lalrpop"))]
    include!("python.rs");
}
