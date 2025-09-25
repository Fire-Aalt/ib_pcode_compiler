use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"


Class Bank(A)
  this.grann = A

this.create = function(A)
{
  output "Hi"
}

end class


vk = new Bank(1)


    "#;

    compile_and_run(code);
}