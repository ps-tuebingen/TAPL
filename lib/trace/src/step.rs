use super::rules::EvaluationRule;
use std::fmt;
use syntax::{
    Location, Var,
    language::Language,
    terms::{Assign, Cast, Deref, False, IsNil, Loc, Num, Pair, Pred, Succ, True, Unit, Variable},
    types::Type,
    values::Value,
};

/// Step in an evaluation of a term in a given language
#[derive(Debug)]
pub struct EvalStep<Lang>
where
    Lang: Language,
{
    /// Term to be evaluated
    pub source: Lang::Term,
    /// Rule to be applied
    pub rule: EvaluationRule,
    /// Term after evaluation
    pub target: Lang::Term,
}

impl<Lang> EvalStep<Lang>
where
    Lang: Language,
{
    /// Turn `Self` into a congruence rule
    /// maps [`Self::source`] and [`Self::target`] according to the given function
    #[must_use]
    pub fn congruence(self, into_fun: &impl Fn(Lang::Term) -> Lang::Term) -> Self {
        Self {
            source: into_fun(self.source),
            rule: self.rule,
            target: into_fun(self.target),
        }
    }

    /// `AppAbs` evaluation rule
    pub fn app_abs<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::AppAbs,
            target: target.into(),
        }
    }

    /// Ascription evaluation rule
    pub fn ascribe<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::Ascribe,
            target: target.into(),
        }
    }

    /// Assign evaluation rule
    /// v1:=v2 -> unit
    pub fn assign<V1, V2>(lhs: V1, rhs: V2) -> Self
    where
        V1: Into<Lang::Term>,
        V2: Into<Lang::Term>,
        Unit<Lang>: Into<Lang::Term>,
        Assign<Lang>: Into<Lang::Term>,
    {
        Self {
            source: Assign::new(lhs, rhs).into(),
            rule: EvaluationRule::Assign,
            target: Unit::new().into(),
        }
    }

    /// Cast evaluation rule
    /// v as T -> v
    pub fn cast<Ty, V>(ty: Ty, val: V) -> Self
    where
        Ty: Type + Into<Lang::Type>,
        V: Value + Into<Lang::Term>,
        Cast<Lang>: Into<Lang::Term>,
    {
        Self {
            source: Cast::new(val.clone(), ty).into(),
            rule: EvaluationRule::Cast,
            target: val.into(),
        }
    }

    // Dereference evaluation rule
    pub fn deref<V1, V2>(loc_val: V1, env_val: V2) -> Self
    where
        V2: Into<Lang::Term>,
        V1: Into<Lang::Term>,
        Deref<Lang>: Into<Lang::Term>,
    {
        Self {
            source: Deref::new(loc_val).into(),
            rule: EvaluationRule::Deref,
            target: env_val.into(),
        }
    }

    /// Fold evaluation rule
    pub fn fold<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::Fold,
            target: target.into(),
        }
    }

    /// Tuple first evaluation rule
    pub fn fst<T1, T2>(f: T1, s: T2) -> Self
    where
        T1: Into<Lang::Term> + Clone,
        T2: Into<Lang::Term>,
        Pair<Lang>: Into<Lang::Term>,
    {
        Self {
            source: Pair::new(f.clone(), s).into(),
            rule: EvaluationRule::Fst,
            target: f.into(),
        }
    }

    /// List head evaluation rule
    pub fn head<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::Head,
            target: target.into(),
        }
    }

    /// If true evaluation rule
    pub fn if_true<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::IfTrue,
            target: target.into(),
        }
    }

    /// If false evaluation rule
    pub fn if_false<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::IfFalse,
            target: target.into(),
        }
    }

    /// is-nil for nil rule
    pub fn isnil_true<Ty>(ty: Ty) -> Self
    where
        Ty: Type + Into<Lang::Type>,
        True<Lang>: Into<Lang::Term>,
        IsNil<Lang>: Into<Lang::Term>,
    {
        Self {
            source: IsNil::new(True::new(), ty).into(),
            rule: EvaluationRule::IsNilTrue,
            target: True::new().into(),
        }
    }

    /// is-nil for cons rule
    pub fn isnil_false<Ty>(ty: Ty) -> Self
    where
        Ty: Type + Into<Lang::Type>,
        IsNil<Lang>: Into<Lang::Term>,
        False<Lang>: Into<Lang::Term>,
    {
        Self {
            source: IsNil::new(False::new(), ty).into(),
            rule: EvaluationRule::IsNilFalse,
            target: False::new().into(),
        }
    }

    /// is zero for zero rule
    pub fn iszero_true<T1>(source: T1) -> Self
    where
        T1: Into<Lang::Term>,
        True<Lang>: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::IsZeroTrue,
            target: True::new().into(),
        }
    }

    /// is-zero for nonzero rule
    pub fn iszero_false<T1>(source: T1) -> Self
    where
        T1: Into<Lang::Term>,
        False<Lang>: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::IsZeroFalse,
            target: False::new().into(),
        }
    }

    /// let evaluation rule
    pub fn lett<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::LetSubst,
            target: target.into(),
        }
    }

    /// list case of nil rule
    pub fn listcase_nil<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::ListCaseNil,
            target: target.into(),
        }
    }

    /// list case of cons rule
    pub fn listcase_cons<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::ListCaseCons,
            target: target.into(),
        }
    }

    /// predecessor evaluation rule
    #[must_use]
    pub fn pred(num: i64) -> Self
    where
        Pred<Lang>: Into<Lang::Term>,
        Num<Lang>: Into<Lang::Term>,
    {
        Self {
            source: Pred::new(Num::new(num)).into(),
            rule: EvaluationRule::Pred,
            target: Num::new(num - 1).into(),
        }
    }

    /// projection (tuple) evaluation rule
    pub fn projection<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::Projection,
            target: target.into(),
        }
    }

    /// projection (record) evaluation rule
    pub fn recordproj<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::RecordProj,
            target: target.into(),
        }
    }

    /// reference evaluation rule
    pub fn reft<T1>(source: T1, loc: Location) -> Self
    where
        Loc<Lang>: Into<Lang::Term>,
        T1: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::Ref,
            target: Loc::new(loc).into(),
        }
    }

    /// tuple second evaluation rule
    pub fn snd<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::Snd,
            target: target.into(),
        }
    }

    /// some case of some evaluation rule
    pub fn somecase_some<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::SomecaseSome,
            target: target.into(),
        }
    }

    /// some case of none evaluation rule
    pub fn somecase_none<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::SomecaseNone,
            target: target.into(),
        }
    }

    /// successor evaluation rule
    #[must_use]
    pub fn succ(num: i64) -> Self
    where
        Succ<Lang>: Into<Lang::Term>,
        Num<Lang>: Into<Lang::Term>,
    {
        Self {
            source: Succ::new(Num::new(num)).into(),
            rule: EvaluationRule::Succ,
            target: Num::new(num).into(),
        }
    }

    /// sumcase of inl rule
    pub fn sumcase_left<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::SumcaseLeft,
            target: target.into(),
        }
    }

    /// sumcase of inr rule
    pub fn sumcase_right<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::SumcaseRight,
            target: target.into(),
        }
    }

    /// tail of list rule
    pub fn tail<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::Tail,
            target: target.into(),
        }
    }

    /// evaluate try block when error occurs
    pub fn try_catch<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::TryCatch,
            target: target.into(),
        }
    }

    /// evalute try block when no error occurs
    pub fn try_succ<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::TrySucc,
            target: target.into(),
        }
    }

    /// evaluate tey-catch block with error
    pub fn tryval_catch<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::TryValCatch,
            target: target.into(),
        }
    }

    /// evaluate try-catch block with no error
    pub fn tryval_succ<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::TryValSucc,
            target: target.into(),
        }
    }

    /// evaluate type application with abstraction
    pub fn tyappabs<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::TyAppAbs,
            target: target.into(),
        }
    }

    /// evaluate type application with application (bounded)
    pub fn tyappabs_sub<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::TyAppAbsSub,
            target: target.into(),
        }
    }

    /// evaluate unfold of fold
    pub fn unfoldfold<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::UnfoldFold,
            target: target.into(),
        }
    }

    /// evaluate unpack of pack
    pub fn unpackpack<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::UnpackPack,
            target: target.into(),
        }
    }

    /// evaluate variant case
    pub fn variantcase<T1, T2>(source: T1, target: T2) -> Self
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Self {
            source: source.into(),
            rule: EvaluationRule::VariantCase,
            target: target.into(),
        }
    }

    /// substitute variable (from environment)
    pub fn subst_var<T>(var: &Var, body: T) -> Self
    where
        T: Into<Lang::Term>,
        Variable<Lang>: Into<Lang::Term>,
    {
        Self {
            source: Variable::new(var).into(),
            rule: EvaluationRule::SubstName,
            target: body.into(),
        }
    }
}

impl<Lang> fmt::Display for EvalStep<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -- {} --> {}", self.source, self.rule, self.target)
    }
}
