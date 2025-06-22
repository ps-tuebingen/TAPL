use super::{Conclusion, TypingRule};
use syntax::{env::Environment, terms::Term, types::Type, untyped::Untyped};

pub struct TypingDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub conc: Conclusion<T, Ty>,
    pub label: TypingRule,
    pub premises: Vec<TypingDerivation<T, Ty>>,
}

impl<T, Ty> TypingDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn ty(&self) -> Ty {
        self.conc.ty.clone()
    }

    pub fn app(
        conc: Conclusion<T, Ty>,
        left: TypingDerivation<T, Ty>,
        right: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::App,
            premises: vec![left, right],
        }
    }

    pub fn ascribe(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Ascribe,
            premises: vec![prem],
        }
    }

    pub fn assign(
        conc: Conclusion<T, Ty>,
        left: TypingDerivation<T, Ty>,
        right: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Assign,
            premises: vec![left, right],
        }
    }

    pub fn cast(conc: Conclusion<T, Ty>, prem: TypingDerivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Cast,
            premises: vec![prem],
        }
    }

    pub fn cons(
        conc: Conclusion<T, Ty>,
        head: TypingDerivation<T, Ty>,
        tail: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Cons,
            premises: vec![head, tail],
        }
    }

    pub fn deref(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Deref,
            premises: vec![prem],
        }
    }

    pub fn exception(conc: Conclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Exception,
            premises: vec![],
        }
    }

    pub fn fix(conc: Conclusion<T, Ty>, prem: TypingDerivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Fix,
            premises: vec![prem],
        }
    }

    pub fn fls(conc: Conclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::False,
            premises: vec![],
        }
    }

    pub fn fold(conc: Conclusion<T, Ty>, prem: TypingDerivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Fold,
            premises: vec![prem],
        }
    }

    pub fn fst(conc: Conclusion<T, Ty>, prem: TypingDerivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Fst,
            premises: vec![prem],
        }
    }

    pub fn head(conc: Conclusion<T, Ty>, prem: TypingDerivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Head,
            premises: vec![prem],
        }
    }

    pub fn ift(
        conc: Conclusion<T, Ty>,
        if_deriv: TypingDerivation<T, Ty>,
        then_deriv: TypingDerivation<T, Ty>,
        else_deriv: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::If,
            premises: vec![if_deriv, then_deriv, else_deriv],
        }
    }

    pub fn isnil(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::IsNil,
            premises: vec![prem],
        }
    }

    pub fn iszero(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::IsZero,
            premises: vec![prem],
        }
    }

    pub fn lambda(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Lambda,
            premises: vec![prem],
        }
    }

    pub fn lambdasub(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::LambdaSub,
            premises: vec![prem],
        }
    }

    pub fn left(conc: Conclusion<T, Ty>, prem: TypingDerivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Left,
            premises: vec![prem],
        }
    }

    pub fn lett(
        conc: Conclusion<T, Ty>,
        bound_deriv: TypingDerivation<T, Ty>,
        in_deriv: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Let,
            premises: vec![bound_deriv, in_deriv],
        }
    }

    pub fn listcase(
        conc: Conclusion<T, Ty>,
        nil_deriv: TypingDerivation<T, Ty>,
        cons_deriv: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::ListCase,
            premises: vec![nil_deriv, cons_deriv],
        }
    }

    pub fn loc(conc: Conclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Loc,
            premises: vec![],
        }
    }

    pub fn nil(conc: Conclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Nil,
            premises: vec![],
        }
    }

    pub fn nothing(conc: Conclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Nothing,
            premises: vec![],
        }
    }

    pub fn num(conc: Conclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Num,
            premises: vec![],
        }
    }

    pub fn pack(conc: Conclusion<T, Ty>, prem: TypingDerivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Pack,
            premises: vec![prem],
        }
    }

    pub fn pack_bound(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::PackBound,
            premises: vec![prem],
        }
    }

    pub fn pair(
        conc: Conclusion<T, Ty>,
        fst_deriv: TypingDerivation<T, Ty>,
        snd_deriv: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Pair,
            premises: vec![fst_deriv, snd_deriv],
        }
    }

    pub fn pred(conc: Conclusion<T, Ty>, prem: TypingDerivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Pred,
            premises: vec![prem],
        }
    }

    pub fn projection(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Projection,
            premises: vec![prem],
        }
    }

    pub fn raise(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Raise,
            premises: vec![prem],
        }
    }
    pub fn record(
        conc: Conclusion<T, Ty>,
        prems: Vec<TypingDerivation<T, Ty>>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Record,
            premises: prems,
        }
    }

    pub fn recordproj(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::RecordProj,
            premises: vec![prem],
        }
    }

    pub fn reft(conc: Conclusion<T, Ty>, prem: TypingDerivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Ref,
            premises: vec![prem],
        }
    }

    pub fn right(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Right,
            premises: vec![prem],
        }
    }

    pub fn snd(conc: Conclusion<T, Ty>, prem: TypingDerivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Snd,
            premises: vec![prem],
        }
    }

    pub fn somecase(
        conc: Conclusion<T, Ty>,
        bound_deriv: TypingDerivation<T, Ty>,
        some_deriv: TypingDerivation<T, Ty>,
        none_deriv: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::SomeCase,
            premises: vec![bound_deriv, some_deriv, none_deriv],
        }
    }

    pub fn something(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Something,
            premises: vec![prem],
        }
    }

    pub fn succ(conc: Conclusion<T, Ty>, prem: TypingDerivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Succ,
            premises: vec![prem],
        }
    }

    pub fn sumcase(
        conc: Conclusion<T, Ty>,
        bound_deriv: TypingDerivation<T, Ty>,
        left_deriv: TypingDerivation<T, Ty>,
        right_deriv: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::SumCase,
            premises: vec![bound_deriv, left_deriv, right_deriv],
        }
    }

    pub fn tail(conc: Conclusion<T, Ty>, prem: TypingDerivation<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Tail,
            premises: vec![prem],
        }
    }

    pub fn tru(conc: Conclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::True,
            premises: vec![],
        }
    }

    pub fn tryt(
        conc: Conclusion<T, Ty>,
        term_deriv: TypingDerivation<T, Ty>,
        handler_deriv: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Try,
            premises: vec![term_deriv, handler_deriv],
        }
    }

    pub fn try_val(
        conc: Conclusion<T, Ty>,
        term_deriv: TypingDerivation<T, Ty>,
        handler_deriv: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::TryVal,
            premises: vec![term_deriv, handler_deriv],
        }
    }

    pub fn tuple(
        conc: Conclusion<T, Ty>,
        term_derivs: Vec<TypingDerivation<T, Ty>>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Tuple,
            premises: term_derivs,
        }
    }

    pub fn tyapp(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::TyApp,
            premises: vec![prem],
        }
    }

    pub fn tyapp_bounded(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::TyAppBounded,
            premises: vec![prem],
        }
    }

    pub fn tylambda(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::TyLambda,
            premises: vec![prem],
        }
    }

    pub fn unfold(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Unfold,
            premises: vec![prem],
        }
    }

    pub fn unit(conc: Conclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Unit,
            premises: vec![],
        }
    }

    pub fn unpack(
        conc: Conclusion<T, Ty>,
        bound_deriv: TypingDerivation<T, Ty>,
        in_deriv: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Unpack,
            premises: vec![bound_deriv, in_deriv],
        }
    }

    pub fn unpack_bounded(
        conc: Conclusion<T, Ty>,
        bound_deriv: TypingDerivation<T, Ty>,
        in_deriv: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::UnpackBounded,
            premises: vec![bound_deriv, in_deriv],
        }
    }

    pub fn var(conc: Conclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Variable,
            premises: vec![],
        }
    }

    pub fn variant(
        conc: Conclusion<T, Ty>,
        prem: TypingDerivation<T, Ty>,
    ) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Variant,
            premises: vec![prem],
        }
    }

    pub fn variantcase(
        conc: Conclusion<T, Ty>,
        bound_deriv: TypingDerivation<T, Ty>,
        mut rhs_derivs: Vec<TypingDerivation<T, Ty>>,
    ) -> TypingDerivation<T, Ty> {
        rhs_derivs.insert(0, bound_deriv);
        TypingDerivation {
            conc,
            label: TypingRule::VariantCase,
            premises: rhs_derivs,
        }
    }

    pub fn untyped_lambda(conc: Conclusion<T, Ty>) -> TypingDerivation<T, Ty> {
        TypingDerivation {
            conc,
            label: TypingRule::Empty,
            premises: vec![],
        }
    }
}

impl<T> TypingDerivation<T, Untyped>
where
    T: Term,
{
    pub fn empty<T1>(t: T1) -> TypingDerivation<T, Untyped>
    where
        T1: Into<T>,
    {
        TypingDerivation {
            conc: Conclusion::new(Environment::default(), t, Untyped),
            label: TypingRule::Empty,
            premises: vec![],
        }
    }
}
