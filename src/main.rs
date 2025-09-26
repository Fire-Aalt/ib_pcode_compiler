use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"
NAMES = ["Alex","Bobby","Cho","Deke"]

STACK = new Stack()

loop COUNT from 0 to 3
   STACK.push(NAMES[COUNT])
end loop

loop while NOT(STACK.isEmpty())
   NAME = STACK.pop()
   output NAME
end loop

    "#;

    compile_and_run(code);
}