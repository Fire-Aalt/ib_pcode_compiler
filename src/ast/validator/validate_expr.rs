use crate::ast::{AST, main_hash};
use crate::data::Validator;
use crate::data::ast_nodes::{Expr, ExprNode};
use crate::data::diagnostic::{Diagnostic, ErrorType};
use crate::env::Env;

impl AST {
    pub fn validate_expr(
        &self,
        expr_node: &ExprNode,
        env: &mut Env,
        validator: &mut Validator,
    ) -> Result<(), Diagnostic> {
        match &expr_node.expr {
            Expr::Ident(name) => {
                let _ = env.get(name).ok_or_else(|| expr_node.compile_error(
                    ErrorType::Uninitialized,
                    format!("cannot find variable `{}` in this scope", name).as_str(),
                    "not found in this scope",
                    validator,
                ));
                Ok(())
            }
            Expr::Data(_) => Ok(()),
            Expr::Array(data) => {
                for expr in data {
                    let _ = self.validate_expr(expr, env, validator);
                }
                Ok(())
            }
            Expr::Unary(_, expr) => {
                let _ = self.validate_expr(expr, env, validator);
                Ok(())
            }
            Expr::BinOp(left, _, right) => {
                let _ = self.validate_expr(left, env, validator);
                let _ = self.validate_expr(right, env, validator);
                Ok(())
            }
            Expr::LocalMethodCall(fn_name, params) => {
                let class_name = &env.get_local_env().class_name.clone();

                self.validate_fn_call(&expr_node.line_info, class_name, fn_name, validator)?;
                let fn_def = self.get_function(class_name, fn_name).unwrap();

                for expr in params {
                    let _ = self.validate_expr(expr, env, validator);
                }

                if class_name == main_hash() {
                    let _ =
                        self.validate_fn_definition(class_name, fn_name, fn_def, env, validator);
                }

                if !fn_def.returns {
                    expr_node.compile_error(
                        ErrorType::NoReturn,
                        format!(
                            "not all code paths return for function `{}` in class `{}`",
                            fn_name, class_name
                        )
                        .as_str(),
                        "expected to return a value",
                        validator,
                    )?
                }
                Ok(())
            }
            Expr::SubstringCall { expr, start, end } => {
                let _ = self.validate_expr(expr, env, validator);
                let _ = self.validate_expr(start, env, validator);
                let _ = self.validate_expr(end, env, validator);
                Ok(())
            }
            Expr::LengthCall(expr) => {
                let _ = self.validate_expr(expr, env, validator);
                Ok(())
            }
            Expr::ClassNew(class_name_hash, params) => {
                let _ = self.validate_class_call(&expr_node.line_info, class_name_hash, validator);
                for expr in params {
                    let _ = self.validate_expr(expr, env, validator);
                }
                Ok(())
            }
            Expr::ClassMethodCall {
                expr,
                fn_name: _fn_name,
                params,
            } => {
                let _ = self.validate_expr(expr, env, validator);

                for param in params {
                    let _ = self.validate_expr(param, env, validator);
                }
                Ok(())
            }
            Expr::Index(left, index) => {
                let _ = self.validate_expr(left, env, validator);
                let _ = self.validate_expr(index, env, validator);
                Ok(())
            }
            Expr::Input(text) => {
                let _ = self.validate_expr(text, env, validator);
                Ok(())
            }
            Expr::Div(left, right) => {
                let _ = self.validate_expr(left, env, validator);
                let _ = self.validate_expr(right, env, validator);
                Ok(())
            }
        }
    }
}
