use ib_pcode_compiler::ast::AST;
use ib_pcode_compiler::compiler::compile;
use ib_pcode_compiler::env::{Env, EnvMode};
use ib_pcode_compiler::run;
use std::collections::VecDeque;

pub fn compile_test(code: &str) -> AST {
    compile(code, true)
}

pub fn compile_run_check_logs(code: &str, mock_inputs: &str, logs: &str) -> Env {
    let ast = compile_test(code);
    run_check_logs(&ast, mock_inputs, logs)
}

pub fn run_check_logs(ast: &AST, mock_inputs: &str, logs: &str) -> Env {
    let mut mock_inputs_queue = VecDeque::new();

    for line in mock_inputs.trim().lines() {
        mock_inputs_queue.push_back(line.to_string());
    }

    let mut env = Env::test(mock_inputs_queue);
    run(ast, &mut env);

    assert_logs(&mut env, logs);
    env
}

pub fn assert_logs(env: &mut Env, expected_logs: &str) {
    match &mut env.mode {
        EnvMode::Release => panic!("Expected mode to be Test mode"),
        EnvMode::Test {
            mock_inputs: _,
            logs,
        } => {
            for (i, line) in expected_logs.trim().lines().enumerate() {
                let log = match logs.pop_front() {
                    Some(log) => log,
                    None => panic!("Expected log at line {}", i),
                };

                assert_eq!(line, log);
            }

            if !logs.is_empty() {
                panic!("Not all logs were checked, remaining: {}", logs.len());
            }
        }
    }
}
