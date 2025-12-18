use super::SystemF;
use macros::{
    FreeTypeVars, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, NoKinds, NoNorm,
    NoSubtypes, Spanned, SubstType,
};
use syntax::types::{Forall, Fun, Type as TypeTrait, TypeGroup, TypeVariable};

#[derive(
    FreeTypeVars,
    Spanned,
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
    fn into_fun(self) -> Option<Fun<Self::Lang>> {
        if let Self::Fun(fun) = self {
            Some(fun)
        } else {
            None
        }
    }

    fn into_forall(self) -> Option<Forall<Self::Lang>> {
        if let Self::Forall(forall) = self {
            Some(forall)
        } else {
            None
        }
    }
}
