use crate::ast::{AST, MAIN_CLASS};
use crate::compiler::errors::{
    compile_error, diagnostic, invalid_number_of_params_error, no_public_var_error,
    no_return_error, undefined_fn_in_class_error,
};
use crate::data::ast_nodes::{Expr, ExprNode, Function, NativeMethod};
use crate::data::diagnostic::{ErrorType, LineInfo};
use crate::data::{NameHash, Validator};
use crate::env::Env;

impl AST {
    pub fn validate_expr(&self, expr_node: &ExprNode, env: &mut Env, validator: &mut Validator) {
        let line = &expr_node.line_info;
        match &expr_node.expr {
            Expr::Ident(name) => {
                let _ = env.get(name).ok_or_else(|| {
                    compile_error(
                        diagnostic(
                            line,
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

                for expr in params {
                    self.validate_expr(expr, env, validator);
                }

                let Some(fn_def) = self.get_function(class_name, fn_name) else {
                    compile_error(
                        diagnostic(
                            line,
                            ErrorType::Uninitialized,
                            format!("cannot find function `{}` in this scope", fn_name),
                            "not found in this scope",
                        ),
                        validator,
                    );
                    return;
                };

                if class_name == MAIN_CLASS {
                    self.validate_fn_definition(class_name, fn_name, fn_def, env, validator);
                }
                self.validate_fn_call(line, class_name, fn_name, fn_def, params, validator);
            }
            Expr::StaticMethodCall(fn_line, class_name, fn_name, params) => {
                self.validate_class_get(line, class_name, validator);

                for expr in params {
                    self.validate_expr(expr, env, validator);
                }

                let Some(fn_def) = self.get_function(class_name, fn_name) else {
                    compile_error(
                        undefined_fn_in_class_error(fn_line, class_name, fn_name),
                        validator,
                    );
                    return;
                };

                self.validate_fn_call(fn_line, class_name, fn_name, fn_def, params, validator);
            }
            Expr::StaticGetVar(var_line, class_name, var_name) => {
                let Some(class_def) = self.validate_class_get(line, class_name, validator) else {
                    return;
                };

                if !class_def.public_vars.contains(var_name) {
                    compile_error(
                        no_public_var_error(var_line, var_name, class_name),
                        validator,
                    );
                }
            }
            Expr::ClassMethodCall {
                expr,
                fn_line: _,
                fn_name: _,
                params,
            } => {
                self.validate_expr(expr, env, validator);

                for param in params {
                    self.validate_expr(param, env, validator);
                }
            }
            Expr::ClassGetVar(expr, _, _) => {
                self.validate_expr(expr, env, validator);
            }
            Expr::ClassNew(class_name_hash, params) => {
                if self.static_classes.contains(class_name_hash) {
                    compile_error(
                        diagnostic(
                            line,
                            ErrorType::Unsupported,
                            format!(
                                "cannot make an instance of a static class `{}`",
                                class_name_hash
                            ),
                            "expected to be a non static class",
                        ),
                        validator,
                    );
                } else {
                    self.validate_class_get(line, class_name_hash, validator);
                }

                for expr in params {
                    self.validate_expr(expr, env, validator);
                }
            }
            Expr::Index(left, index) => {
                self.validate_expr(left, env, validator);
                self.validate_expr(index, env, validator);
            }
            Expr::NativeMethodCall(native_method, target, fn_line, params) => {
                match native_method {
                    NativeMethod::Div => {
                        Self::valid_number_of_args(
                            fn_line,
                            params,
                            |len| len == 2,
                            &"2",
                            validator,
                        );
                    }
                    NativeMethod::Input => {
                        Self::valid_number_of_args(
                            fn_line,
                            params,
                            |len| len <= 1,
                            &"0 or 1",
                            validator,
                        );
                    }
                    NativeMethod::MathRandom => {
                        Self::valid_number_of_args(
                            fn_line,
                            params,
                            |len| len == 0,
                            &"0",
                            validator,
                        );
                    }
                    NativeMethod::SubstringCall => {
                        Self::valid_number_of_args(
                            fn_line,
                            params,
                            |len| len == 2,
                            &"2",
                            validator,
                        );
                        self.validate_expr(target.as_ref().unwrap(), env, validator);
                    }
                    NativeMethod::LengthCall => {
                        self.validate_expr(target.as_ref().unwrap(), env, validator);
                    }
                }

                for param in params {
                    self.validate_expr(param, env, validator);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_fn_call(
        &self,
        line: &LineInfo,
        class_name: &NameHash,
        fn_name: &NameHash,
        fn_def: &Function,
        params: &[ExprNode],
        validator: &mut Validator,
    ) {
        if params.len() != fn_def.args.len() {
            compile_error(
                invalid_number_of_params_error(line, params.len(), fn_def.args.len().to_string()),
                validator,
            );
        }

        if !fn_def.returns {
            compile_error(no_return_error(line, fn_name, class_name), validator);
        }
    }
}
