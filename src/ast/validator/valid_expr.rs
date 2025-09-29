use crate::ast::{main_hash, AST};
use crate::data::ast_nodes::{Expr, ExprNode};
use crate::data::diagnostic::{Diagnostic, ErrorType};
use crate::data::Validator;
use crate::env::Env;

impl AST {
    pub fn valid_expr(&self, expr_node: &ExprNode, env: &mut Env, validator: &mut Validator) -> Result<(), Diagnostic> {
        match &expr_node.expr {
            Expr::Ident(name) =>  {
                let _ = match env.get(name) {
                    Some(_) => Ok(()),
                    None => expr_node.valid_error(
                        ErrorType::Uninitialized,
                        format!("Variable {} was not initialized", name).as_str(),
                        validator
                    ),
                };
                Ok(())
            }
            Expr::Data(_) => Ok(()),
            Expr::Array(data) => {
                for expr in data {
                    let _ = self.valid_expr(expr, env, validator);
                }
                Ok(())
            }
            Expr::Unary(_, expr) => {
                let _ = self.valid_expr(expr, env, validator);
                Ok(())
            }
            Expr::BinOp(left, _, right) => {
                let _ = self.valid_expr(left, env, validator);
                let _ = self.valid_expr(right, env, validator);
                Ok(())
            }
            Expr::MethodCall(fn_name, params) => {
                let class_name =  &env.get_local_env().class_name.clone();
                let _ = self.valid_fn_call(&expr_node.line_info, class_name, fn_name, validator);
                for expr in params {
                    let _ = self.valid_expr(expr, env, validator);
                }

                let fn_def = self.get_function(class_name, fn_name).ok_or_else(|| {
                    expr_node.compile_diagnostic(
                        ErrorType::Uninitialized,
                        format!("Undefined function in class {}", class_name).as_str(),
                        validator
                    )
                })?.clone();

                if class_name == main_hash() {
                    let _ = self.validate_fn_definition(class_name, fn_name, &fn_def, env, validator);
                }

                if !self.class_map[class_name].functions[fn_name].returns {
                    return expr_node.valid_error(
                        ErrorType::NoReturn,
                        format!(
                            "No return found for function {} in class {}",
                            fn_name, class_name
                        ).as_str(),
                        validator
                    )
                }

                Ok(())
            }
            Expr::SubstringCall { expr, start, end } => {
                let _ = self.valid_expr(expr, env, validator);
                let _ = self.valid_expr(start, env, validator);
                let _ = self.valid_expr(end, env, validator);
                Ok(())
            }
            Expr::LengthCall(expr) => {
                let _ = self.valid_expr(expr, env, validator);
                Ok(())
            }
            Expr::ClassNew(class_name_hash, params) => {
                let _ = self.valid_class_call(&expr_node.line_info, class_name_hash, validator);
                for expr in params {
                    let _ = self.valid_expr(expr, env, validator);
                }
                Ok(())
            }
            Expr::Call { expr, fn_name, params } => {
                let _ = self.valid_expr(expr, env, validator);

                for param in params {
                    let _ = self.valid_expr(param, env, validator);
                }

                Ok(())
            }
            Expr::Index(left, index) => {
                let _ = self.valid_expr(left, env, validator);
                let _ = self.valid_expr(index, env, validator);
                Ok(())
            }
            Expr::Input(text) => {
                let _ = self.valid_expr(text, env, validator);
                Ok(())
            }
            Expr::Div(left, right) => {
                let _ = self.valid_expr(left, env, validator);
                let _ = self.valid_expr(right, env, validator);
                Ok(())
            }
        }
    }
}