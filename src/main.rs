use crate::ast::AST;
use crate::env::Env;
use pest::Parser;
use pest_derive::Parser;

pub mod utils;
pub mod env;
pub mod ast;
pub mod ast_nodes;
pub mod ast_builder;
pub mod ast_evaluator;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DSLParser;

fn main() {
    let code = r#"
//*** INPUT, IF..THEN.. , and STRINGS ********************
//
// Lucy has trouble remembering things like: 1 km = 0.6 miles
// She wants to make a program that pops-up conversion factors.
// She wants to be able to type in a unit, like "km",
// and see a list of conversions, e.g.: 1 km = 1000 m = 0.6 miles
// This requires the ability to have a pop-up box that inputs data.
//
//********************************************************

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

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Add a few more conversion facts -
// maybe some money values, like  1 BP = 1.3 EUROS
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//==========================================
// INPUT - the input command makes a box pop-up on the screen,
//         waits for the user to type an answer,
//         and then stores the answer in a variable (UNIT).
//
// IF..THEN.. - checks whether UNIT matches a specific value.
//         If so, the program executes the command(s) between then..end if
//         Notice that the user is going to type letters, not numbers,
//         so you need "quotation marks" around the matching STRING.
//
// STRING - a STRING is a value that contains letters and maybe
//         numbers and punctuation marks -
//         e.g. any characters on a keyboard.
//==========================================

    "#;

    let mut ast = compile(code);
    println!("{:#?}", ast.method_map);
    println!("{}", ast);

    let mut env = Env::new();

    run(&mut ast, &mut env);
    println!("Final env: {:?}", env);
}

fn compile(code: &str) -> AST {
    let parsed = DSLParser::parse(Rule::program, code)
        .expect("parse failed")
        .next()
        .unwrap();

    let mut ast = AST::default();
    ast.build_ast(parsed);
    ast
}


fn run(ast: &mut AST, env: &mut Env) {
    ast.traverse(env);
}