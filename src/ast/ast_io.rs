use crate::ast::AST;
use crate::data::Value;
use crate::env::{Env, EnvMode};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn blocking_request_input(prompt: &str) -> JsValue;
    pub fn write_output(s: &str);
}

impl AST {
    pub fn exec_input(ask_string: &str, env: &mut Env) -> Value {
        let user_string = match &mut env.mode {
            EnvMode::Release => {
                #[cfg(target_arch = "wasm32")]
                {
                    let jsv = blocking_request_input(ask_string);
                    jsv.as_string().unwrap_or_default()
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    print!("{}: ", ask_string);
                    std::io::stdout().flush().unwrap();

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    input
                }
            }
            EnvMode::Test {
                mock_inputs,
                logs: _,
            } => {
                mock_inputs.pop_front().expect("no mock input available")
            }
        };
        parse_input_to_value(user_string.trim())
    }

    pub fn exec_output(output: String, env: &mut Env) {
        match &mut env.mode {
            EnvMode::Release => {
                #[cfg(target_arch = "wasm32")]
                {
                    write_output(&output);
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    println!("{}", &output);
                }
            }
            EnvMode::Test {
                mock_inputs: _,
                logs,
            } => Env::record_log(logs, output),
        }
    }
}

fn parse_input_to_value(input: &str) -> Value {
    let input = input.trim();
    match input.parse::<f64>() {
        Ok(f) => Value::Number(f),
        Err(_) => Value::String(input.to_string()),
    }
}
