use super::Value;
use crate::{language::Language, terms::Raise as RaiseT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Raise<Lang>
where
    Lang: Language,
{
    pub val: Box<Lang::Value>,
    pub cont_ty: Lang::Type,
    pub exception_ty: Lang::Type,
}

impl<Lang> Raise<Lang>
where
    Lang: Language,
{
    pub fn new<V1, Ty1, Ty2>(v: V1, cont_ty: Ty1, ex_ty: Ty2) -> Raise<Lang>
    where
        V1: Into<Lang::Value>,
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Raise {
            val: Box::new(v.into()),
            cont_ty: cont_ty.into(),
            exception_ty: ex_ty.into(),
        }
    }
}

impl<Lang> Value for Raise<Lang>
where
    Lang: Language,
    RaiseT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = RaiseT<Lang>;
}

impl<Lang> From<Raise<Lang>> for RaiseT<Lang>
where
    Lang: Language,
{
    fn from(r: Raise<Lang>) -> RaiseT<Lang> {
        RaiseT::new(*r.val, r.exception_ty, r.cont_ty)
    }
}

impl<Lang> fmt::Display for Raise<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "raise[{}]({} : {})",
            self.cont_ty, self.val, self.exception_ty
        )
    }
}
