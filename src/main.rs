use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"

output G

    "#;

    compile_and_run(code);
}