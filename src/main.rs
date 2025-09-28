use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"


Class GG()



end Class

output new GG().Show()

    "#;

    compile_and_run(code);
}