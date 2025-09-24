use crate::common::{compile_run_check_logs, run_check_logs};
use ib_pseudocompiler::compiler::compile;

pub mod common;

#[test]
fn primes_array() {
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

    compile_run_check_logs(
        code,
        "",
        r#"
These are the PRIME numbers under 100
2
3
5
7
11
13
17
19
23
29
31
37
41
43
47
53
59
61
67
71
73
79
83
89
97
"#,
    );
}


#[test]
fn binary_search() {
    let code = r#"
ID = [1001,1002,1050,1100,1120,1180,1200,1400]
NAME = ["Apple","Cherry","Peach","Banana","Fig","Grape","Olive","Mango"]

output "Type the ID number that you wish to find"
input TARGET

LOW = 0
HIGH = 7
FOUND = -1

loop while FOUND = -1 AND LOW <= HIGH
MID = div( LOW + HIGH , 2 )   // should be (LOW + HIGH) div 2
// but (A div B) doesn't work correctly
// so there is a special method below
if ID[MID] = TARGET then
FOUND = MID
else if TARGET < ID[MID] then
HIGH = MID - 1
else
LOW = MID + 1
end if
end while

if FOUND >= 0 then
output TARGET , ":" , NAME[FOUND]
else
output TARGET , " was not found"
end if
    "#;

    compile_run_check_logs(
        code,
        "1050",
        r#"
Type the ID number that you wish to find
1050 : Peach
"#,
    );
}