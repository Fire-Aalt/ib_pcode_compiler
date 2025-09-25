use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"


Class Bank(A)


this.create = function(A)
{
  output "Hi"
}

end class



    "#;

    compile_and_run(code);
}