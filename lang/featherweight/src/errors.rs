use super::{
    eval::Value,
    syntax::{ClassName, FieldName, MethodName, Term, Var},
};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVar(Var),
    ClassNotFound(ClassName),
    FieldNotFound {
        class: ClassName,
        field: FieldName,
    },
    MethodNotFound {
        class: ClassName,
        method: MethodName,
    },
    BadValue {
        found: Value,
        expected: String,
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
    NameMismatch {
        found: String,
        expected: String,
        name: String,
    },
    BadOverride {
        class: ClassName,
        sup: ClassName,
        method: MethodName,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ClassNotFound(class) => write!(f, "Could not find {class} in class table"),
            Error::FreeVar(v) => write!(f, "Variable {v} cannot appear free"),
            Error::StuckTerm(t) => write!(f,"Term {t} cannot be evaluated further"),
            Error::MethodNotFound{class,method}=> write!(f,"Could not find method {method} in class {class}"),
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
            Error::BadValue{found,expected} => write!(f,"Unexpected value {found}, expected {expected}"),
            Error::NameMismatch{found,expected,name} => write!(f,"Name Mismatch for {name}: expected {expected}, found {found}"),
                Error::BadOverride{class,sup,method} => write!(f,"Bad Override for method {method} from {sup} in class {class}")
        }
    }
}

impl std::error::Error for Error {}
