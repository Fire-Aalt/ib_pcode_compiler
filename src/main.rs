use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"

method Get()
  return "ggg"
end method

A = [5, Get(), 485, "HI", true]
output A
    "#;

    compile_and_run(code);
}