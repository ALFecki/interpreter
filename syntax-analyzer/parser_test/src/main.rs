use std::fs::File;
use std::io::Read;
use rustpython_parser::{Parse, ast};

fn main() {
    let mut file = File::open("./test.py").expect("Unable to open the file");
    let mut python_source = String::new();
    file.read_to_string(&mut python_source).expect("Unable to read the file");

    let python_statements = ast::Suite::parse_without_path(python_source.as_str());

    match python_statements {
        Ok(stmts) => {println!("{:#?}", stmts);},
        Err(e) => { println!("ERROR IN PARSING: {}", e)}
    }
}