use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"
output 9.5 div 2
    "#;

    compile_and_run(code);
}