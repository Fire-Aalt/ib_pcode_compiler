use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"
PEOPLE = ["Alex","Bobby","Cho","Deke","Ellen"]
DOGS = ["spot","woofie","bruiser"]

PQ = new Queue()          // Queue for People names
DQ = new Queue()          // Queue for dog names

/////////////////////////// copy people names
loop P from 0 to 4
   PQ.enqueue(PEOPLE[P])
end loop

//////////////////////////// copy dog names
loop D from 0 to 2
   DQ.enqueue(DOGS[D])
end loop

loop while NOT(PQ.isEmpty()) OR NOT(DQ.isEmpty())
   if NOT(PQ.isEmpty()) then
      output "Person = " , PQ.dequeue()
   else
      output "People list is empty"
   end if

   if NOT(DQ.isEmpty()) then
      output "Dog = " , DQ.dequeue()
   else
      output "Dog list is empty"
   end if
end loop

    "#;

    compile_and_run(code);
}