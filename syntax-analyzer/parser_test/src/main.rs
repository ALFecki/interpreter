use std::fs::File;
use std::io::Read;
use rustpython_parser::{Parse, ast};

fn main() {
    let mut file = File::open("./test.py").expect("Unable to open the file");
    let mut python_source = String::new();
    file.read_to_string(&mut python_source).expect("Unable to read the file");
    // let ast = ast::Suite::parse("", "./test.py");
    let python_statements = ast::Suite::parse(python_source.as_str(), "<test>").unwrap();  // statements
    // let python_expr = ast::Expr::parse(python_source, "<embedded>").unwrap();  // or expr
    println!("{:#?}", python_statements);


    // assert!(ast.is_ok());
}