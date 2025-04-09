use crate::{
    errors::Error,
    syntax::{ClassName, ClassTable, Var},
};
use common::Typecheck;
use std::{collections::HashMap, fmt};

pub mod class;
pub mod methods;
pub mod subtyping;
pub mod terms;
pub use subtyping::is_subtype;

pub type Env = HashMap<Var, ClassName>;

pub struct Checked;

impl Typecheck<'_> for ClassTable {
    type Type = Checked;
    type Err = Error;
    type Env = ();
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(())
    }

    fn check(&self, _: Self::Env) -> Result<Self::Type, Self::Err> {
        for (_, decl) in self.classes.iter() {
            decl.check(self)?;
        }
        Ok(Checked)
    }
}

impl fmt::Display for Checked {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("OK")
    }
}
