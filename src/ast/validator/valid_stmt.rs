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
                self.valid_expr(array_expr, env, validator)?;
                self.valid_expr(index_expr, env, validator)?;
                Ok(())
            }
        }
    }

    pub fn valid_stmt(&self, stmt_node: &StmtNode, env: &mut Env, validator: &mut Validator) -> Result<(), Diagnostic> {
        match &stmt_node.stmt {
            Stmt::Assign(target, _, expr) => {
                self.valid_assign_stmt(target, env, validator)?;
                self.valid_expr(expr, env, validator)?;
                Ok(())
            }
            Stmt::Increment(target) => {
                self.valid_assign_stmt(target, env, validator)?;
                Ok(())
            }
            Stmt::Decrement(target) => {
                self.valid_assign_stmt(target, env, validator)?;
                Ok(())
            }
            Stmt::If { cond, then_branch, elifs, else_branch }  => {
                self.valid_expr(cond, env, validator)?;

                for stmt_node in then_branch {
                    self.valid_stmt(stmt_node, env, validator)?;
                }
                for (cond, stmt_nodes) in elifs {
                    self.valid_expr(cond, env, validator)?;
                    for stmt_node in stmt_nodes {
                        self.valid_stmt(stmt_node, env, validator)?;
                    }
                }
                if let Some(else_branch) = else_branch {
                    for stmt_node in else_branch {
                        self.valid_stmt(stmt_node, env, validator)?;
                    }
                }
                Ok(())
            }
            Stmt::While(cond, body) => {
                self.valid_expr(cond, env, validator)?;

                for stmt_node in body {
                    self.valid_stmt(stmt_node, env, validator)?;
                }
                Ok(())
            }
            Stmt::For(name_hash, start_num, end_num, body) => {
                env.assign(name_hash, Value::Number(0.0));
                self.valid_expr(start_num, env, validator)?;
                self.valid_expr(end_num, env, validator)?;

                for stmt_node in body {
                    self.valid_stmt(stmt_node, env, validator)?;
                }
                Ok(())
            }
            Stmt::Until(expr, body) => {
                self.valid_expr(expr, env, validator)?;

                for stmt_node in body {
                    self.valid_stmt(stmt_node, env, validator)?;
                }
                Ok(())
            }
            Stmt::Input(name_hash) => {
                env.assign(name_hash, Value::Number(0.0));
                Ok(())
            }
            Stmt::Output(body) => {
                for expr_node in body {
                    self.valid_expr(expr_node, env, validator)?;
                }
                Ok(())
            }
            Stmt::Assert(expr, expected) => {
                self.valid_expr(expr, env, validator)?;
                self.valid_expr(expected, env, validator)?;
                Ok(())
            }
            Stmt::Expr(expr_node) => {
                match self.valid_expr(expr_node, env, validator) {
                    Err(e) => {
                        match e.error_type {
                            ErrorType::NoReturn => Ok(()),
                            _ => Err(e),
                        }
                    },
                    Ok(_) => Ok(()),
                }
            }
            Stmt::FunctionDeclaration(_) => Ok(()),
            Stmt::ClassDeclaration(_) => Ok(()),
            Stmt::MethodReturn(_) => Ok(()),
            Stmt::EOI => Ok(()),
        }
    }
}