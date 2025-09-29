use crate::ast::validator::Validator;
use crate::ast::AST;
use crate::data::ast_nodes::{AssignTarget, Stmt, StmtNode};
use crate::data::diagnostic::{Diagnostic, ErrorType};
use crate::data::Value;
use crate::env::Env;

impl AST {
    fn valid_assign_stmt(&self, target: &AssignTarget, env: &mut Env, validator: &mut Validator) -> Result<(), Diagnostic> {
        match target {
            AssignTarget::Ident(name_hash) => {
                env.assign(name_hash, Value::Number(0.0));
                Ok(())
            }
            AssignTarget::Array(array_expr, index_expr) => {
                let _ = self.validate_expr(array_expr, env, validator);
                let _ = self.validate_expr(index_expr, env, validator);
                Ok(())
            }
        }
    }

    fn validate_body(&self, body: &Vec<StmtNode>, env: &mut Env, validator: &mut Validator) {
        env.push_scope();
        for stmt_node in body {
            let _ = self.validate_stmt(stmt_node, env, validator);
        }
        env.pop_scope();
    }

    pub fn validate_stmt(&self, stmt_node: &StmtNode, env: &mut Env, validator: &mut Validator) -> Result<(), Diagnostic> {
        match &stmt_node.stmt {
            Stmt::Assign(target, _, expr) => {
                let _ = self.valid_assign_stmt(target, env, validator);
                let _ = self.validate_expr(expr, env, validator);
                Ok(())
            }
            Stmt::Increment(target) => {
                let _ = self.valid_assign_stmt(target, env, validator);
                Ok(())
            }
            Stmt::Decrement(target) => {
                let _ = self.valid_assign_stmt(target, env, validator);
                Ok(())
            }
            Stmt::If { cond, then_branch, elifs, else_branch }  => {
                let _ = self.validate_expr(cond, env, validator);

                self.validate_body(then_branch, env, validator);

                for (cond, stmt_nodes) in elifs {
                    let _ = self.validate_expr(cond, env, validator);

                    self.validate_body(stmt_nodes, env, validator);
                }
                if let Some(else_branch) = else_branch {
                    self.validate_body(else_branch, env, validator);
                }
                Ok(())
            }
            Stmt::While(cond, body) => {
                let _ = self.validate_expr(cond, env, validator);

                self.validate_body(body, env, validator);
                Ok(())
            }
            Stmt::For(name_hash, start_num, end_num, body) => {
                env.assign(name_hash, Value::Number(0.0));
                let _ = self.validate_expr(start_num, env, validator);
                let _ = self.validate_expr(end_num, env, validator);

                self.validate_body(body, env, validator);
                Ok(())
            }
            Stmt::Until(expr, body) => {
                let _ = self.validate_expr(expr, env, validator);

                self.validate_body(body, env, validator);
                Ok(())
            }
            Stmt::Input(name_hash) => {
                env.assign(name_hash, Value::Number(0.0));
                Ok(())
            }
            Stmt::Output(body) => {
                for expr_node in body {
                    let _ = self.validate_expr(expr_node, env, validator);
                }
                Ok(())
            }
            Stmt::Assert(expr, expected) => {
                let _ = self.validate_expr(expr, env, validator);
                let _ = self.validate_expr(expected, env, validator);
                Ok(())
            }
            Stmt::Expr(expr_node) => {
                match self.validate_expr(expr_node, env, validator) {
                    Err(e) => {
                        match e.error_type {
                            ErrorType::NoReturn =>  {
                                validator.errors.pop();
                                Ok(())
                            },
                            _ => {
                                Ok(())
                            },
                        }
                    },
                    Ok(_) => Ok(()),
                }
            }
            Stmt::MethodReturn(expr) => {
                let _ = self.validate_expr(expr, env, validator);
                Ok(())
            },
            Stmt::FunctionDeclaration(_) => Ok(()),
            Stmt::ClassDeclaration(_) => Ok(()),
            Stmt::EOI => Ok(()),
        }
    }
}