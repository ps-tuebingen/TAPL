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

#[derive(Default)]
pub struct ClassTable {
    pub classes: HashMap<ClassName, ClassDeclaration>,
}

impl fmt::Display for ClassTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut classes: Vec<&ClassDeclaration> = self.classes.values().collect();
        classes.sort_by(|decl1, decl2| decl1.name.cmp(&decl2.name));
        for decl in classes {
            writeln!(f, "{decl}")?;
        }
        Ok(())
    }
}
