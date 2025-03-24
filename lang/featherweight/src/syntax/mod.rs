use std::{collections::HashMap, fmt};

pub mod class;
pub mod constructor;
pub mod methods;
pub mod terms;

pub use class::{ClassDeclaration, ClassName, FieldName};
pub use constructor::ConstructorDeclaration;
pub use methods::{MethodDeclaration, MethodName, MethodType};
pub use terms::Term;

pub type Var = String;
pub struct ClassTable {
    pub classes: HashMap<ClassName, ClassDeclaration>,
}

impl fmt::Display for ClassTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_, decl) in self.classes.iter() {
            writeln!(f, "{decl}")?;
        }
        Ok(())
    }
}
