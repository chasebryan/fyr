use std::path::{Path, PathBuf};

use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let {
        name: String,
        ty: TypeName,
        value: Expr,
        span: Span,
        source_path: Option<PathBuf>,
    },
    Var {
        name: String,
        ty: TypeName,
        value: Expr,
        span: Span,
        source_path: Option<PathBuf>,
    },
    Assign {
        name: String,
        value: Expr,
        span: Span,
        source_path: Option<PathBuf>,
    },
    Import {
        path: String,
        span: Span,
        source_path: Option<PathBuf>,
    },
    Struct {
        name: String,
        fields: Vec<Param>,
        span: Span,
        source_path: Option<PathBuf>,
    },
    Fn {
        name: String,
        params: Vec<Param>,
        return_type: TypeName,
        body: Vec<Statement>,
        span: Span,
        source_path: Option<PathBuf>,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
        span: Span,
        source_path: Option<PathBuf>,
    },
    For {
        name: String,
        iterable: Expr,
        body: Vec<Statement>,
        span: Span,
        source_path: Option<PathBuf>,
    },
    If {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_branch: Vec<Statement>,
        span: Span,
        source_path: Option<PathBuf>,
    },
    Return {
        value: Option<Expr>,
        span: Span,
        source_path: Option<PathBuf>,
    },
    Break {
        span: Span,
        source_path: Option<PathBuf>,
    },
    Continue {
        span: Span,
        source_path: Option<PathBuf>,
    },
    Expr {
        expr: Expr,
        span: Span,
        source_path: Option<PathBuf>,
    },
}

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Statement::Let { span, .. }
            | Statement::Var { span, .. }
            | Statement::Assign { span, .. }
            | Statement::Import { span, .. }
            | Statement::Struct { span, .. }
            | Statement::Fn { span, .. }
            | Statement::While { span, .. }
            | Statement::For { span, .. }
            | Statement::If { span, .. }
            | Statement::Return { span, .. }
            | Statement::Break { span, .. }
            | Statement::Continue { span, .. }
            | Statement::Expr { span, .. } => *span,
        }
    }

    pub fn source_path(&self) -> Option<&Path> {
        match self {
            Statement::Let { source_path, .. }
            | Statement::Var { source_path, .. }
            | Statement::Assign { source_path, .. }
            | Statement::Import { source_path, .. }
            | Statement::Struct { source_path, .. }
            | Statement::Fn { source_path, .. }
            | Statement::While { source_path, .. }
            | Statement::For { source_path, .. }
            | Statement::If { source_path, .. }
            | Statement::Return { source_path, .. }
            | Statement::Break { source_path, .. }
            | Statement::Continue { source_path, .. }
            | Statement::Expr { source_path, .. } => source_path.as_deref(),
        }
    }

    pub fn set_source_path_recursive(&mut self, path: &Path) {
        let origin = Some(path.to_path_buf());
        match self {
            Statement::Let {
                source_path: target,
                ..
            }
            | Statement::Var {
                source_path: target,
                ..
            }
            | Statement::Assign {
                source_path: target,
                ..
            }
            | Statement::Import {
                source_path: target,
                ..
            }
            | Statement::Struct {
                source_path: target,
                ..
            }
            | Statement::Return {
                source_path: target,
                ..
            }
            | Statement::Break {
                source_path: target,
                ..
            }
            | Statement::Continue {
                source_path: target,
                ..
            }
            | Statement::Expr {
                source_path: target,
                ..
            } => {
                *target = origin;
            }
            Statement::Fn {
                body, source_path, ..
            }
            | Statement::While {
                body, source_path, ..
            }
            | Statement::For {
                body, source_path, ..
            } => {
                *source_path = origin;
                for statement in body {
                    statement.set_source_path_recursive(path);
                }
            }
            Statement::If {
                then_branch,
                else_branch,
                source_path,
                ..
            } => {
                *source_path = origin;
                for statement in then_branch {
                    statement.set_source_path_recursive(path);
                }
                for statement in else_branch {
                    statement.set_source_path_recursive(path);
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub ty: TypeName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeName {
    Infer,
    I64,
    Bool,
    Str,
    Unit,
    Struct(String),
    Array(Box<TypeName>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    Str(String),
    Variable(String),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
    },
    StructInit {
        name: String,
        fields: Vec<(String, Expr)>,
    },
    Field {
        object: Box<Expr>,
        field: String,
    },
    Array(Vec<Expr>),
    Index {
        collection: Box<Expr>,
        index: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Vec<Statement>,
        else_branch: Vec<Statement>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}
