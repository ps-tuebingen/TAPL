use super::SystemF;
use errors::TypeMismatch;
use macros::{
    FromVariants, GrammarDescribe, LangDisplay, LatexFmt, NoKinds, NoNorm, NoSubtypes, SubstType,
};
use syntax::types::{Forall, Fun, Type as TypeTrait, TypeGroup, TypeVariable};

#[derive(
    GrammarDescribe,
    FromVariants,
    SubstType,
    LatexFmt,
    LangDisplay,
    NoNorm,
    NoKinds,
    NoSubtypes,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(SystemF)]
pub enum Type {
    Var(TypeVariable<SystemF>),
    Fun(Fun<SystemF>),
    Forall(Forall<SystemF>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = SystemF;
    fn into_fun(self) -> Result<Fun<Self::Lang>, TypeMismatch> {
        if let Self::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_forall(self) -> Result<Forall<Self::Lang>, TypeMismatch> {
        if let Self::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
        }
    }
}
