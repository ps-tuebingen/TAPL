use crate::{Driver, cli::Command, format::FormatMethod};
use errors::driver_error::DriverError;
use language::languages::{
    BoundedQuantification, Exceptions, Existential, FOmega, FOmegaSub, LambdaOmega, Recursive,
    References, Stlc, Subtypes, SystemF, TypedArithmetic, UntypedArithmetic, UntypedLambda,
};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllLanguages {
    UntypedArithmetic,
    UntypedLambda,
    TypedArithmetic,
    Stlc,
    Exceptions,
    References,
    Existential,
    Recursive,
    Subtypes,
    SystemF,
    BoundedQuantification,
    LambdaOmega,
    FOmega,
    FOmegaSub,
}

impl AllLanguages {
    pub fn dispatch_run(
        &self,
        driver: &Driver,
        method: &FormatMethod,
        cmd: &Command,
        input: String,
    ) -> Result<String, DriverError> {
        match self {
            AllLanguages::UntypedArithmetic => {
                driver.run_format::<UntypedArithmetic>(method, cmd, input)
            }
            AllLanguages::UntypedLambda => driver.run_format::<UntypedLambda>(method, cmd, input),
            AllLanguages::TypedArithmetic => {
                driver.run_format::<TypedArithmetic>(method, cmd, input)
            }
            AllLanguages::Stlc => driver.run_format::<Stlc>(method, cmd, input),
            AllLanguages::Exceptions => driver.run_format::<Exceptions>(method, cmd, input),
            AllLanguages::References => driver.run_format::<References>(method, cmd, input),
            AllLanguages::Existential => driver.run_format::<Existential>(method, cmd, input),
            AllLanguages::Recursive => driver.run_format::<Recursive>(method, cmd, input),
            AllLanguages::Subtypes => driver.run_format::<Subtypes>(method, cmd, input),
            AllLanguages::SystemF => driver.run_format::<SystemF>(method, cmd, input),
            AllLanguages::BoundedQuantification => {
                driver.run_format::<BoundedQuantification>(method, cmd, input)
            }
            AllLanguages::LambdaOmega => driver.run_format::<LambdaOmega>(method, cmd, input),
            AllLanguages::FOmega => driver.run_format::<FOmega>(method, cmd, input),
            AllLanguages::FOmegaSub => driver.run_format::<FOmegaSub>(method, cmd, input),
        }
    }
}

impl FromStr for AllLanguages {
    type Err = DriverError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "untyped-arithmetic" => Ok(AllLanguages::UntypedArithmetic),
            "untyped-lambda" => Ok(AllLanguages::UntypedLambda),
            "typed-arithmetic" => Ok(AllLanguages::TypedArithmetic),
            "stlc" => Ok(AllLanguages::Stlc),
            "references" => Ok(AllLanguages::References),
            "exceptions" => Ok(AllLanguages::Exceptions),
            "subtypes" => Ok(AllLanguages::Subtypes),
            "recursive" => Ok(AllLanguages::Recursive),
            "existential" => Ok(AllLanguages::Existential),
            "system-f" => Ok(AllLanguages::SystemF),
            "bounded-quantification" => Ok(AllLanguages::BoundedQuantification),
            "lambda-omega" => Ok(AllLanguages::LambdaOmega),
            "f-omega" => Ok(AllLanguages::FOmega),
            "f-omega-sub" => Ok(AllLanguages::FOmegaSub),
            _ => Err(DriverError::UndefinedLanguage(s.to_owned())),
        }
    }
}

impl fmt::Display for AllLanguages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UntypedArithmetic => f.write_str("untyped-arithmetic"),
            Self::UntypedLambda => f.write_str("untyped-lambda"),
            Self::TypedArithmetic => f.write_str("typed-arithmetic"),
            Self::Stlc => f.write_str("stlc"),
            Self::References => f.write_str("references"),
            Self::Exceptions => f.write_str("exceptions"),
            Self::Subtypes => f.write_str("subtypes"),
            Self::Recursive => f.write_str("recursive"),
            Self::Existential => f.write_str("existential"),
            Self::SystemF => f.write_str("system-f"),
            Self::BoundedQuantification => f.write_str("bounded-quantification"),
            Self::LambdaOmega => f.write_str("lambda-omega"),
            Self::FOmega => f.write_str("f-omega"),
            Self::FOmegaSub => f.write_str("f-omega-sub"),
        }
    }
}
