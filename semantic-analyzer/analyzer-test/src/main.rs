use std::path::PathBuf;
use erg_compiler::artifact::{CompleteArtifact, IncompleteArtifact};
use erg_common::config::ErgConfig;
use erg_common::io::Input;
use erg_common::traits::Stream;
use pylyzer::PythonAnalyzer;

pub fn exec_analyzer(file_path: &'static str) -> Result<CompleteArtifact, IncompleteArtifact> {
    println!("Start checking!");
    let cfg = ErgConfig {
        input: Input::file(PathBuf::from(file_path)),
        ..Default::default()
    };
    let mut analyzer = PythonAnalyzer::new(cfg);
    let py_code = analyzer.cfg.input.read();
    analyzer.analyze(py_code, "exec")
}

fn main() {
    match exec_analyzer("./test.py") {
        Ok(artifact) => {
            println!("All checks are done!");
            println!("Warnings founded in code: {}\n {}", artifact.warns.len(), artifact.warns);
        },
        Err(artifact) => {
            println!("Error in analyzing code!");
            println!("Errors founded in code: {}\n {}", artifact.errors.len(), artifact.errors);
            println!("Warnings founded in code: {}\n {}", artifact.warns.len(), artifact.warns);
        }
    }
}