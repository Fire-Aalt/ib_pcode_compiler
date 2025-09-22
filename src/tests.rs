use crate::ast_nodes::Value;
use super::*;

#[test]
fn case_sensitive() {
    let code = r#"
        X = 4.5
        A = "fsdfsdf"

        // comment
        loop I from -81 to 10
            if X >= -99 then
                X = X - 1.5
            end if
        end loop
    "#;

    let env = compile_and_run(code);
    assert_env(&env, "A", &Value::String("fsdfsdf".to_string()));
    assert_env(&env, "I", &Value::Number(10.0));
    assert_env(&env, "X", &Value::Number(-100.5));
}

fn compile_and_run(code: &str) -> Env {
    let mut ast = compile(code);
    let mut env = Env::new();
    run(&mut ast, &mut env);
    env
}

fn assert_env(env: &Env, var_name: &str, expected: &Value) {
    let var = env.get(var_name).unwrap();

    let correct = match var {
        Value::Number(n) => {
            match expected {
                Value::Number(e_n) => n == *e_n,
                Value::String(e_s) => panic!("Expected {} but got {}", e_s, n),
            }
        }
        Value::String(ref s) => {
            match expected {
                Value::Number(e_n) => panic!("Expected {} but got {}", e_n, s),
                Value::String(e_s) => s == e_s
            }
        }
    };
    assert!(correct, "Environment variable wasn't as expected. Expected {} but got {}", expected, var);
}
