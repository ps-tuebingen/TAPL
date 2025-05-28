use check::{env::CheckEnvironment, Kindcheck, Normalize, Subtypecheck, Typecheck};
use common::parse::Parse;
use eval::{env::EvalEnvironment, values::ValueGroup, Eval};
use syntax::{
    subst::{SubstTerm, SubstType},
    terms::Term,
    types::TypeGroup,
};

pub mod languages;

pub trait Language {
    type Term: Term
        + Parse
        + SubstTerm<Self::Term, Target = Self::Term>
        + SubstType<Self::Type, Target = Self::Term>
        + Eval<Env = Self::EvalEnv, Value = Self::Value>
        + Typecheck<Type = Self::Type, Env = Self::CheckEnv>;

    type Type: TypeGroup
        + SubstType<Self::Type, Target = Self::Type>
        + Subtypecheck<Self::Type, Env = Self::CheckEnv>
        + Normalize<Self::Type, Env = Self::CheckEnv>
        + Kindcheck<Self::Type, Env = Self::CheckEnv>;

    type Value: ValueGroup<Term = Self::Term, Type = Self::Type>;

    type CheckEnv: CheckEnvironment<Type = Self::Type>;

    type EvalEnv: EvalEnvironment<Self::Value>;
}
