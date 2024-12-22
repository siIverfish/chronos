use std::{fmt::Debug, sync::Arc};

use crate::interpreter::Environment;



// type TFunction = Box<dyn Fn(TData) -> TData>;
// type TMacro = Box<dyn Fn(Environment, AST) -> AST>;

#[derive(Debug, Clone)]
pub enum TData {
    String(String),
    Number(f64),
    Function(TFunction),
    Macro(TMacro),
    TwoData(Box<TData>, Box<TData>),
    Lazy(Box<AST>),
    Nil,
}

// TODO: can these two be unified?
#[derive(Clone)]
pub struct TFunction(pub Arc<dyn Fn(TData) -> TData + Sync + Send>);
impl Debug for TFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TMacro@{:p}>", Arc::as_ptr(&self.0))
    }
}

#[derive(Clone)]
pub struct TMacro(pub Arc<dyn Fn(&'static Environment, AST) -> AST + Sync + Send>);
impl Debug for TMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<TMacro@{:p}>", Arc::as_ptr(&self.0))
    }
}


#[derive(Debug, Clone)]
pub enum TAbstract {
    Name(String),
    Application {f: Box<AST>, arg: Box<AST>},
    TwoAST(Box<AST>, Box<AST>),
}

#[derive(Debug, Clone)]
pub enum AST {
    Data(TData),
    Abstract(TAbstract),
}