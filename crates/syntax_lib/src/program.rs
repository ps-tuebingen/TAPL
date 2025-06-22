use crate::{
    definition::Definition,
    subst::{SubstTerm, SubstType},
    terms::Term,
    types::Type,
    TypeVar, Var,
};
use common::errors::DuplicateDefinition;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Program<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub definitions: Vec<Definition<T, Ty>>,
    pub main: T,
}

impl<T, Ty> Program<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<T1>(main: T1, definitions: Vec<Definition<T, Ty>>) -> Program<T, Ty>
    where
        T1: Into<T>,
    {
        Program {
            definitions,
            main: main.into(),
        }
    }

    pub fn add_definition(&mut self, def: Definition<T, Ty>) -> Result<(), DuplicateDefinition> {
        if self.definitions.iter().any(|df| df.name == def.name) {
            Err(DuplicateDefinition::new(&def.name))
        } else {
            self.definitions.push(def);
            Ok(())
        }
    }
}

impl<T, Ty> SubstTerm<T> for Program<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
{
    type Target = Program<T, Ty>;

    fn subst(self, v: &Var, t: &T) -> Self::Target {
        Program {
            definitions: self
                .definitions
                .into_iter()
                .map(|def| def.subst(v, t))
                .collect(),
            main: self.main.subst(v, t),
        }
    }
}

impl<T, Ty> SubstTerm<T> for Definition<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Ty: Type,
{
    type Target = Definition<T, Ty>;

    fn subst(self, v: &Var, t: &T) -> Self::Target {
        Definition {
            name: self.name,
            annot: self.annot,
            body: self.body.subst(v, t),
        }
    }
}

impl<T, Ty> SubstType<Ty> for Program<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
{
    type Target = Program<T, Ty>;

    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Program {
            definitions: self
                .definitions
                .into_iter()
                .map(|def| def.subst_type(v, ty))
                .collect(),
            main: self.main.subst_type(v, ty),
        }
    }
}

impl<T, Ty> SubstType<Ty> for Definition<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
{
    type Target = Definition<T, Ty>;

    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Definition {
            name: self.name,
            annot: self.annot.subst_type(v, ty),
            body: self.body.subst_type(v, ty),
        }
    }
}

impl<T, Ty> fmt::Display for Program<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n\n def main := {}",
            self.definitions
                .iter()
                .map(|def| def.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
            self.main.to_string()
        )
    }
}
