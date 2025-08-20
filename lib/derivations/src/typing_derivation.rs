use crate::{Derivation, TypingConclusion, TypingRule};
use std::fmt;
use syntax::{env::Environment, terms::Term, types::Type, untyped::Untyped};

#[derive(Debug)]
pub struct TypingDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub conc: TypingConclusion<T, Ty>,
    pub label: TypingRule,
    pub premises: Vec<Derivation<T, Ty>>,
}

impl<T, Ty> TypingDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn ret_ty(&self) -> Ty {
        self.conc.ty().clone()
    }

    pub fn app(
        conc: TypingConclusion<T, Ty>,
        left: Derivation<T, Ty>,
        right: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::App,
            premises: vec![left, right],
        }
    }

    pub fn ascribe(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Ascribe,
            premises: vec![prem],
        }
    }

    pub fn assign(
        conc: TypingConclusion<T, Ty>,
        left: Derivation<T, Ty>,
        right: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Assign,
            premises: vec![left, right],
        }
    }

    pub fn cast(conc: TypingConclusion<T, Ty>, prem: Derivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Cast,
            premises: vec![prem],
        }
    }

    pub fn cons(
        conc: TypingConclusion<T, Ty>,
        head: Derivation<T, Ty>,
        tail: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Cons,
            premises: vec![head, tail],
        }
    }

    pub fn deref(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Deref,
            premises: vec![prem],
        }
    }

    pub fn exception(conc: TypingConclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Exception,
            premises: vec![],
        }
    }

    pub fn fix(conc: TypingConclusion<T, Ty>, prem: Derivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Fix,
            premises: vec![prem],
        }
    }

    pub fn fls(conc: TypingConclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::False,
            premises: vec![],
        }
    }

    pub fn fold(conc: TypingConclusion<T, Ty>, prem: Derivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Fold,
            premises: vec![prem],
        }
    }

    pub fn fst(conc: TypingConclusion<T, Ty>, prem: Derivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Fst,
            premises: vec![prem],
        }
    }

    pub fn head(conc: TypingConclusion<T, Ty>, prem: Derivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Head,
            premises: vec![prem],
        }
    }

    pub fn ift(
        conc: TypingConclusion<T, Ty>,
        if_deriv: Derivation<T, Ty>,
        then_deriv: Derivation<T, Ty>,
        else_deriv: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::If,
            premises: vec![if_deriv, then_deriv, else_deriv],
        }
    }

    pub fn isnil(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::IsNil,
            premises: vec![prem],
        }
    }

    pub fn iszero(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::IsZero,
            premises: vec![prem],
        }
    }

    pub fn lambda(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Lambda,
            premises: vec![prem],
        }
    }

    pub fn lambdasub(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::LambdaSub,
            premises: vec![prem],
        }
    }

    pub fn left(conc: TypingConclusion<T, Ty>, prem: Derivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Left,
            premises: vec![prem],
        }
    }

    pub fn lett(
        conc: TypingConclusion<T, Ty>,
        bound_deriv: Derivation<T, Ty>,
        in_deriv: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Let,
            premises: vec![bound_deriv, in_deriv],
        }
    }

    pub fn listcase(
        conc: TypingConclusion<T, Ty>,
        nil_deriv: Derivation<T, Ty>,
        cons_deriv: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::ListCase,
            premises: vec![nil_deriv, cons_deriv],
        }
    }

    pub fn loc(conc: TypingConclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Loc,
            premises: vec![],
        }
    }

    pub fn nil(conc: TypingConclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Nil,
            premises: vec![],
        }
    }

    pub fn nothing(conc: TypingConclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Nothing,
            premises: vec![],
        }
    }

    pub fn num(conc: TypingConclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Num,
            premises: vec![],
        }
    }

    pub fn pack(conc: TypingConclusion<T, Ty>, prem: Derivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Pack,
            premises: vec![prem],
        }
    }

    pub fn pack_bound(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::PackBound,
            premises: vec![prem],
        }
    }

    pub fn pair(
        conc: TypingConclusion<T, Ty>,
        fst_deriv: Derivation<T, Ty>,
        snd_deriv: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Pair,
            premises: vec![fst_deriv, snd_deriv],
        }
    }

    pub fn pred(conc: TypingConclusion<T, Ty>, prem: Derivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Pred,
            premises: vec![prem],
        }
    }

    pub fn projection(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Projection,
            premises: vec![prem],
        }
    }

    pub fn raise(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Raise,
            premises: vec![prem],
        }
    }
    pub fn record(
        conc: TypingConclusion<T, Ty>,
        prems: Vec<Derivation<T, Ty>>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Record,
            premises: prems,
        }
    }

    pub fn recordproj(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::RecordProj,
            premises: vec![prem],
        }
    }

    pub fn reft(conc: TypingConclusion<T, Ty>, prem: Derivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Ref,
            premises: vec![prem],
        }
    }

    pub fn right(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Right,
            premises: vec![prem],
        }
    }

    pub fn snd(conc: TypingConclusion<T, Ty>, prem: Derivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Snd,
            premises: vec![prem],
        }
    }

    pub fn somecase(
        conc: TypingConclusion<T, Ty>,
        bound_deriv: Derivation<T, Ty>,
        some_deriv: Derivation<T, Ty>,
        none_deriv: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::SomeCase,
            premises: vec![bound_deriv, some_deriv, none_deriv],
        }
    }

    pub fn something(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Something,
            premises: vec![prem],
        }
    }

    pub fn succ(conc: TypingConclusion<T, Ty>, prem: Derivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Succ,
            premises: vec![prem],
        }
    }

    pub fn sumcase(
        conc: TypingConclusion<T, Ty>,
        bound_deriv: Derivation<T, Ty>,
        left_deriv: Derivation<T, Ty>,
        right_deriv: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::SumCase,
            premises: vec![bound_deriv, left_deriv, right_deriv],
        }
    }

    pub fn tail(conc: TypingConclusion<T, Ty>, prem: Derivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Tail,
            premises: vec![prem],
        }
    }

    pub fn tru(conc: TypingConclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::True,
            premises: vec![],
        }
    }

    pub fn tryt(
        conc: TypingConclusion<T, Ty>,
        term_deriv: Derivation<T, Ty>,
        handler_deriv: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Try,
            premises: vec![term_deriv, handler_deriv],
        }
    }

    pub fn try_val(
        conc: TypingConclusion<T, Ty>,
        term_deriv: Derivation<T, Ty>,
        handler_deriv: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::TryVal,
            premises: vec![term_deriv, handler_deriv],
        }
    }

    pub fn tuple(
        conc: TypingConclusion<T, Ty>,
        term_derivs: Vec<Derivation<T, Ty>>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Tuple,
            premises: term_derivs,
        }
    }

    pub fn tyapp(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::TyApp,
            premises: vec![prem],
        }
    }

    pub fn tyapp_bounded(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::TyAppBounded,
            premises: vec![prem],
        }
    }

    pub fn tylambda(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::TyLambda,
            premises: vec![prem],
        }
    }

    pub fn unfold(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Unfold,
            premises: vec![prem],
        }
    }

    pub fn unit(conc: TypingConclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Unit,
            premises: vec![],
        }
    }

    pub fn unpack(
        conc: TypingConclusion<T, Ty>,
        bound_deriv: Derivation<T, Ty>,
        in_deriv: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Unpack,
            premises: vec![bound_deriv, in_deriv],
        }
    }

    pub fn unpack_bounded(
        conc: TypingConclusion<T, Ty>,
        bound_deriv: Derivation<T, Ty>,
        in_deriv: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::UnpackBounded,
            premises: vec![bound_deriv, in_deriv],
        }
    }

    pub fn var(conc: TypingConclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Variable,
            premises: vec![],
        }
    }

    pub fn variant(
        conc: TypingConclusion<T, Ty>,
        prem: Derivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Variant,
            premises: vec![prem],
        }
    }

    pub fn variantcase(
        conc: TypingConclusion<T, Ty>,
        bound_deriv: Derivation<T, Ty>,
        mut rhs_derivs: Vec<Derivation<T, Ty>>,
    ) -> TypingDerivation<T, Ty> {
        rhs_derivs.insert(0, bound_deriv);
        TypingDerivation {
            conc,
            label: TypingRule::VariantCase,
            premises: rhs_derivs,
        }
    }

    pub fn untyped_lambda(conc: TypingConclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Empty,
            premises: vec![],
        }
    }
}

impl<T> TypingDerivation<T, Untyped<T>>
where
    T: Term,
{
    pub fn empty<T1>(t: T1) -> TypingDerivation<T, Untyped<T>>
    where
        T1: Into<T>,
    {
        TypingDerivation {
            conc: TypingConclusion::new(Environment::default(), t, Untyped::new()),
            label: TypingRule::Empty,
            premises: vec![],
        }
    }
}

impl<T, Ty> fmt::Display for TypingDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for prem in self.premises.iter() {
            writeln!(f, "{prem}")?;
            writeln!(f,)?;
        }
        writeln!(f, "=== {} ===", self.label)?;
        write!(f, "{}", self.conc)
    }
}
