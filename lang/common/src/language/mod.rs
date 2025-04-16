use crate::{
    check::{CheckEnvironment, Typecheck},
    eval::{Eval, EvalEnvironment},
};

mod terms;
mod types;
mod values;

pub use terms::LanguageTerm;
pub use types::LanguageType;
pub use values::LanguageValue;

pub trait Language {
    type CheckEnv: CheckEnvironment<Type = Self::Type>;
    type EvalEnv: EvalEnvironment;
    type Term: LanguageTerm<Type = Self::Type, Value = Self::Value>
        + Typecheck<Env = Self::CheckEnv, Type = Self::Type>
        + Eval<Value = Self::Value, Env = Self::EvalEnv>;
    type Type: LanguageType;
    type Value: LanguageValue<Term = Self::Term>;
}
