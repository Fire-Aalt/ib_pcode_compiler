use crate::ast::{AST, main_hash};
use crate::compiler::errors::{
    compile_error, diagnostic, invalid_number_of_params_error, no_return_error,
};
use crate::data::ast_nodes::{Expr, ExprNode, Function};
use crate::data::diagnostic::ErrorType;
use crate::data::{NameHash, Validator};
use crate::env::Env;

impl AST {
    pub fn validate_expr(&self, expr_node: &ExprNode, env: &mut Env, validator: &mut Validator) {
        match &expr_node.expr {
            Expr::Ident(name) => {
                let _ = env.get(name).ok_or_else(|| {
                    compile_error(
                        diagnostic(
                            &expr_node.line_info,
                            ErrorType::Uninitialized,
                            format!("cannot find variable `{}` in this scope", name),
                            "not found in this scope",
                        ),
                        validator,
                    )
                });
            }
            Expr::Data(_) => {}
            Expr::Array(data) => {
                for expr in data {
                    self.validate_expr(expr, env, validator);
                }
            }
            Expr::Unary(_, expr) => {
                self.validate_expr(expr, env, validator);
            }
            Expr::BinOp(left, _, right) => {
                self.validate_expr(left, env, validator);
                self.validate_expr(right, env, validator);
            }
            Expr::LocalMethodCall(fn_name, params) => {
                let class_name = &env.get_local_env().class_name.clone();

                let Some(fn_def) =
                    self.validate_fn_get(&expr_node.line_info, class_name, fn_name, validator)
                else {
                    return;
                };

                if class_name == main_hash() {
                    self.validate_fn_definition(class_name, fn_name, fn_def, env, validator);
                }

                self.validate_fn_call(
                    expr_node, class_name, fn_name, fn_def, params, env, validator,
                );
            }
            Expr::StaticMethodCall(class_name, fn_name, params) => {
                self.validate_class_get(&expr_node.line_info, class_name, validator);
                if !self.statics.contains(class_name) {
                    compile_error(
                        diagnostic(
                            &expr_node.line_info,
                            ErrorType::InvalidType,
                            format!(
                                "tried to call function `{}` on a non static class `{}`",
                                fn_name, class_name
                            ),
                            "cannot call a function",
                        ),
                        validator,
                    );
                }

                let Some(fn_def) =
                    self.validate_fn_get(&expr_node.line_info, class_name, fn_name, validator)
                else {
                    return;
                };

                self.validate_fn_call(
                    expr_node, class_name, fn_name, fn_def, params, env, validator,
                );
            }
            Expr::ClassMethodCall {
                expr,
                fn_name: _fn_name,
                params,
            } => {
                self.validate_expr(expr, env, validator);

                for param in params {
                    self.validate_expr(param, env, validator);
                }
            }
            Expr::SubstringCall { expr, start, end } => {
                self.validate_expr(expr, env, validator);
                self.validate_expr(start, env, validator);
                self.validate_expr(end, env, validator);
            }
            Expr::LengthCall(expr) => {
                self.validate_expr(expr, env, validator);
            }
            Expr::ClassNew(class_name_hash, params) => {
                self.validate_class_get(&expr_node.line_info, class_name_hash, validator);
                for expr in params {
                    self.validate_expr(expr, env, validator);
                }
            }
            Expr::Index(left, index) => {
                self.validate_expr(left, env, validator);
                self.validate_expr(index, env, validator);
            }
            Expr::Input(text) => {
                self.validate_expr(text, env, validator);
            }
            Expr::Div(left, right) => {
                self.validate_expr(left, env, validator);
                self.validate_expr(right, env, validator);
            }
            Expr::MathRandom => {}
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_fn_call(
        &self,
        expr_node: &ExprNode,
        class_name: &NameHash,
        fn_name: &NameHash,
        fn_def: &Function,
        params: &Vec<ExprNode>,
        env: &mut Env,
        validator: &mut Validator,
    ) {
        if params.len() != fn_def.args.len() {
            compile_error(
                invalid_number_of_params_error(
                    &expr_node.line_info,
                    params.len(),
                    fn_def.args.len(),
                ),
                validator,
            );
        }

        for expr in params {
            self.validate_expr(expr, env, validator);
        }

        if !fn_def.returns {
            compile_error(
                no_return_error(&expr_node.line_info, fn_name, class_name),
                validator,
            );
        }
    }
}
