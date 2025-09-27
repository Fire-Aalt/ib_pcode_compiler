use crate::common::{compile_run_check_logs, run_check_logs};
use ib_pseudocompiler::compiler::compile;

mod common;

#[test]
fn intro() {
    let code = r#"
output "Welcome"
loop COUNT from 1 to 5
  output COUNT
end loop
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Welcome
1
2
3
4
5
"#,
    );
}

#[test]
fn calculations() {
    let code = r#"
output "=== Simple Calculations ==="

output "Adding 1...10 = " , 1+2+3+4+5+6+7+8+9+10

output "10 Factorial = " , 1*2*3*4*5*6*7*8*9*10

output "Fractions = 1/2 + 1/4 + 1/5 = " , 1/2 + 1/4 + 1/5

output "Pythagoras = 3^2 + 4^2 = 5^2 = " , 3*3 + 4*4 , " and " , 5*5

output "Big Numbers = one trillion ^ 2 = " , 1000000000000 * 1000000000000

output "Easier big numbers = " , 2e12 * 3e12

output "10307 is not prime = " , 10307 / 11 , " * " , 11

output "15% of 12345 = " , 15/100*12345

output "Incorrect calculation = " , 1234567890 * 1234567890

output "Another error = " , 1/2 + 1/3 + 1/6

output "One more problem = " , 0.1+0.1+0.1+0.1+0.1+0.1+0.1+0.1

output "And another problem = " , 3.2 - 0.3
    "#;

    compile_run_check_logs(
        code,
        "",
        r#"
=== Simple Calculations ===
Adding 1...10 = 55
10 Factorial = 3628800
Fractions = 1/2 + 1/4 + 1/5 = 0.95
Pythagoras = 3^2 + 4^2 = 5^2 = 25 and 25
Big Numbers = one trillion ^ 2 = 1e24
Easier big numbers = 6e24
10307 is not prime = 937 * 11
15% of 12345 = 1851.75
Incorrect calculation = 1524157875019052000
Another error = 0.9999999999999999
One more problem = 0.7999999999999999
And another problem = 2.9000000000000004
"#,
    );
}

#[test]
fn solve_equations() {
    let code = r#"
X = 4
Y = X*X - 9*X + 14
output "x = " , X , " .... y = " , Y
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
x = 4 .... y = -6
"#,
    );
}

#[test]
fn solving2() {
    let code = r#"
A = 10
B = 100
output "Sum = " , A + B
output "Product = " , A * B
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Sum = 110
Product = 1000
"#,
    );
}

#[test]
fn ski_trip() {
    let code = r#"
 CARS = 8
 BUSSES = 8
 HOTEL = 10
 LODGE = 12
 SEATS = CARS*4 + BUSSES*20
 BEDS = HOTEL*4 + LODGE*12
 COST = CARS*250 + BUSSES*1000 + HOTEL*300 + LODGE*800

 output CARS , " cars and " , BUSSES , " busses = " , SEATS , " seats"
 output HOTEL , " rooms and " , LODGE , " lodges = " , BEDS , " beds"
 output "Total cost = " , COST
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
8 cars and 8 busses = 192 seats
10 rooms and 12 lodges = 184 beds
Total cost = 22600
"#,
    );
}

#[test]
fn if_then() {
    let code = r#"
  UNIT = input("Type a unit")

  if  UNIT = "km"  then
     output "1 km = 1000 m = 0.6 miles"
  end if

  if  UNIT = "mi"  then
     output "1 mi = 5280 ft = 1.6 km"
  end if

  if  UNIT = "ft"  then
     output "1 ft = 12 in = 30.5 cm"
  end if

  if  UNIT = "liter"  then
     output "1 liter = 1000 ml = 1/3 gallon"
     output "Don't forget that IMPERIAL GALLONS"
     output "are different than US GALLONS"
  end if
   "#;

    let ast = compile(code);

    run_check_logs(
        &ast,
        "km",
        r#"
1 km = 1000 m = 0.6 miles
"#,
    );
    run_check_logs(
        &ast,
        "mi",
        r#"
1 mi = 5280 ft = 1.6 km
"#,
    );
    run_check_logs(
        &ast,
        "ft",
        r#"
1 ft = 12 in = 30.5 cm
"#,
    );
    run_check_logs(
        &ast,
        "liter",
        r#"
1 liter = 1000 ml = 1/3 gallon
Don't forget that IMPERIAL GALLONS
are different than US GALLONS
"#,
    );
    run_check_logs(&ast, "WRONG", "");
}

#[test]
fn password_logic() {
    let code = r#"
 NAME = input("Type your user name")
 PASSWORD = input("Type your password")

 if  NAME = "bozo"  AND  PASSWORD = "clown"  then
    output "Correct!"
 end if

 if  NAME = "einstein"  AND  PASSWORD = "e=mc2"  then
    output "Correct!"
 end if

 if  NAME = "guest"  OR  NAME = "trial"  then
    output "You will be logged in as a GUEST"
 end if
   "#;

    let ast = compile(code);

    run_check_logs(
        &ast,
        r#"
bozo
clown
"#,
        r#"
Correct!
"#,
    );

    run_check_logs(
        &ast,
        r#"
einstein
e=mc2
"#,
        r#"
Correct!
"#,
    );

    run_check_logs(
        &ast,
        r#"
guest
WRONG
"#,
        r#"
You will be logged in as a GUEST
"#,
    );

    run_check_logs(
        &ast,
        r#"
trial
WRONG
"#,
        r#"
You will be logged in as a GUEST
"#,
    );

    run_check_logs(
        &ast,
        r#"
WRONG
WRONG
"#,
        r#"
"#,
    );
}

#[test]
fn discount_logic() {
    let code = r#"
QUANTITY = input("How many hamburgers do you want?")

if  QUANTITY >= 10  then
PRICE = 2.59
else if  QUANTITY <= 9  AND  QUANTITY >= 5  then
PRICE = 2.89
else if  QUANTITY < 5  then
PRICE = 3.25
end if

output "That costs " , PRICE , " per burger"
output "Total cost = " , PRICE * QUANTITY , " for " , QUANTITY , " burgers"
   "#;

    let ast = compile(code);

    run_check_logs(
        &ast,
        "12",
        r#"
That costs 2.59 per burger
Total cost = 31.08 for 12 burgers
"#,
    );
    run_check_logs(
        &ast,
        "7",
        r#"
That costs 2.89 per burger
Total cost = 20.23 for 7 burgers
"#,
    );
    run_check_logs(
        &ast,
        "3",
        r#"
That costs 3.25 per burger
Total cost = 9.75 for 3 burgers
"#,
    );
}

#[test]
fn mice_loops() {
    let code = r#"
loop A from 1 to 2
   output "Three blind mice"
end loop

loop B from 3 to 4
   output "See how they run"
end loop

output "They all ran up to the farmer's wife"
output "She cut off their tails with a carving knife"
output "Did you ever see such a sight in your life, as"

C = 5
loop while C < 20
   output "Three blind mice"
   C = C*2
end loop
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Three blind mice
Three blind mice
See how they run
See how they run
They all ran up to the farmer's wife
She cut off their tails with a carving knife
Did you ever see such a sight in your life, as
Three blind mice
Three blind mice
"#,
    );
}

#[test]
fn money_decisions() {
    let code = r#"
EUROS = 50.00

POUNDS = 0.8*EUROS

DOLLARS = EUROS / 0.75

YEN = EUROS * 90

output EUROS , " EUR"

output YEN , " Yen"

if YEN > 1000 then
output "That is a lot of Yen"
end if

output POUNDS , " BP"

if POUNDS < 100 then
output "That is a small number of Pounds"
end if

output "$" , DOLLARS

if DOLLARS = 100 then
output "BINGO"
end if
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
50 EUR
4500 Yen
That is a lot of Yen
40 BP
That is a small number of Pounds
$ 66.66666666666667
"#,
    );
}

#[test]
fn common_factors() {
    let code = r#"
A = 28
B = 42

output "Common factors of " , A , " and " , B

loop C from 1 to B
    if (A mod C = 0) AND (B mod C = 0) then
       output C
    end if
end loop
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
Common factors of 28 and 42
1
2
7
14
"#,
    );
}

#[test]
fn math_values() {
    let code = r#"
output "X , Y"

loop C from 0 to 10
   X = C / 2.0
   Y = 3*X*X - 7*X + 2
   output X , " , " , Y
end loop
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
X , Y
0 , 2
0.5 , -0.75
1 , -2
1.5 , -1.75
2 , 0
2.5 , 3.25
3 , 8
3.5 , 14.25
4 , 22
4.5 , 31.25
5 , 42
"#,
    );
}

#[test]
fn collatz_sequence() {
    let code = r#"
NUM = 29

loop until NUM = 1

output NUM

if NUM mod 2 = 0 then
NUM = NUM / 2
else
NUM = NUM * 3 + 1
end if

end loop

output NUM
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
29
88
44
22
11
34
17
52
26
13
40
20
10
5
16
8
4
2
1
"#,
    );
}

#[test]
fn translate_strings() {
    let code = r#"
input ENGLISH

if ENGLISH = "hello" then
 GERMAN = "guten Tag"
else if ENGLISH = "goodbye" then
 GERMAN = "auf Wiedersehen"
else if ENGLISH = "stop" then
 GERMAN = "halt"
else
 GERMAN = "???"
end if

output "English = " , ENGLISH
output "German = " , GERMAN
   "#;

    let ast = compile(code);

    run_check_logs(
        &ast,
        "hello",
        r#"
English = hello
German = guten Tag
"#,
    );
    run_check_logs(
        &ast,
        "goodbye",
        r#"
English = goodbye
German = auf Wiedersehen
"#,
    );
    run_check_logs(
        &ast,
        "stop",
        r#"
English = stop
German = halt
"#,
    );
    run_check_logs(
        &ast,
        "WRONG",
        r#"
English = WRONG
German = ???
"#,
    );
}

#[test]
fn elapsed_minutes() {
    let code = r#"
input START_HOURS
input START_MINUTES

input END_HOURS
input END_MINUTES

if START_HOURS > 23 OR START_MINUTES > 59 then
 output "Start time is not valid"
else if END_HOURS > 23 OR END_MINUTES > 59 then
 output "Times are not valid"
else
 MINUTES = (END_HOURS - START_HOURS)*60 + (END_MINUTES-START_MINUTES)
 output "Elapsed time = " , MINUTES , " minutes"
end if
   "#;

    let ast = compile(code);

    run_check_logs(
        &ast,
        r#"
4
8
8
2"#,
        r#"
Elapsed time = 234 minutes
"#,
    );
    run_check_logs(
        &ast,
        r#"
8
8
28
8"#,
        r#"
Times are not valid
"#,
    );
}

#[test]
fn date_validation() {
    let code = r#"
input MONTH
input DAY
input YEAR

output MONTH , "/" , DAY , "/" , YEAR

if YEAR mod 4 = 0 then
   FEBMAX = 29
else
   FEBMAX = 28
end if

M = MONTH
D = DAY

if M < 1 OR M > 12 then
    output "Month is not valid"
else if D < 1 OR D > 31 then
    output "Day is not valid"
else if D = 31 AND (M = 4 OR M = 6 OR M = 9 OR M = 11) then
    output "That month does not have 31 days"
else if M = 2 AND D > FEBMAX then
    output "February only has " , FEBMAX , " days"
else
    output "Date is valid"
end if
   "#;

    let ast = compile(code);

    run_check_logs(
        &ast,
        r#"
8
84
2
"#,
        r#"
8 / 84 / 2
Day is not valid
"#,
    );
    run_check_logs(
        &ast,
        r#"
8
20
2004
"#,
        r#"
8 / 20 / 2004
Date is valid
"#,
    );
}

#[test]
fn add_up_number() {
    let code = r#"
MAX = 10

SUM = 0

loop COUNT from 0 to MAX
    output COUNT
    SUM = SUM + COUNT
end loop

output "Total = " , SUM
   "#;

    compile_run_check_logs(
        code,
        "",
        r#"
0
1
2
3
4
5
6
7
8
9
10
Total = 55
"#,
    );
}
