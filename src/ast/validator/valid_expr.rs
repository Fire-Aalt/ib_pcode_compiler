use std::cmp::PartialEq;
use crate::ast::{main_hash, AST};
use crate::ast::validator::Validator;
use crate::data::ast_nodes::{Expr, ExprNode};
use crate::data::diagnostic::{Diagnostic, ErrorType};
use crate::env::Env;

impl AST {
    pub fn valid_expr(&self, expr_node: &ExprNode, env: &mut Env, validator: &mut Validator) -> Result<(), Diagnostic> {
        match &expr_node.expr {
            Expr::Ident(name) => match env.get(name) {
                Some(_) => Ok(()),
                None => expr_node.valid_error(
                    ErrorType::Uninitialized,
                    format!("Variable {} was not initialized", name).as_str(),
                ),
            }
            Expr::Data(_) => Ok(()),
            Expr::Array(data) => {
                for expr in data {
                    self.valid_expr(expr, env, validator)?
                }
                Ok(())
            }
            Expr::Unary(_, expr) => {
                self.valid_expr(expr, env, validator)?;
                Ok(())
            }
            Expr::BinOp(left, _, right) => {
                self.valid_expr(left, env, validator)?;
                self.valid_expr(right, env, validator)?;
                Ok(())
            }
            Expr::MethodCall(fn_name, params) => {
                let class_name =  &env.get_local_env().class_name.clone();
                self.valid_fn_call(&expr_node.line_info, class_name, fn_name)?;
                for expr in params {
                    self.valid_expr(expr, env, validator)?
                }

                let fn_def = self.get_function(class_name, fn_name).ok_or_else(|| {
                    expr_node.diagnostic(
                        ErrorType::Uninitialized,
                        format!("Undefined function in class {}", class_name).as_str(),
                    )
                })?.clone();

                if class_name == main_hash() {
                    self.valid_fn_definition(class_name, fn_name, &fn_def, env, validator)?;
                }

                if !self.class_map[class_name].functions[fn_name].returns {
                    return expr_node.valid_error(
                        ErrorType::NoReturn,
                        format!(
                            "No return found for function {} in class {}",
                            fn_name, class_name
                        ).as_str(),
                    )
                }

                Ok(())
            }
            Expr::SubstringCall { expr, start, end } => {
                self.valid_expr(expr, env, validator)?;
                self.valid_expr(start, env, validator)?;
                self.valid_expr(end, env, validator)?;
                Ok(())
            }
            Expr::LengthCall(expr) => {
                self.valid_expr(expr, env, validator)?;
                Ok(())
            }
            Expr::ClassNew(class_name_hash, params) => {
                self.valid_class_call(&expr_node.line_info, class_name_hash)?;
                for expr in params {
                    self.valid_expr(expr, env, validator)?
                }
                Ok(())
            }
            Expr::Call { expr, fn_name, params } => {
                self.valid_expr(expr, env, validator)?;

                for param in params {
                    self.valid_expr(param, env, validator)?;
                }

                Ok(())
            }
            Expr::Index(left, index) => {
                self.valid_expr(left, env, validator)?;
                self.valid_expr(index, env, validator)?;
                Ok(())
            }
            Expr::Input(text) => {
                self.valid_expr(text, env, validator)?;
                Ok(())
            }
            Expr::Div(left, right) => {
                self.valid_expr(left, env, validator)?;
                self.valid_expr(right, env, validator)?;
                Ok(())
            }
        }
    }
}