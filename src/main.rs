use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"
A = [
      [8,1,6] ,
      [3,5,7] ,
      [4,9,2]
    ]

A[0][0]++

output A

    "#;

    compile_and_run(code);
}