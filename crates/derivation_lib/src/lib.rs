use syntax::{terms::Term, types::Type};

pub mod conclusion;
pub mod rules;
pub use conclusion::Conclusion;
pub use rules::TypingRule;

pub struct Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    conc: Conclusion<T, Ty>,
    label: TypingRule,
    premises: Vec<Derivation<T, Ty>>,
}

impl<T, Ty> Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn ty(&self) -> Ty {
        self.conc.ty.clone()
    }

    pub fn app(
        conc: Conclusion<T, Ty>,
        left: Derivation<T, Ty>,
        right: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::App,
            premises: vec![left, right],
        }
    }

    pub fn ascribe(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Ascribe,
            premises: vec![prem],
        }
    }

    pub fn assign(
        conc: Conclusion<T, Ty>,
        left: Derivation<T, Ty>,
        right: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Assign,
            premises: vec![left, right],
        }
    }

    pub fn cast(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Cast,
            premises: vec![prem],
        }
    }

    pub fn cons(
        conc: Conclusion<T, Ty>,
        head: Derivation<T, Ty>,
        tail: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Cons,
            premises: vec![head, tail],
        }
    }

    pub fn deref(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Deref,
            premises: vec![prem],
        }
    }

    pub fn exception(conc: Conclusion<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Exception,
            premises: vec![],
        }
    }

    pub fn fix(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Fix,
            premises: vec![prem],
        }
    }

    pub fn fls(conc: Conclusion<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::False,
            premises: vec![],
        }
    }

    pub fn fold(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Fold,
            premises: vec![prem],
        }
    }

    pub fn fst(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Fst,
            premises: vec![prem],
        }
    }

    pub fn head(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Head,
            premises: vec![prem],
        }
    }

    pub fn ift(
        conc: Conclusion<T, Ty>,
        if_deriv: Derivation<T, Ty>,
        then_deriv: Derivation<T, Ty>,
        else_deriv: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::If,
            premises: vec![if_deriv, then_deriv, else_deriv],
        }
    }

    pub fn isnil(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::IsNil,
            premises: vec![prem],
        }
    }

    pub fn iszero(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::IsZero,
            premises: vec![prem],
        }
    }

    pub fn lambda(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Lambda,
            premises: vec![prem],
        }
    }

    pub fn lambdasub(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::LambdaSub,
            premises: vec![prem],
        }
    }

    pub fn left(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Left,
            premises: vec![prem],
        }
    }

    pub fn lett(
        conc: Conclusion<T, Ty>,
        bound_deriv: Derivation<T, Ty>,
        in_deriv: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Let,
            premises: vec![bound_deriv, in_deriv],
        }
    }

    pub fn listcase(
        conc: Conclusion<T, Ty>,
        nil_deriv: Derivation<T, Ty>,
        cons_deriv: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::ListCase,
            premises: vec![nil_deriv, cons_deriv],
        }
    }

    pub fn loc(conc: Conclusion<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Loc,
            premises: vec![],
        }
    }

    pub fn nil(conc: Conclusion<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Nil,
            premises: vec![],
        }
    }

    pub fn nothing(conc: Conclusion<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Nothing,
            premises: vec![],
        }
    }

    pub fn num(conc: Conclusion<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Num,
            premises: vec![],
        }
    }

    pub fn pack(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Pack,
            premises: vec![prem],
        }
    }

    pub fn pack_bound(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::PackBound,
            premises: vec![prem],
        }
    }

    pub fn pair(
        conc: Conclusion<T, Ty>,
        fst_deriv: Derivation<T, Ty>,
        snd_deriv: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Pair,
            premises: vec![fst_deriv, snd_deriv],
        }
    }

    pub fn pred(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Pred,
            premises: vec![prem],
        }
    }

    pub fn projection(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Projection,
            premises: vec![prem],
        }
    }

    pub fn raise(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Raise,
            premises: vec![prem],
        }
    }
    pub fn record(conc: Conclusion<T, Ty>, prems: Vec<Derivation<T, Ty>>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Record,
            premises: prems,
        }
    }

    pub fn recordproj(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::RecordProj,
            premises: vec![prem],
        }
    }

    pub fn reft(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Ref,
            premises: vec![prem],
        }
    }

    pub fn right(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Right,
            premises: vec![prem],
        }
    }

    pub fn snd(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Snd,
            premises: vec![prem],
        }
    }

    pub fn somecase(
        conc: Conclusion<T, Ty>,
        bound_deriv: Derivation<T, Ty>,
        some_deriv: Derivation<T, Ty>,
        none_deriv: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::SomeCase,
            premises: vec![bound_deriv, some_deriv, none_deriv],
        }
    }

    pub fn something(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Something,
            premises: vec![prem],
        }
    }

    pub fn succ(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Succ,
            premises: vec![prem],
        }
    }

    pub fn sumcase(
        conc: Conclusion<T, Ty>,
        bound_deriv: Derivation<T, Ty>,
        left_deriv: Derivation<T, Ty>,
        right_deriv: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::SumCase,
            premises: vec![bound_deriv, left_deriv, right_deriv],
        }
    }

    pub fn tail(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Tail,
            premises: vec![prem],
        }
    }

    pub fn tru(conc: Conclusion<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::True,
            premises: vec![],
        }
    }

    pub fn tryt(
        conc: Conclusion<T, Ty>,
        term_deriv: Derivation<T, Ty>,
        handler_deriv: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Try,
            premises: vec![term_deriv, handler_deriv],
        }
    }

    pub fn try_val(
        conc: Conclusion<T, Ty>,
        term_deriv: Derivation<T, Ty>,
        handler_deriv: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::TryVal,
            premises: vec![term_deriv, handler_deriv],
        }
    }

    pub fn tuple(
        conc: Conclusion<T, Ty>,
        term_derivs: Vec<Derivation<T, Ty>>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Tuple,
            premises: term_derivs,
        }
    }

    pub fn tyapp(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::TyApp,
            premises: vec![prem],
        }
    }

    pub fn tyapp_bounded(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::TyAppBounded,
            premises: vec![prem],
        }
    }

    pub fn tylambda(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::TyLambda,
            premises: vec![prem],
        }
    }

    pub fn unfold(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Unfold,
            premises: vec![prem],
        }
    }

    pub fn unit(conc: Conclusion<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Unit,
            premises: vec![],
        }
    }

    pub fn unpack(
        conc: Conclusion<T, Ty>,
        bound_deriv: Derivation<T, Ty>,
        in_deriv: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Unpack,
            premises: vec![bound_deriv, in_deriv],
        }
    }

    pub fn unpack_bounded(
        conc: Conclusion<T, Ty>,
        bound_deriv: Derivation<T, Ty>,
        in_deriv: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::UnpackBounded,
            premises: vec![bound_deriv, in_deriv],
        }
    }

    pub fn var(conc: Conclusion<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Variable,
            premises: vec![],
        }
    }

    pub fn variant(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Variant,
            premises: vec![prem],
        }
    }

    pub fn variantcase(
        conc: Conclusion<T, Ty>,
        bound_deriv: Derivation<T, Ty>,
        mut rhs_derivs: Vec<Derivation<T, Ty>>,
    ) -> Derivation<T, Ty> {
        rhs_derivs.insert(0, bound_deriv);
        Derivation {
            conc,
            label: TypingRule::VariantCase,
            premises: rhs_derivs,
        }
    }
}
