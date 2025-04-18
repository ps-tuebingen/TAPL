use super::{ClassName, Term, Var};
use std::fmt;

pub type MethodName = String;

#[derive(Debug, Clone)]
pub struct MethodDeclaration {
    pub class: ClassName,
    pub name: MethodName,
    pub args: Vec<(ClassName, Var)>,
    pub ret: Term,
}

impl MethodDeclaration {
    pub fn get_type(&self) -> MethodType {
        MethodType {
            args: self.args.iter().map(|arg| arg.0.clone()).collect(),
            ret: self.class.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MethodType {
    pub ret: ClassName,
    pub args: Vec<ClassName>,
}

impl fmt::Display for MethodDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let args_strs: Vec<String> = self
            .args
            .iter()
            .map(|(class, var)| format!("{class} {var}"))
            .collect();
        write!(
            f,
            "{} {}({}) {{\n\t return {};\n}}",
            self.class,
            self.name,
            args_strs.join(", "),
            self.ret
        )
    }
}
