use super::*;
use std::collections::VecDeque;

#[test]
fn intro() {
    let code = r#"
output "Welcome"
loop COUNT from 1 to 5
  output COUNT
end loop
    "#;

    compile_run_check_logs(code, "", r#"
Welcome
1
2
3
4
5
"#);
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

    compile_run_check_logs(code, "", r#"
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
"#);
}

#[test]
fn solve_equations() {
    let code = r#"
X = 4
Y = X*X - 9*X + 14
output "x = " , X , " .... y = " , Y
   "#;

    compile_run_check_logs(code, "", r#"
x = 4 .... y = -6
"#);
}

#[test]
fn solving2() {
    let code = r#"
A = 10
B = 100
output "Sum = " , A + B
output "Product = " , A * B
   "#;

    compile_run_check_logs(code, "", r#"
Sum = 110
Product = 1000
"#);
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

    compile_run_check_logs(code, "", r#"
8 cars and 8 busses = 192 seats
10 rooms and 12 lodges = 184 beds
Total cost = 22600
"#);
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

    run_check_logs(&ast, "km", r#"
1 km = 1000 m = 0.6 miles
"#);
    run_check_logs(&ast, "mi", r#"
1 mi = 5280 ft = 1.6 km
"#);
    run_check_logs(&ast, "ft", r#"
1 ft = 12 in = 30.5 cm
"#);
    run_check_logs(&ast, "liter", r#"
1 liter = 1000 ml = 1/3 gallon
Don't forget that IMPERIAL GALLONS
are different than US GALLONS
"#);
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

    run_check_logs(&ast, r#"
bozo
clown
"#, r#"
Correct!
"#);

    run_check_logs(&ast, r#"
einstein
e=mc2
"#, r#"
Correct!
"#);

    run_check_logs(&ast, r#"
guest
WRONG
"#, r#"
You will be logged in as a GUEST
"#);

    run_check_logs(&ast, r#"
trial
WRONG
"#, r#"
You will be logged in as a GUEST
"#);

    run_check_logs(&ast, r#"
WRONG
WRONG
"#, r#"
"#);
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

    run_check_logs(&ast, "12", r#"
That costs 2.59 per burger
Total cost = 31.08 for 12 burgers
"#);
    run_check_logs(&ast, "7", r#"
That costs 2.89 per burger
Total cost = 20.23 for 7 burgers
"#);
    run_check_logs(&ast, "3", r#"
That costs 3.25 per burger
Total cost = 9.75 for 3 burgers
"#);
}




fn compile_run_check_logs(code: &str, mock_inputs: &str, logs: &str) -> Env {
    let ast = compile(code);
    run_check_logs(&ast, mock_inputs, logs)
}

fn run_check_logs(ast: &AST, mock_inputs: &str, logs: &str) -> Env {
    let mut mock_inputs_queue = VecDeque::new();

    for line in mock_inputs.trim().lines() {
        mock_inputs_queue.push_back(line.to_string());
    }

    let mut env = Env::new(mock_inputs_queue, true);
    run(ast, &mut env);

    assert_logs(&mut env, logs);
    env
}

fn assert_logs(env: &mut Env, expected_logs: &str) {
    for (i, line) in expected_logs.trim().lines().enumerate() {

        let log = match env.logs.pop_front() {
            Some(log) => log,
            None => panic!("Expected log at line {}", i)
        };

        assert_eq!(line, log);
    }

    if !env.logs.is_empty() {
        panic!("Not all logs were checked, remaining: {}", env.logs.len());
    }
}