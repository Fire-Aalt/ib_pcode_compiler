use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"

X = 150

output X / 427493

loop I from 0 to 45

end loop

    "#;

    compile_and_run(code);
}