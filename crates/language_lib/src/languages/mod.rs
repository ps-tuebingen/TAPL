use super::errors::LanguageError;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllLanguages {
    UntypedArithmetic(untyped_arithmetic::UntypedArithmetic),
    UntypedLambda(untyped_lambda::UntypedLambda),
    TypedArithmetic(typed_arithmetic::TypedArithmetic),
    Stlc(stlc::Stlc),
    Exceptions(exceptions::Exceptions),
    References(references::References),
    Existential(existential::Existential),
    Recursive(recursive::Recursive),
    Subtypes(subtypes::Subtypes),
    SystemF(system_f::SystemF),
    BoundedQuantification(bounded_quantification::BoundedQuantification),
    LambdaOmega(lambda_omega::LambdaOmega),
    FOmega(f_omega::FOmega),
    FOmegaSub(f_omega_sub::FOmegaSub),
}

impl AllLanguages {
    pub fn all() -> [AllLanguages; 14] {
        [
            untyped_arithmetic::UntypedArithmetic.into(),
            untyped_lambda::UntypedLambda.into(),
            typed_arithmetic::TypedArithmetic.into(),
            stlc::Stlc.into(),
            references::References.into(),
            exceptions::Exceptions.into(),
            subtypes::Subtypes.into(),
            recursive::Recursive.into(),
            existential::Existential.into(),
            system_f::SystemF.into(),
            bounded_quantification::BoundedQuantification.into(),
            lambda_omega::LambdaOmega.into(),
            f_omega::FOmega.into(),
            f_omega_sub::FOmegaSub.into(),
        ]
    }
}

impl FromStr for AllLanguages {
    type Err = LanguageError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "untyped-arithmetic" => Ok(untyped_arithmetic::UntypedArithmetic.into()),
            "untyped-lambda" => Ok(untyped_lambda::UntypedLambda.into()),
            "typed-arithmetic" => Ok(typed_arithmetic::TypedArithmetic.into()),
            "stlc" => Ok(stlc::Stlc.into()),
            "references" => Ok(references::References.into()),
            "exceptions" => Ok(exceptions::Exceptions.into()),
            "subtypes" => Ok(subtypes::Subtypes.into()),
            "recursive" => Ok(recursive::Recursive.into()),
            "existential" => Ok(existential::Existential.into()),
            "system-f" => Ok(system_f::SystemF.into()),
            "bounded-quantification" => Ok(bounded_quantification::BoundedQuantification.into()),
            "lambda-omega" => Ok(lambda_omega::LambdaOmega.into()),
            "f-omega" => Ok(f_omega::FOmega.into()),
            "f-omega-sub" => Ok(f_omega_sub::FOmegaSub.into()),
            _ => Err(LanguageError::UndefinedLanguage(s.to_owned())),
        }
    }
}

impl fmt::Display for AllLanguages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UntypedArithmetic(_) => f.write_str("untyped-arithmetic"),
            Self::UntypedLambda(_) => f.write_str("untyped-lambda"),
            Self::TypedArithmetic(_) => f.write_str("typed-arithmetic"),
            Self::Stlc(_) => f.write_str("stlc"),
            Self::References(_) => f.write_str("references"),
            Self::Exceptions(_) => f.write_str("exceptions"),
            Self::Subtypes(_) => f.write_str("subtypes"),
            Self::Recursive(_) => f.write_str("recursive"),
            Self::Existential(_) => f.write_str("existential"),
            Self::SystemF(_) => f.write_str("system-f"),
            Self::BoundedQuantification(_) => f.write_str("bounded-quantification"),
            Self::LambdaOmega(_) => f.write_str("lambda-omega"),
            Self::FOmega(_) => f.write_str("f-omega"),
            Self::FOmegaSub(_) => f.write_str("f-omega-sub"),
        }
    }
}

impl From<untyped_arithmetic::UntypedArithmetic> for AllLanguages {
    fn from(untyped_arith: untyped_arithmetic::UntypedArithmetic) -> AllLanguages {
        AllLanguages::UntypedArithmetic(untyped_arith)
    }
}

impl From<untyped_lambda::UntypedLambda> for AllLanguages {
    fn from(untyped_lambda: untyped_lambda::UntypedLambda) -> AllLanguages {
        AllLanguages::UntypedLambda(untyped_lambda)
    }
}

impl From<typed_arithmetic::TypedArithmetic> for AllLanguages {
    fn from(typed_arith: typed_arithmetic::TypedArithmetic) -> AllLanguages {
        AllLanguages::TypedArithmetic(typed_arith)
    }
}

impl From<stlc::Stlc> for AllLanguages {
    fn from(stlc: stlc::Stlc) -> AllLanguages {
        AllLanguages::Stlc(stlc)
    }
}

impl From<references::References> for AllLanguages {
    fn from(refs: references::References) -> AllLanguages {
        AllLanguages::References(refs)
    }
}

impl From<exceptions::Exceptions> for AllLanguages {
    fn from(exc: exceptions::Exceptions) -> AllLanguages {
        AllLanguages::Exceptions(exc)
    }
}
impl From<subtypes::Subtypes> for AllLanguages {
    fn from(subt: subtypes::Subtypes) -> AllLanguages {
        AllLanguages::Subtypes(subt)
    }
}

impl From<recursive::Recursive> for AllLanguages {
    fn from(rec: recursive::Recursive) -> AllLanguages {
        AllLanguages::Recursive(rec)
    }
}

impl From<existential::Existential> for AllLanguages {
    fn from(exist: existential::Existential) -> AllLanguages {
        AllLanguages::Existential(exist)
    }
}

impl From<system_f::SystemF> for AllLanguages {
    fn from(sysf: system_f::SystemF) -> AllLanguages {
        AllLanguages::SystemF(sysf)
    }
}
impl From<bounded_quantification::BoundedQuantification> for AllLanguages {
    fn from(bound: bounded_quantification::BoundedQuantification) -> AllLanguages {
        AllLanguages::BoundedQuantification(bound)
    }
}

impl From<lambda_omega::LambdaOmega> for AllLanguages {
    fn from(lamo: lambda_omega::LambdaOmega) -> AllLanguages {
        AllLanguages::LambdaOmega(lamo)
    }
}

impl From<f_omega::FOmega> for AllLanguages {
    fn from(fomega: f_omega::FOmega) -> AllLanguages {
        AllLanguages::FOmega(fomega)
    }
}

impl From<f_omega_sub::FOmegaSub> for AllLanguages {
    fn from(fomegasub: f_omega_sub::FOmegaSub) -> AllLanguages {
        AllLanguages::FOmegaSub(fomegasub)
    }
}

pub mod bounded_quantification;
pub mod exceptions;
pub mod existential;
pub mod f_omega;
pub mod f_omega_sub;
pub mod lambda_omega;
pub mod recursive;
pub mod references;
pub mod stlc;
pub mod subtypes;
pub mod system_f;
pub mod typed_arithmetic;
pub mod untyped_arithmetic;
pub mod untyped_lambda;

pub use bounded_quantification::BoundedQuantification;
pub use exceptions::Exceptions;
pub use existential::Existential;
pub use f_omega::FOmega;
pub use f_omega_sub::FOmegaSub;
pub use lambda_omega::LambdaOmega;
pub use recursive::Recursive;
pub use references::References;
pub use stlc::Stlc;
pub use subtypes::Subtypes;
pub use system_f::SystemF;
pub use typed_arithmetic::TypedArithmetic;
pub use untyped_arithmetic::UntypedArithmetic;
pub use untyped_lambda::UntypedLambda;

#[cfg(test)]
mod lang_tests {
    use super::AllLanguages;
    use std::str::FromStr;

    #[test]
    fn from_str_to_str() {
        for lang in AllLanguages::all() {
            let from_to = AllLanguages::from_str(&lang.to_string()).unwrap();
            assert_eq!(lang, from_to)
        }
    }
}
