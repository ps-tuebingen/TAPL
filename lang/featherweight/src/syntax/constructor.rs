use super::{ClassName, FieldName};
use std::fmt;

#[derive(Debug, Clone)]
pub struct ConstructorDeclaration {
    pub name: ClassName,
    pub super_args: Vec<FieldName>,
    pub self_args: Vec<(ClassName, FieldName)>,
    pub self_fields: Vec<FieldName>,
}

impl ConstructorDeclaration {
    pub fn num_args(&self) -> usize {
        self.super_args.len() + self.self_args.len()
    }
}

impl fmt::Display for ConstructorDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let self_strs: Vec<String> = self
            .self_args
            .iter()
            .map(|(cl, arg)| format!("{cl} {arg}"))
            .collect();
        let super_ctor: Vec<String> = self.super_args.iter().map(|arg| arg.to_string()).collect();
        let self_assign: Vec<String> = self
            .self_args
            .iter()
            .zip(self.self_fields.iter())
            .map(|((_, arg_name), arg_value)| format!("this.{arg_name} = {arg_value};"))
            .collect();
        write!(
            f,
            "{}({}) {{\n\t super({});\n\t{}\n}}",
            self.name,
            self_strs.join(", "),
            super_ctor.join(", "),
            self_assign.join("\n\t")
        )
    }
}
