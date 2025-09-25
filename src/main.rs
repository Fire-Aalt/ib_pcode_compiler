use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"


Class Bank(A)
  this.grann = A

this.create = function(A)
{
  output "Hi"
  return 8
}

end class


vk = new Bank(1)

TR = vk.create(44)
output TR

    "#;

    compile_and_run(code);
}