use ib_pseudocompiler::compile_and_run;

fn main() {
    let code = r#"
A = [
      [8,1,6] ,
      [3,5,7] ,
      [4,9,2]
    ]

OK = "correct"

loop R from 0 to 2
   output A[R][0] , " " , A[R][1] , " " , A[R][2]
end loop

loop R from 0 to 2
   SUM = 0
   loop C from 0 to 2
      SUM = SUM + A[R][C]
   end loop

   if SUM != 15 then
      output "Row " , R , " is wrong"
      OK = "wrong"
   end if
end loop

loop C from 0 to 2
   SUM = 0
   loop R from 0 to 2
      SUM = SUM + A[R][C]
   end loop

   if SUM != 15 then
      output "Column " , C , " is wrong"
      OK = "wrong"
   end if
end loop

SUM = 0
loop X from 0 to 2
   R = X
   C = X
   SUM = SUM + A[R][C]
end loop

if SUM != 15 then
   output "Main diag is wrong"
   OK = "wrong"
end if

SUM = 0
loop X from 0 to 2
   R = X
   C = 2-X
   SUM = SUM + A[R][C]
end loop

if SUM != 15 then
   output "Other diag is wrong"
   OK = "wrong"
end if

output "Entire square is " , OK   

    "#;

    compile_and_run(code);
}