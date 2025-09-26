use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"
NAMES = new Collection()

NAME = ""

loop while NAME <> "quit"
   input NAME
   if NAME <> "quit" then
       if NAMES.contains(NAME) then
           output NAME , " returned"
           NAMES.remove(NAME)
       else
           output NAME , " is leaving"
           NAMES.addItem(NAME)
       end if
   end if
end loop

output "The following students left and did not return"

NAMES.resetNext()

loop while NAMES.hasNext()
   output NAMES.getNext()
end loop

    "#;

    compile_and_run(code);
}