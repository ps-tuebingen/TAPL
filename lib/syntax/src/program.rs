use crate::{
    TypeVar, Var,
    definition::Definition,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use errors::DuplicateDefinition;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Program<Lang>
where
    Lang: Language,
{
    pub definitions: Vec<Definition<Lang>>,
    pub main: Lang::Term,
}

impl<Lang> Program<Lang>
where
    Lang: Language,
{
    pub fn new<T>(main: T, definitions: Vec<Definition<Lang>>) -> Program<Lang>
    where
        T: Into<<Lang as Language>::Term>,
    {
        Program {
            definitions,
            main: main.into(),
        }
    }

    pub fn add_definition(&mut self, def: Definition<Lang>) -> Result<(), DuplicateDefinition> {
        if self.definitions.iter().any(|df| df.name == def.name) {
            Err(DuplicateDefinition::new(&def.name))
        } else {
            self.definitions.push(def);
            Ok(())
        }
    }
}

impl<Lang> SubstTerm for Program<Lang>
where
    Lang: Language,
{
    type Target = Program<Lang>;
    type Lang = Lang;

    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
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

impl<Lang> SubstTerm for Definition<Lang>
where
    Lang: Language,
{
    type Target = Definition<Lang>;
    type Lang = Lang;

    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Definition {
            name: self.name,
            annot: self.annot,
            body: self.body.subst(v, t),
        }
    }
}

impl<Lang> SubstType for Program<Lang>
where
    Lang: Language,
{
    type Target = Program<Lang>;
    type Lang = Lang;

    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
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

impl<Lang> SubstType for Definition<Lang>
where
    Lang: Language,
{
    type Target = Definition<Lang>;
    type Lang = Lang;

    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Definition {
            name: self.name,
            annot: self.annot.subst_type(v, ty),
            body: self.body.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Program<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n\n def main := {};",
            self.definitions
                .iter()
                .map(|def| def.to_string())
                .collect::<Vec<String>>()
                .join("\n\n"),
            self.main
        )
    }
}
