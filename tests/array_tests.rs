use crate::common::compile_run_check_logs;

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

#[test]
fn selection_sort() {
    let code = r#"
NUMS = [15,30,85,25,40,90,50,65,20,60]

output "Before sorting"
printNums()

loop FIRST from 0 to 9
    LEAST = FIRST
    loop CURRENT from FIRST+1 to 9
        if NUMS[CURRENT] > NUMS[LEAST] then
           LEAST = CURRENT
        end if
    end loop
    TEMP = NUMS[LEAST]
    NUMS[LEAST] = NUMS[FIRST]
    NUMS[FIRST] = TEMP
end loop

output "After sorting"
printNums()

method printNums()
   loop C from 0 to 9
      output NUMS[C]
   end loop
   output "========"
end method
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Before sorting
15
30
85
25
40
90
50
65
20
60
========
After sorting
90
85
65
60
50
40
30
25
20
15
========
"#,
    );
}

#[test]
fn bubble_sort() {
    let code = r#"
NUMS = [15,30,85,25,40,90,50,65,20,60]

output "Before sorting"
loop C from 0 to 9
   output NUMS[C]
end loop

output "========"

loop PASS from 0 to 8
    loop CURRENT from 0 to 8
        if NUMS[CURRENT] < NUMS[CURRENT + 1] then
          TEMP = NUMS[CURRENT]
          NUMS[CURRENT] = NUMS[CURRENT+1]
          NUMS[CURRENT+1] = TEMP
        end if
    end loop
end loop

output "After sorting"

loop C from 0 to 9
   output NUMS[C]
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Before sorting
15
30
85
25
40
90
50
65
20
60
========
After sorting
90
85
65
60
50
40
30
25
20
15
"#,
    );
}

#[test]
fn reverse_array() {
    let code = r#"
NAMES = ["Robert","Boris","Brad","George","David"]

N = 5     // the number of elements in the array
K = 0     // this is the first index in the array

loop while K < N - 1
TEMP = NAMES[K]
NAMES[K] = NAMES[N - K - 1]
NAMES[N - K - 1] = TEMP
K = K + 1
end loop

loop C from 0 to N-1
output NAMES[C]
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
David
Boris
Brad
George
Robert
"#,
    );
}

#[test]
fn frequency_distribution() {
    let code = r#"
DATA = [17,20,23,29,33,42,60,61,75,75,90,99]
FREQS = [0,0,0,0,0,0,0,0,0,0]

loop C from 0 to 11
VALUE = DATA[C]
loop F from 0 to 9
if VALUE >= 10*F AND VALUE < 10*(F+1) then
FREQS[F] = FREQS[F] + 1
end if
end loop
end loop

output "Data"

loop D from 0 to 11
output DATA[D]
end loop

output "Range : Frequency"

loop F from 0 to 9
output F*10 , " - " , (F+1)*10 , " : " , FREQS[F]
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Data
17
20
23
29
33
42
60
61
75
75
90
99
Range : Frequency
0 - 10 : 0
10 - 20 : 1
20 - 30 : 3
30 - 40 : 1
40 - 50 : 1
50 - 60 : 0
60 - 70 : 2
70 - 80 : 2
80 - 90 : 0
90 - 100 : 2
"#,
    );
}

#[test]
fn appointments_list() {
    let code = r#"
APPS = new Array()
NAME = ""

loop T from 0 to 2400
APPS[T] = ""
end loop

loop while NAME <> "quit"
input NAME
input TIME
if TIME >= 0 AND TIME <= 2359 then
APPS[TIME] = NAME
end if

loop T from 0 to 2400
if APPS[T] <> "" then
output T , " : " , APPS[T]
end if
end loop
output "=================="
end loop
    "#;

    compile_run_check_logs(
        code,
        r#"
Me
1230
Brown
2350
quit
0
        "#,
        r#"
1230 : Me
==================
1230 : Me
2350 : Brown
==================
0 : quit
1230 : Me
2350 : Brown
==================
"#,
    );
}


#[test]
fn find_duplicates() {
    let code = r#"
TENNIS = ["Al","Bobby","Carla","Dave","Ellen"]
BBALL = ["Lou","Dave","Hellen","Alan","Al"]

output "The following appear in both lists"

loop T from 0 to 4
loop B from 0 to 4
if TENNIS[T] = BBALL[B] then
output TENNIS[T]
end if
end loop
end loop
    "#;

    compile_run_check_logs(
        code,
        r#""#,
        r#"
The following appear in both lists
Al
Dave
"#,
    );
}

#[test]
fn cities_array() {
    let code = r#"
CITIES = ["Athens","Berlin","Dallas","Denver","London","New York","Rome"]

COUNT = 0

loop C from 0 to 6
if firstLetter( CITIES[C] ) = "D" then
COUNT = COUNT + 1
output CITIES[C]
end if
end loop

output "That was " , COUNT , " D-cities"

method firstLetter(s)
return s.substring(0,1)
end method
    "#;

    compile_run_check_logs(
        code,
        r#""#,
        r#"
Dallas
Denver
That was 2 D-cities
"#,
    );
}

#[test]
fn magic_square() {
    let code = r#"
    A = [
      [8,1,6] ,
      [3,5,7] ,
      [4,9,2]
    ]

OK = "correct"

loop R from 0 to 2
   output A[R][0] , A[R][1] , A[R][2]
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

    compile_run_check_logs(
        code,
        r#""#,
        r#"
8 1 6
3 5 7
4 9 2
Entire square is correct
"#);
}
