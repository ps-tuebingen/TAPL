use super::syntax::{ClassName, FieldName, MethodName, Term, Var};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVar(Var),
    ClassNotFound(ClassName),
    FieldNotFound {
        class: ClassName,
        field: FieldName,
    },
    StuckTerm(Term),
    NotASubClass {
        sub: ClassName,
        sup: ClassName,
    },
    ConstructorArity {
        class: ClassName,
        found: usize,
        expected: usize,
    },
    MethodArity {
        class: ClassName,
        method: MethodName,
        found: usize,
        expected: usize,
    },
    UnexpectedTerm {
        found: Term,
        expected: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ClassNotFound(class) => write!(f, "Could not find {class} in class table"),
            Error::FreeVar(v) => write!(f, "Variable {v} cannot appear free"),
            Error::StuckTerm(t) => write!(f,"Term {t} cannot be evaluated further"),
            Error::FieldNotFound{class, field} => {
                write!(f, "Class {class} does not have field {field}")
            }
            Error::UnexpectedTerm { found, expected } => {
                write!(f, "Unexpected term: {found}, expected: {expected}")
            }
            Error::ConstructorArity {
                class,
                found,
                expected,
            } => write!(f, "Wrong number of arguments for constructor {class}, found {found}, expected {expected}"),
            Error::MethodArity{class,method,found,expected} => write!(f,"Wrong number of arguments for method {method} of class {class}: found:{found}, expected:{expected}"),
            Error::NotASubClass{sub,sup} => write!(f,"{sub} is not a subclass of {sup}"),
        }
    }
}

impl std::error::Error for Error {}
