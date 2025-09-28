use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"
A = 1
output X
    "#;

    compile_and_run(code);
}