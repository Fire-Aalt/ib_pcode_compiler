use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"

NUMS = new Array()

loop N from 1 to 100
   NUMS[N] = 0
end loop

loop P from 2 to 50
   N = P * 2
   loop while N <= 100
      NUMS[N] = 1
      N = N + P
   end loop
end loop

output "These are the PRIME numbers under 100"

loop N from 2 to 100
   if NUMS[N] = 0 then
      output N
   end if
end loop
    "#;

    compile_and_run(code);
}