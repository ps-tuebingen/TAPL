use super::{ConstructorDeclaration, MethodDeclaration};
use std::fmt;

pub type ClassName = String;
pub type FieldName = String;

#[derive(Debug, Clone)]
pub struct ClassDeclaration {
    pub name: ClassName,
    pub parent: ClassName,
    pub fields: Vec<(ClassName, FieldName)>,
    pub constructor: ConstructorDeclaration,
    pub methods: Vec<MethodDeclaration>,
}

impl ClassDeclaration {
    pub fn object() -> ClassDeclaration {
        ClassDeclaration {
            name: "Object".to_owned(),
            parent: "".to_owned(),
            fields: vec![],
            constructor: ConstructorDeclaration {
                name: "Object".to_owned(),
                self_args: vec![],
                super_args: vec![],
                self_fields: vec![],
            },
            methods: vec![],
        }
    }
}

impl Default for ClassDeclaration {
    fn default() -> ClassDeclaration {
        ClassDeclaration::object()
    }
}

impl fmt::Display for ClassDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let field_strs: Vec<String> = self
            .fields
            .iter()
            .map(|(class, field)| format!("{class} {field};"))
            .collect();
        let method_strs: Vec<String> = self
            .methods
            .iter()
            .map(|method| format!("{method}"))
            .collect();
        write!(
            f,
            "class {} extends {} {{\n\t{}\n\n\t{}\n\t{}\n}}",
            self.name,
            self.parent,
            field_strs.join("\n\t"),
            self.constructor,
            method_strs.join("\n\t")
        )
    }
}
