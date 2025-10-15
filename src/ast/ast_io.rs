use crate::ast::AST;
use crate::data::Value;
use crate::env::{Env, EnvMode};
use std::io::{self, Write};


impl AST {
    pub fn exec_input(ask_string: &str, env: &mut Env) -> Value {
        let mut input;

        match &mut env.mode {
            EnvMode::Release => {
                print!("{}: ", ask_string);
                io::stdout().flush().unwrap();

                input = String::new();
                io::stdin().read_line(&mut input).unwrap();
            }
            EnvMode::Test {
                mock_inputs,
                logs: _,
            } => {
                input = mock_inputs.pop_front().unwrap();
            }
        }
        Self::parse_input_to_value(input.trim())
    }

    pub fn exec_output(output: String, env: &mut Env) {
        match &mut env.mode {
            EnvMode::Release => println!("{}", &output),
            EnvMode::Test {
                mock_inputs: _,
                logs,
            } => Env::record_log(logs, output),
        }
    }

    fn parse_input_to_value(input: &str) -> Value {
        let input = input.trim();
        match input.parse::<f64>() {
            Ok(f) => Value::Number(f),
            Err(_) => Value::String(input.to_string()),
        }
    }
}