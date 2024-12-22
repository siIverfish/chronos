use crate::ast::{*, AST::*, TAbstract::*, TData::*};
use std::sync::{Arc, OnceLock};

use std::collections::HashMap;


fn chronos_inner_print(data: TData) -> TData {
    // TODO: replace TData with TwoData in inner functions
    let TwoData(box d, _) = data else {panic!()};

    if let String(s) = d {
        println!("{s}");
    } else {
        println!("{d:?}");
    }
    return TData::Nil;
}

fn chronos_inner_add(d: TData) -> TData {
    match d {
        TwoData(box Number(a), box TwoData(box Number(b), box Nil)) => Number(a + b),
        _ => {
            println!("{d:#?}");
            panic!()
        }, // TODO: make more stable
    }
}

fn chronos_inner_lambda(e: &'static Environment, d: AST) -> AST {
    let Abstract(TwoAST(box Abstract(Name(parameter)), box Abstract(TwoAST(box expr, _)))) = d
        else { panic!("could not parse arguments to lambda") };
    
    let function: TFunction = TFunction(Arc::new(move |data| {
        let parameter = parameter.clone();
        let expr = expr.clone();
        let TwoData(box data, _) = data else { panic!("expect 1 argument"); };
        let data = HashMap::from([(parameter, data)]);
        // TODO: this is very bad.
        let env = Box::leak(Box::new(Environment::from_parent_and_data(e, data)));
        env.eval(expr)
    }));

    Data(Function(function))
}

#[derive(Clone, Debug)]
pub struct Environment {
    parent: Option<&'static Environment>,
    data: HashMap<std::string::String, TData>,
    //children: Vec<Environment>,
}

// TODO: is this static variable necessary?
// or can one global env be kept track of
static GLOBAL_ENVIRONMENT: OnceLock<Environment> = OnceLock::new();

impl Environment {
    pub fn global() -> &'static Self {
        GLOBAL_ENVIRONMENT.get_or_init(|| 
            Environment {
                parent: None,
                data: HashMap::from([
                    ("print" .into(), Function(TFunction(Arc::new(chronos_inner_print)))),
                    ("+"     .into(), Function(TFunction(Arc::new(chronos_inner_add)))),
                    ("lambda".into(), Macro(   TMacro   (Arc::new(chronos_inner_lambda)))),
                ])
            }
        )
    }

    pub fn from_parent_and_data(parent: &'static Environment, data: HashMap<std::string::String, TData>) -> Environment {
        Environment {
            parent: Some(parent),
            data
        }
    }

    fn resolve(&self, name: &str) -> Option<TData> {
        if let Some(data) = self.data.get(name) {
            Some(data.clone())
        } else if let Some(parent) = self.parent {
            parent.resolve(name)
        } else {
            None
        }
    }

    pub fn eval(&'static self, ast: AST) -> TData {
        match ast {
            Abstract(Application {box f, box arg }) => {
                let func = self.eval(f);
                if let Function(f) = func {
                    let arg = self.eval(arg);
                    (*f.0)(arg)
                } else if let Macro(f) = func {
                    let result = (*f.0)(self, arg);
                    self.eval(result)
                } else {
                    unimplemented!() // TODO better errors
                }
            },
            Abstract(TwoAST(box a, box b)) => TwoData(
                Box::new(self.eval(a)), 
                Box::new(self.eval(b))
            ),
            Abstract(Name(ref name)) => self.resolve(name).expect("all names must be defined"),
            Data(data) => data,
        }
    }
}

pub fn interpret(ast: AST) {
    // println!("{ast:#?}");
    Environment::global().eval(ast);
}
