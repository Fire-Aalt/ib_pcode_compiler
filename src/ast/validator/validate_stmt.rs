use crate::ast::AST;
use crate::ast::validator::Validator;
use crate::data::Value;
use crate::data::ast_nodes::{AssignTarget, Stmt, StmtNode};
use crate::data::diagnostic::ErrorType;
use crate::env::Env;

impl AST {
    pub fn validate_stmt(&self, stmt_node: &StmtNode, env: &mut Env, validator: &mut Validator) {
        match &stmt_node.stmt {
            Stmt::Assign(target, _, expr) => {
                self.valid_assign_stmt(target, env, validator);
                self.validate_expr(expr, env, validator);
            }
            Stmt::Increment(target) => {
                self.valid_assign_stmt(target, env, validator);
            }
            Stmt::Decrement(target) => {
                self.valid_assign_stmt(target, env, validator);
            }
            Stmt::If {
                cond,
                then_branch,
                elifs,
                else_branch,
            } => {
                self.validate_expr(cond, env, validator);
                self.validate_body(then_branch, env, validator);

                for (cond, stmt_nodes) in elifs {
                    let _ = self.validate_expr(cond, env, validator);

                    self.validate_body(stmt_nodes, env, validator);
                }
                if let Some(else_branch) = else_branch {
                    self.validate_body(else_branch, env, validator);
                }
            }
            Stmt::While(cond, body) => {
                self.validate_expr(cond, env, validator);
                self.validate_body(body, env, validator);
            }
            Stmt::For(name_hash, start_num, end_num, body) => {
                let previous_value = env.get(name_hash); // Save previous state
                env.assign(name_hash, Value::Number(0.0));  // Override control variable
                
                self.validate_expr(start_num, env, validator);
                self.validate_expr(end_num, env, validator);
                self.validate_body(body, env, validator);
                
                match previous_value {
                    None => env.undefine(name_hash), // Remove control variable
                    Some(val) => env.assign(name_hash, val) // Restore previous state
                }
            }
            Stmt::Until(expr, body) => {
                self.validate_expr(expr, env, validator);
                self.validate_body(body, env, validator);
            }
            Stmt::Input(name_hash) => {
                env.assign(name_hash, Value::Number(0.0));
            }
            Stmt::Output(body) => {
                for expr_node in body {
                    self.validate_expr(expr_node, env, validator);
                }
            }
            Stmt::Assert(expr, expected) => {
                self.validate_expr(expr, env, validator);
                self.validate_expr(expected, env, validator);
            }
            Stmt::Expr(expr_node) => {
                validator.start_record();
                self.validate_expr(expr_node, env, validator);

                if validator.is_last_recorded_expr_error(ErrorType::NoReturn) {
                    validator.errors.pop();
                }
            }
            Stmt::MethodReturn(expr) => {
                self.validate_expr(expr, env, validator);
            }
            Stmt::FunctionDeclaration(_) => {}
            Stmt::ClassDeclaration(_) => {}
            Stmt::EOI => {}
        }
    }

    fn valid_assign_stmt(&self, target: &AssignTarget, env: &mut Env, validator: &mut Validator) {
        match target {
            AssignTarget::Ident(name_hash) => {
                env.assign(name_hash, Value::Number(0.0));
            }
            AssignTarget::Array(array_expr, index_expr) => {
                self.validate_expr(array_expr, env, validator);
                self.validate_expr(index_expr, env, validator);
            }
        }
    }

    fn validate_body(&self, body: &Vec<StmtNode>, env: &mut Env, validator: &mut Validator) {
        env.push_scope();
        for stmt_node in body {
            self.validate_stmt(stmt_node, env, validator);
        }
        env.pop_scope();
    }
}
