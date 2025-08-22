use crate::{Derivation, TypingConclusion, TypingRule};
use std::fmt;
use syntax::language::Language;

#[derive(Debug)]
pub struct TypingDerivation<Lang>
where
    Lang: Language,
{
    pub conc: TypingConclusion<Lang>,
    pub label: TypingRule,
    pub premises: Vec<Derivation<Lang>>,
}

impl<Lang> TypingDerivation<Lang>
where
    Lang: Language,
{
    pub fn ret_ty(&self) -> Lang::Type {
        self.conc.ty().clone()
    }

    pub fn app(
        conc: TypingConclusion<Lang>,
        left: Derivation<Lang>,
        right: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::App,
            premises: vec![left, right],
        }
    }

    pub fn ascribe(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Ascribe,
            premises: vec![prem],
        }
    }

    pub fn assign(
        conc: TypingConclusion<Lang>,
        left: Derivation<Lang>,
        right: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Assign,
            premises: vec![left, right],
        }
    }

    pub fn cast(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Cast,
            premises: vec![prem],
        }
    }

    pub fn cons(
        conc: TypingConclusion<Lang>,
        head: Derivation<Lang>,
        tail: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Cons,
            premises: vec![head, tail],
        }
    }

    pub fn deref(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Deref,
            premises: vec![prem],
        }
    }

    pub fn exception(conc: TypingConclusion<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Exception,
            premises: vec![],
        }
    }

    pub fn fix(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Fix,
            premises: vec![prem],
        }
    }

    pub fn fls(conc: TypingConclusion<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::False,
            premises: vec![],
        }
    }

    pub fn fold(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Fold,
            premises: vec![prem],
        }
    }

    pub fn fst(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Fst,
            premises: vec![prem],
        }
    }

    pub fn head(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Head,
            premises: vec![prem],
        }
    }

    pub fn ift(
        conc: TypingConclusion<Lang>,
        if_deriv: Derivation<Lang>,
        then_deriv: Derivation<Lang>,
        else_deriv: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::If,
            premises: vec![if_deriv, then_deriv, else_deriv],
        }
    }

    pub fn isnil(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::IsNil,
            premises: vec![prem],
        }
    }

    pub fn iszero(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::IsZero,
            premises: vec![prem],
        }
    }

    pub fn lambda(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Lambda,
            premises: vec![prem],
        }
    }

    pub fn lambdasub(
        conc: TypingConclusion<Lang>,
        prem: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::LambdaSub,
            premises: vec![prem],
        }
    }

    pub fn left(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Left,
            premises: vec![prem],
        }
    }

    pub fn lett(
        conc: TypingConclusion<Lang>,
        bound_deriv: Derivation<Lang>,
        in_deriv: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Let,
            premises: vec![bound_deriv, in_deriv],
        }
    }

    pub fn listcase(
        conc: TypingConclusion<Lang>,
        nil_deriv: Derivation<Lang>,
        cons_deriv: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::ListCase,
            premises: vec![nil_deriv, cons_deriv],
        }
    }

    pub fn loc(conc: TypingConclusion<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Loc,
            premises: vec![],
        }
    }

    pub fn nil(conc: TypingConclusion<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Nil,
            premises: vec![],
        }
    }

    pub fn nothing(conc: TypingConclusion<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Nothing,
            premises: vec![],
        }
    }

    pub fn num(conc: TypingConclusion<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Num,
            premises: vec![],
        }
    }

    pub fn pack(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Pack,
            premises: vec![prem],
        }
    }

    pub fn pack_bound(
        conc: TypingConclusion<Lang>,
        prem: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::PackBound,
            premises: vec![prem],
        }
    }

    pub fn pair(
        conc: TypingConclusion<Lang>,
        fst_deriv: Derivation<Lang>,
        snd_deriv: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Pair,
            premises: vec![fst_deriv, snd_deriv],
        }
    }

    pub fn pred(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Pred,
            premises: vec![prem],
        }
    }

    pub fn projection(
        conc: TypingConclusion<Lang>,
        prem: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Projection,
            premises: vec![prem],
        }
    }

    pub fn raise(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Raise,
            premises: vec![prem],
        }
    }
    pub fn record(
        conc: TypingConclusion<Lang>,
        prems: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Record,
            premises: prems,
        }
    }

    pub fn recordproj(
        conc: TypingConclusion<Lang>,
        prem: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::RecordProj,
            premises: vec![prem],
        }
    }

    pub fn reft(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Ref,
            premises: vec![prem],
        }
    }

    pub fn right(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Right,
            premises: vec![prem],
        }
    }

    pub fn snd(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Snd,
            premises: vec![prem],
        }
    }

    pub fn somecase(
        conc: TypingConclusion<Lang>,
        bound_deriv: Derivation<Lang>,
        some_deriv: Derivation<Lang>,
        none_deriv: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::SomeCase,
            premises: vec![bound_deriv, some_deriv, none_deriv],
        }
    }

    pub fn something(
        conc: TypingConclusion<Lang>,
        prem: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Something,
            premises: vec![prem],
        }
    }

    pub fn succ(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Succ,
            premises: vec![prem],
        }
    }

    pub fn sumcase(
        conc: TypingConclusion<Lang>,
        bound_deriv: Derivation<Lang>,
        left_deriv: Derivation<Lang>,
        right_deriv: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::SumCase,
            premises: vec![bound_deriv, left_deriv, right_deriv],
        }
    }

    pub fn tail(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Tail,
            premises: vec![prem],
        }
    }

    pub fn tru(conc: TypingConclusion<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::True,
            premises: vec![],
        }
    }

    pub fn tryt(
        conc: TypingConclusion<Lang>,
        term_deriv: Derivation<Lang>,
        handler_deriv: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Try,
            premises: vec![term_deriv, handler_deriv],
        }
    }

    pub fn try_val(
        conc: TypingConclusion<Lang>,
        term_deriv: Derivation<Lang>,
        handler_deriv: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::TryVal,
            premises: vec![term_deriv, handler_deriv],
        }
    }

    pub fn tuple(
        conc: TypingConclusion<Lang>,
        term_derivs: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Tuple,
            premises: term_derivs,
        }
    }

    pub fn tyapp(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::TyApp,
            premises: vec![prem],
        }
    }

    pub fn tyapp_bounded(
        conc: TypingConclusion<Lang>,
        prem: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::TyAppBounded,
            premises: vec![prem],
        }
    }

    pub fn tylambda(
        conc: TypingConclusion<Lang>,
        prem: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::TyLambda,
            premises: vec![prem],
        }
    }

    pub fn unfold(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Unfold,
            premises: vec![prem],
        }
    }

    pub fn unit(conc: TypingConclusion<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Unit,
            premises: vec![],
        }
    }

    pub fn unpack(
        conc: TypingConclusion<Lang>,
        bound_deriv: Derivation<Lang>,
        in_deriv: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Unpack,
            premises: vec![bound_deriv, in_deriv],
        }
    }

    pub fn unpack_bounded(
        conc: TypingConclusion<Lang>,
        bound_deriv: Derivation<Lang>,
        in_deriv: Derivation<Lang>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::UnpackBounded,
            premises: vec![bound_deriv, in_deriv],
        }
    }

    pub fn var(conc: TypingConclusion<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Variable,
            premises: vec![],
        }
    }

    pub fn variant(conc: TypingConclusion<Lang>, prem: Derivation<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Variant,
            premises: vec![prem],
        }
    }

    pub fn variantcase(
        conc: TypingConclusion<Lang>,
        bound_deriv: Derivation<Lang>,
        mut rhs_derivs: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        rhs_derivs.insert(0, bound_deriv);
        TypingDerivation {
            conc,
            label: TypingRule::VariantCase,
            premises: rhs_derivs,
        }
    }

    pub fn untyped_lambda(conc: TypingConclusion<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Empty,
            premises: vec![],
        }
    }
}

impl<Lang> fmt::Display for TypingDerivation<Lang>
where
    Lang: Language,
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
