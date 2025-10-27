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
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::App,
            premises,
        }
    }

    pub fn ascribe(
        conc: TypingConclusion<Lang>,
        prem: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Ascribe,
            premises: prem,
        }
    }

    pub fn assign(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Assign,
            premises,
        }
    }

    pub fn cast(
        conc: TypingConclusion<Lang>,
        prem: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Cast,
            premises: prem,
        }
    }

    pub fn cons(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Cons,
            premises,
        }
    }

    pub fn deref(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Deref,
            premises,
        }
    }

    pub fn exception(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Exception,
            premises,
        }
    }

    pub fn fix(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Fix,
            premises,
        }
    }

    pub fn fls(conc: TypingConclusion<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::False,
            premises: vec![],
        }
    }

    pub fn fold(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Fold,
            premises,
        }
    }

    pub fn fst(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Fst,
            premises,
        }
    }

    pub fn head(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Head,
            premises,
        }
    }

    pub fn ift(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::If,
            premises,
        }
    }

    pub fn isnil(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::IsNil,
            premises,
        }
    }

    pub fn iszero(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::IsZero,
            premises,
        }
    }

    pub fn lambda(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Lambda,
            premises,
        }
    }

    pub fn lambdasub(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::LambdaSub,
            premises,
        }
    }

    pub fn left(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Left,
            premises,
        }
    }

    pub fn lett(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Let,
            premises,
        }
    }

    pub fn listcase(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::ListCase,
            premises,
        }
    }

    pub fn loc(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Loc,
            premises,
        }
    }

    pub fn nil(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Nil,
            premises,
        }
    }

    pub fn nothing(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Nothing,
            premises,
        }
    }

    pub fn num(conc: TypingConclusion<Lang>) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Num,
            premises: vec![],
        }
    }

    pub fn pack(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Pack,
            premises,
        }
    }

    pub fn pack_bound(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::PackBound,
            premises,
        }
    }

    pub fn pair(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Pair,
            premises,
        }
    }

    pub fn pred(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Pred,
            premises,
        }
    }

    pub fn projection(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Projection,
            premises,
        }
    }

    pub fn raise(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Raise,
            premises,
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
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::RecordProj,
            premises,
        }
    }

    pub fn reft(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Ref,
            premises,
        }
    }

    pub fn right(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Right,
            premises,
        }
    }

    pub fn snd(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Snd,
            premises,
        }
    }

    pub fn somecase(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::SomeCase,
            premises,
        }
    }

    pub fn something(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Something,
            premises,
        }
    }

    pub fn succ(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Succ,
            premises,
        }
    }

    pub fn sumcase(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::SumCase,
            premises,
        }
    }

    pub fn tail(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Tail,
            premises,
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
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Try,
            premises,
        }
    }

    pub fn try_val(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::TryVal,
            premises,
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

    pub fn tyapp(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::TyApp,
            premises,
        }
    }

    pub fn tyapp_bounded(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::TyAppBounded,
            premises,
        }
    }

    pub fn tylambda(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::TyLambda,
            premises,
        }
    }

    pub fn unfold(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Unfold,
            premises,
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
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::Unpack,
            premises,
        }
    }

    pub fn unpack_bounded(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::UnpackBounded,
            premises,
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
        premises: Vec<Derivation<Lang>>,
    ) -> TypingDerivation<Lang> {
        TypingDerivation {
            conc,
            label: TypingRule::VariantCase,
            premises,
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
