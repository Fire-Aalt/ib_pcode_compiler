use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"


Class GG()

this.show = function() {

}

end Class

output new GG().show()

    "#;

    compile_and_run(code);
}