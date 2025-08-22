use super::{Top, Type};
use crate::{TypeVar, kinds::Kind, language::Language, subst::SubstType};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpLambdaSub<Lang>
where
    Lang: Language,
{
    pub var: TypeVar,
    pub sup: Box<Lang::Type>,
    pub body: Box<Lang::Type>,
}

impl<Lang> OpLambdaSub<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(var: &str, sup: Ty1, ty: Ty2) -> OpLambdaSub<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        OpLambdaSub {
            var: var.to_owned(),
            sup: Box::new(sup.into()),
            body: Box::new(ty.into()),
        }
    }

    pub fn new_unbounded<Ty1>(var: &str, knd: Kind, ty: Ty1) -> OpLambdaSub<Lang>
    where
        Ty1: Into<Lang::Type>,
        Top<Lang>: Into<Lang::Type>,
    {
        OpLambdaSub {
            var: var.to_owned(),
            sup: Box::new(Top::new(knd).into()),
            body: Box::new(ty.into()),
        }
    }
}

impl<Lang> Type for OpLambdaSub<Lang> where Lang: Language {}

impl<Lang> SubstType for OpLambdaSub<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        let sup_subst = self.sup.subst_type(v, ty);
        if *v == self.var {
            OpLambdaSub {
                var: self.var,
                sup: Box::new(sup_subst),
                body: self.body,
            }
            .into()
        } else {
            OpLambdaSub {
                var: self.var,
                sup: Box::new(sup_subst),
                body: Box::new(self.body.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<Lang> fmt::Display for OpLambdaSub<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:{}.{}", self.var, self.sup, self.body)
    }
}
