use super::{OpLambdaSub, Top, Type};
use crate::{TypeVar, kinds::Kind, language::Language, subst::SubstType};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpLambda<Lang>
where
    Lang: Language,
{
    pub var: TypeVar,
    pub annot: Kind,
    pub body: Rc<Lang::Type>,
}

impl<Lang> OpLambda<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1>(var: &str, knd: Kind, ty: Ty1) -> OpLambda<Lang>
    where
        Ty1: Into<Lang::Type>,
    {
        OpLambda {
            var: var.to_owned(),
            annot: knd,
            body: Rc::new(ty.into()),
        }
    }

    pub fn to_oplambda_unbounded(self) -> OpLambdaSub<Lang>
    where
        Top<Lang>: Into<Lang::Type>,
    {
        OpLambdaSub::new_unbounded(&self.var, self.annot, Rc::unwrap_or_clone(self.body))
    }
}

impl<Lang> Type for OpLambda<Lang> where Lang: Language {}

impl<Lang> SubstType for OpLambda<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        if *v == self.var {
            self
        } else {
            OpLambda {
                var: self.var,
                annot: self.annot,
                body: self.body.subst_type(v, ty),
            }
        }
    }
}

impl<Lang> fmt::Display for OpLambda<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}::{}.{}", self.var, self.annot, self.body)
    }
}
