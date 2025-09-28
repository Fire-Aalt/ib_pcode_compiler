use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"

Class New()
  this.show = function()
  {

  }

end Class

NUMS = [15,30,85,25,
40,90,50,
65,20,60]


X = 54

output NUMS.show()
    "#;

    compile_and_run(code);
}