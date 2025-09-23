use crate::ast_nodes::Value;
use super::*;

#[test]
fn intro() {
    let code = r#"
output "Welcome"
loop COUNT from 1 to 5
  output COUNT
end loop
    "#;

    let env = compile_and_run(code);
    assert_logs(&env, r#"
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

    let env = compile_and_run(code);
    assert_logs(&env, r#"
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


fn compile_and_run(code: &str) -> Env {
    let mut ast = compile(code);
    let mut env = Env::new();
    run(&mut ast, &mut env);
    env
}

fn assert_logs(env: &Env, expected_logs: &str) {
    for (i, line) in expected_logs.trim().lines().enumerate() {
        assert_eq!(line, &env.logs[i]);
    }
}

fn assert_env(env: &Env, var_name: &str, expected: &Value) {
    let var = env.get(var_name).unwrap();

    let correct = match var {
        Value::Number(n) => {
            match expected {
                Value::Number(e_n) => n == *e_n,
                _ => panic!("Expected {} but got {}", expected, n),
            }
        }
        Value::String(ref s) => {
            match expected {
                Value::String(e_s) => s == e_s,
                _ => panic!("Expected {} but got {}", expected, s),
            }
        }
        Value::Bool(b) => {
            match expected {
                Value::Bool(e_b) => b == *e_b,
                _ => panic!("Expected {} but got {}", expected, b),
            }
        }
    };
    assert!(correct, "Environment variable wasn't as expected. Expected {} but got {}", expected, var);
}
