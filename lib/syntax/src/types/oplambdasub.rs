use super::{Top, Type};
use crate::{TypeVar, kinds::Kind, language::Language, subst::SubstType};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpLambdaSub<Lang>
where
    Lang: Language,
{
    pub var: TypeVar,
    pub sup: Rc<Lang::Type>,
    pub body: Rc<Lang::Type>,
}

impl<Lang> OpLambdaSub<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(var: &str, sup: Ty1, ty: Ty2) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            var: var.to_owned(),
            sup: Rc::new(sup.into()),
            body: Rc::new(ty.into()),
        }
    }

    pub fn new_unbounded<Ty1>(var: &str, knd: Kind, ty: Ty1) -> Self
    where
        Ty1: Into<Lang::Type>,
        Top<Lang>: Into<Lang::Type>,
    {
        Self {
            var: var.to_owned(),
            sup: Rc::new(Top::new(knd).into()),
            body: Rc::new(ty.into()),
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
            Self {
                var: self.var,
                sup: sup_subst,
                body: self.body,
            }
        } else {
            Self {
                var: self.var,
                sup: sup_subst,
                body: self.body.subst_type(v, ty),
            }
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
