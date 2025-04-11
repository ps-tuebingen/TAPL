use crate::{
    syntax::{ClassName, ClassTable, Var},
    to_err,
};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    Typecheck,
};
use std::{collections::HashMap, fmt};

pub mod class;
pub mod methods;
pub mod subtyping;
pub mod terms;
pub use subtyping::is_subtype;

pub fn to_check_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Check)
}

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
