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
        self.conc.ty()
    }

    pub const fn app(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::App,
            premises,
        }
    }

    pub const fn ascribe(
        conc: TypingConclusion<Lang>,
        prem: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Ascribe,
            premises: prem,
        }
    }

    pub const fn assign(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Assign,
            premises,
        }
    }

    pub const fn cast(
        conc: TypingConclusion<Lang>,
        prem: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Cast,
            premises: prem,
        }
    }

    pub const fn cons(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Cons,
            premises,
        }
    }

    pub const fn deref(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Deref,
            premises,
        }
    }

    pub const fn exception(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Exception,
            premises,
        }
    }

    pub const fn fix(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Fix,
            premises,
        }
    }

    pub const fn fls(conc: TypingConclusion<Lang>) -> Self {
        Self {
            conc,
            label: TypingRule::False,
            premises: vec![],
        }
    }

    pub const fn fold(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Fold,
            premises,
        }
    }

    pub const fn fst(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Fst,
            premises,
        }
    }

    pub const fn head(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Head,
            premises,
        }
    }

    pub const fn ift(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::If,
            premises,
        }
    }

    pub const fn isnil(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::IsNil,
            premises,
        }
    }

    pub const fn iszero(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::IsZero,
            premises,
        }
    }

    pub const fn lambda(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Lambda,
            premises,
        }
    }

    pub const fn lambdasub(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::LambdaSub,
            premises,
        }
    }

    pub const fn left(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Left,
            premises,
        }
    }

    pub const fn lett(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Let,
            premises,
        }
    }

    pub const fn listcase(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::ListCase,
            premises,
        }
    }

    pub const fn loc(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Loc,
            premises,
        }
    }

    pub const fn nil(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Nil,
            premises,
        }
    }

    pub const fn nothing(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Nothing,
            premises,
        }
    }

    pub const fn num(conc: TypingConclusion<Lang>) -> Self {
        Self {
            conc,
            label: TypingRule::Num,
            premises: vec![],
        }
    }

    pub const fn pack(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Pack,
            premises,
        }
    }

    pub const fn pack_bound(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::PackBound,
            premises,
        }
    }

    pub const fn pair(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Pair,
            premises,
        }
    }

    pub const fn pred(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Pred,
            premises,
        }
    }

    pub const fn projection(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Projection,
            premises,
        }
    }

    pub const fn raise(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Raise,
            premises,
        }
    }
    pub const fn record(
        conc: TypingConclusion<Lang>,
        prems: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Record,
            premises: prems,
        }
    }

    pub const fn recordproj(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::RecordProj,
            premises,
        }
    }

    pub const fn reft(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Ref,
            premises,
        }
    }

    pub const fn right(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Right,
            premises,
        }
    }

    pub const fn snd(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Snd,
            premises,
        }
    }

    pub const fn somecase(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::SomeCase,
            premises,
        }
    }

    pub const fn something(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Something,
            premises,
        }
    }

    pub const fn succ(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Succ,
            premises,
        }
    }

    pub const fn sumcase(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::SumCase,
            premises,
        }
    }

    pub const fn tail(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Tail,
            premises,
        }
    }

    pub const fn tru(conc: TypingConclusion<Lang>) -> Self {
        Self {
            conc,
            label: TypingRule::True,
            premises: vec![],
        }
    }

    pub const fn tryt(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Try,
            premises,
        }
    }

    pub const fn try_val(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::TryVal,
            premises,
        }
    }

    pub const fn tuple(
        conc: TypingConclusion<Lang>,
        term_derivs: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Tuple,
            premises: term_derivs,
        }
    }

    pub const fn tyapp(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::TyApp,
            premises,
        }
    }

    pub const fn tyapp_bounded(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::TyAppBounded,
            premises,
        }
    }

    pub const fn tylambda(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::TyLambda,
            premises,
        }
    }

    pub const fn unfold(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Unfold,
            premises,
        }
    }

    pub const fn unit(conc: TypingConclusion<Lang>) -> Self {
        Self {
            conc,
            label: TypingRule::Unit,
            premises: vec![],
        }
    }

    pub const fn unpack(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Unpack,
            premises,
        }
    }

    pub const fn unpack_bounded(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::UnpackBounded,
            premises,
        }
    }

    pub const fn var(conc: TypingConclusion<Lang>) -> Self {
        Self {
            conc,
            label: TypingRule::Variable,
            premises: vec![],
        }
    }

    pub const fn variant(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::Variant,
            premises,
        }
    }

    pub const fn variantcase(
        conc: TypingConclusion<Lang>,
        premises: Vec<Derivation<Lang>>,
    ) -> Self {
        Self {
            conc,
            label: TypingRule::VariantCase,
            premises,
        }
    }

    pub const fn untyped_lambda(conc: TypingConclusion<Lang>) -> Self {
        Self {
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
        for prem in &self.premises {
            writeln!(f, "{prem}")?;
            writeln!(f,)?;
        }
        writeln!(f, "=== {} ===", self.label)?;
        write!(f, "{}", self.conc)
    }
}
