use check::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use common::parse::Parse;
use eval::{env::EvalEnvironment, values::ValueGroup, Eval};
use syntax::{
    subst::{SubstTerm, SubstType},
    terms::Term,
    types::TypeGroup,
};

pub mod errors;
pub mod languages;

pub use languages::AllLanguages;

pub trait Language {
    type Term: Term
        + Parse<ParseError = Self::LanguageError>
        + SubstTerm<Self::Term, Target = Self::Term>
        + SubstType<Self::Type, Target = Self::Term>
        + Eval<
            Env = Self::EvalEnv,
            Value = Self::Value,
            EvalError: Into<<Self as Language>::LanguageError>,
        > + Typecheck<Type = Self::Type, CheckError: Into<<Self as Language>::LanguageError>>;

    type Type: TypeGroup
        + SubstType<Self::Type, Target = Self::Type>
        + Subtypecheck<Self::Type, CheckError: Into<<Self as Language>::LanguageError>>
        + Normalize<Self::Type>
        + Kindcheck<Self::Type, CheckError: Into<<Self as Language>::LanguageError>>;

    type Value: ValueGroup<Term = Self::Term, Type = Self::Type>;

    type LanguageError: std::error::Error;

    type EvalEnv: EvalEnvironment<Self::Value>;
}
