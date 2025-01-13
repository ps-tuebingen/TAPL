use std::collections::HashMap;

pub mod class;
pub mod constructor;
pub mod methods;
pub mod terms;

pub use class::{ClassDeclaration, ClassName, FieldName};
pub use constructor::ConstructorDeclaration;
pub use methods::{MethodDeclaration, MethodName, MethodType};
pub use terms::Term;

pub type Var = String;
pub type ClassTable = HashMap<ClassName, ClassDeclaration>;
