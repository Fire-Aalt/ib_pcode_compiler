use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"

method Get()
  return "ggg"
end method

output Get()
A = [5, [485, Get()], "HI", Get()]
output A[1][0]
    "#;

    compile_and_run(code);
}