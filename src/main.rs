use std::fs;
use std::ops::AddAssign;
use ib_pseudocompiler::compile_and_run;

const SOURCE: &str = "source";

fn main() {
    let mut contents = fs::read_to_string(SOURCE)
        .expect("Should have been able to read the file");
    contents.add_assign("\n");
    
    compile_and_run(contents.as_str());
}