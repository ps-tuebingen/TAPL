use crate::{
    check::{CheckEnvironment, Subtypecheck, Typecheck},
    eval::{Eval, EvalEnvironment},
};

mod terms;
mod types;
pub mod untyped;
mod values;

pub use terms::LanguageTerm;
pub use types::LanguageType;
pub use values::LanguageValue;

pub trait Language {
    type Term: LanguageTerm<Type = Self::Type, Value = Self::Value>
        + Typecheck<Env = Self::CheckEnv, Type = Self::Type>
        + Eval<Value = Self::Value, Env = Self::EvalEnv>;
    type Type: LanguageType + Subtypecheck<Self::Type, Env = Self::CheckEnv>;
    type Value: LanguageValue<Term = Self::Term>;

    type CheckEnv: CheckEnvironment<Type = Self::Type>;
    type EvalEnv: EvalEnvironment<Self::Value>;
}
