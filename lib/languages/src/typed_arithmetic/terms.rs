use super::{TypedArithmetic, types::Type};
use macros::{Eval, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, Typecheck};
use syntax::{
    TypeVar, Var,
    subst::SubstType,
    terms::{False, If, IsZero, Num, Pred, Succ, True},
};

#[derive(
    SubstTerm, LatexFmt, LangDisplay, GrammarDescribe, Eval, Typecheck, Debug, Clone, PartialEq, Eq,
)]
#[Lang(TypedArithmetic)]
pub enum Term {
    True(True<TypedArithmetic>),
    False(False<TypedArithmetic>),
    If(If<TypedArithmetic>),
    Num(Num<TypedArithmetic>),
    Succ(Succ<TypedArithmetic>),
    Pred(Pred<TypedArithmetic>),
    IsZero(IsZero<TypedArithmetic>),
}

impl syntax::terms::Term for Term {}

impl SubstType for Term {
    type Lang = TypedArithmetic;
    type Target = Term;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}

impl From<True<TypedArithmetic>> for Term {
    fn from(tru: True<TypedArithmetic>) -> Term {
        Term::True(tru)
    }
}
impl From<False<TypedArithmetic>> for Term {
    fn from(fls: False<TypedArithmetic>) -> Term {
        Term::False(fls)
    }
}
impl From<If<TypedArithmetic>> for Term {
    fn from(ift: If<TypedArithmetic>) -> Term {
        Term::If(ift)
    }
}
impl From<Num<TypedArithmetic>> for Term {
    fn from(num: Num<TypedArithmetic>) -> Term {
        Term::Num(num)
    }
}
impl From<Succ<TypedArithmetic>> for Term {
    fn from(succ: Succ<TypedArithmetic>) -> Term {
        Term::Succ(succ)
    }
}
impl From<Pred<TypedArithmetic>> for Term {
    fn from(pred: Pred<TypedArithmetic>) -> Term {
        Term::Pred(pred)
    }
}
impl From<IsZero<TypedArithmetic>> for Term {
    fn from(isz: IsZero<TypedArithmetic>) -> Term {
        Term::IsZero(isz)
    }
}
