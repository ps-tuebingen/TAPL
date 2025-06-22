use super::{FormatMethod, Language, errors::UndefinedLanguage};
use std::{fmt, str::FromStr};

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

    pub fn describe(&self) -> &str {
        match self {
            Self::UntypedArithmetic(_) => "Untyped Arithmetic Expressions",
            Self::UntypedLambda(_) => "Untyped Lambda Calculus",
            Self::TypedArithmetic(_) => "Typed Arithmetic Expressions",
            Self::Stlc(_) => "Simply-Typed Lambda Calculus",
            Self::References(_) => "STLC with Referencs",
            Self::Exceptions(_) => "STLC with Exceptions",
            Self::Subtypes(_) => "STLC with Subtyping",
            Self::Recursive(_) => "STLC with Recursive Types",
            Self::Existential(_) => "STLC with Existential Types",
            Self::SystemF(_) => "System F",
            Self::BoundedQuantification(_) => "System F with Bounded Quantification",
            Self::LambdaOmega(_) => "STLC with Higher Kinded Types",
            Self::FOmega(_) => "Higher Kinded System F",
            Self::FOmegaSub(_) => "Higher Kinded System F with Subtyping",
        }
    }

    pub fn parse(
        &self,
        input: String,
        method: &FormatMethod,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::UntypedArithmetic(ua) => {
                let res = ua.parse(input)?;
                Ok(ua.format_term(&res, method))
            }
            Self::UntypedLambda(ul) => {
                let res = ul.parse(input)?;
                Ok(ul.format_term(&res, method))
            }
            Self::TypedArithmetic(ta) => {
                let res = ta.parse(input)?;
                Ok(ta.format_term(&res, method))
            }
            Self::Stlc(stlc) => {
                let res = stlc.parse(input)?;
                Ok(stlc.format_term(&res, method))
            }
            Self::References(rf) => {
                let res = rf.parse(input)?;
                Ok(rf.format_term(&res, method))
            }
            Self::Exceptions(ex) => {
                let res = ex.parse(input)?;
                Ok(ex.format_term(&res, method))
            }
            Self::Subtypes(s) => {
                let res = s.parse(input)?;
                Ok(s.format_term(&res, method))
            }
            Self::Recursive(rec) => {
                let res = rec.parse(input)?;
                Ok(rec.format_term(&res, method))
            }
            Self::Existential(ex) => {
                let res = ex.parse(input)?;
                Ok(ex.format_term(&res, method))
            }
            Self::SystemF(sys) => {
                let res = sys.parse(input)?;
                Ok(sys.format_term(&res, method))
            }
            Self::BoundedQuantification(bd) => {
                let res = bd.parse(input)?;
                Ok(bd.format_term(&res, method))
            }
            Self::LambdaOmega(lo) => {
                let res = lo.parse(input)?;
                Ok(lo.format_term(&res, method))
            }
            Self::FOmega(fo) => {
                let res = fo.parse(input)?;
                Ok(fo.format_term(&res, method))
            }
            Self::FOmegaSub(fos) => {
                let res = fos.parse(input)?;
                Ok(fos.format_term(&res, method))
            }
        }
    }

    pub fn check(
        &self,
        input: String,
        method: &FormatMethod,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::UntypedArithmetic(ua) => {
                let res = ua.check(input)?;
                Ok(ua.format_derivation(&res, method))
            }
            Self::UntypedLambda(ul) => {
                let res = ul.check(input)?;
                Ok(ul.format_derivation(&res, method))
            }
            Self::TypedArithmetic(ta) => {
                let res = ta.check(input)?;
                Ok(ta.format_derivation(&res, method))
            }
            Self::Stlc(stlc) => {
                let res = stlc.check(input)?;
                Ok(stlc.format_derivation(&res, method))
            }
            Self::References(rf) => {
                let res = rf.check(input)?;
                Ok(rf.format_derivation(&res, method))
            }
            Self::Exceptions(ex) => {
                let res = ex.check(input)?;
                Ok(ex.format_derivation(&res, method))
            }
            Self::Subtypes(s) => {
                let res = s.check(input)?;
                Ok(s.format_derivation(&res, method))
            }
            Self::Recursive(rec) => {
                let res = rec.check(input)?;
                Ok(rec.format_derivation(&res, method))
            }
            Self::Existential(ex) => {
                let res = ex.check(input)?;
                Ok(ex.format_derivation(&res, method))
            }
            Self::SystemF(sys) => {
                let res = sys.check(input)?;
                Ok(sys.format_derivation(&res, method))
            }
            Self::BoundedQuantification(bd) => {
                let res = bd.check(input)?;
                Ok(bd.format_derivation(&res, method))
            }
            Self::LambdaOmega(lo) => {
                let res = lo.check(input)?;
                Ok(lo.format_derivation(&res, method))
            }
            Self::FOmega(fo) => {
                let res = fo.check(input)?;
                Ok(fo.format_derivation(&res, method))
            }
            Self::FOmegaSub(fos) => {
                let res = fos.check(input)?;
                Ok(fos.format_derivation(&res, method))
            }
        }
    }

    pub fn eval(
        &self,
        input: String,
        method: &FormatMethod,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::UntypedArithmetic(ua) => {
                let res = ua.eval(input)?;
                Ok(ua.format_trace(&res, method))
            }
            Self::UntypedLambda(ul) => {
                let res = ul.eval(input)?;
                Ok(ul.format_trace(&res, method))
            }
            Self::TypedArithmetic(ta) => {
                let res = ta.eval(input)?;
                Ok(ta.format_trace(&res, method))
            }
            Self::Stlc(stlc) => {
                let res = stlc.eval(input)?;
                Ok(stlc.format_trace(&res, method))
            }
            Self::References(rf) => {
                let res = rf.eval(input)?;
                Ok(rf.format_trace(&res, method))
            }
            Self::Exceptions(ex) => {
                let res = ex.eval(input)?;
                Ok(ex.format_trace(&res, method))
            }
            Self::Subtypes(s) => {
                let res = s.eval(input)?;
                Ok(s.format_trace(&res, method))
            }
            Self::Recursive(rec) => {
                let res = rec.eval(input)?;
                Ok(rec.format_trace(&res, method))
            }
            Self::Existential(ex) => {
                let res = ex.eval(input)?;
                Ok(ex.format_trace(&res, method))
            }
            Self::SystemF(sys) => {
                let res = sys.eval(input)?;
                Ok(sys.format_trace(&res, method))
            }
            Self::BoundedQuantification(bd) => {
                let res = bd.eval(input)?;
                Ok(bd.format_trace(&res, method))
            }
            Self::LambdaOmega(lo) => {
                let res = lo.eval(input)?;
                Ok(lo.format_trace(&res, method))
            }
            Self::FOmega(fo) => {
                let res = fo.eval(input)?;
                Ok(fo.format_trace(&res, method))
            }
            Self::FOmegaSub(fos) => {
                let res = fos.eval(input)?;
                Ok(fos.format_trace(&res, method))
            }
        }
    }

    pub fn run_all(
        &self,
        input: String,
        method: &FormatMethod,
    ) -> (
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    ) {
        match self {
            Self::UntypedArithmetic(ua) => {
                let res = ua.run_all(input);
                (
                    res.parse_res.map(|p| ua.format_term(&p, method)),
                    res.check_res.map(|c| ua.format_derivation(&c, method)),
                    res.eval_res.map(|e| ua.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::UntypedLambda(ul) => {
                let res = ul.run_all(input);
                (
                    res.parse_res.map(|p| ul.format_term(&p, method)),
                    res.check_res.map(|c| ul.format_derivation(&c, method)),
                    res.eval_res.map(|e| ul.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::TypedArithmetic(ta) => {
                let res = ta.run_all(input);
                (
                    res.parse_res.map(|p| ta.format_term(&p, method)),
                    res.check_res.map(|c| ta.format_derivation(&c, method)),
                    res.eval_res.map(|e| ta.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::Stlc(stlc) => {
                let res = stlc.run_all(input);
                (
                    res.parse_res.map(|p| stlc.format_term(&p, method)),
                    res.check_res.map(|c| stlc.format_derivation(&c, method)),
                    res.eval_res.map(|e| stlc.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::References(rf) => {
                let res = rf.run_all(input);
                (
                    res.parse_res.map(|p| rf.format_term(&p, method)),
                    res.check_res.map(|c| rf.format_derivation(&c, method)),
                    res.eval_res.map(|e| rf.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::Exceptions(ex) => {
                let res = ex.run_all(input);
                (
                    res.parse_res.map(|p| ex.format_term(&p, method)),
                    res.check_res.map(|c| ex.format_derivation(&c, method)),
                    res.eval_res.map(|e| ex.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::Subtypes(s) => {
                let res = s.run_all(input);
                (
                    res.parse_res.map(|p| s.format_term(&p, method)),
                    res.check_res.map(|c| s.format_derivation(&c, method)),
                    res.eval_res.map(|e| s.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::Recursive(rec) => {
                let res = rec.run_all(input);
                (
                    res.parse_res.map(|p| rec.format_term(&p, method)),
                    res.check_res.map(|c| rec.format_derivation(&c, method)),
                    res.eval_res.map(|e| rec.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::Existential(ex) => {
                let res = ex.run_all(input);
                (
                    res.parse_res.map(|p| ex.format_term(&p, method)),
                    res.check_res.map(|c| ex.format_derivation(&c, method)),
                    res.eval_res.map(|e| ex.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::SystemF(sys) => {
                let res = sys.run_all(input);
                (
                    res.parse_res.map(|p| sys.format_term(&p, method)),
                    res.check_res.map(|c| sys.format_derivation(&c, method)),
                    res.eval_res.map(|e| sys.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::BoundedQuantification(bd) => {
                let res = bd.run_all(input);
                (
                    res.parse_res.map(|p| bd.format_term(&p, method)),
                    res.check_res.map(|c| bd.format_derivation(&c, method)),
                    res.eval_res.map(|e| bd.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::LambdaOmega(lo) => {
                let res = lo.run_all(input);
                (
                    res.parse_res.map(|p| lo.format_term(&p, method)),
                    res.check_res.map(|c| lo.format_derivation(&c, method)),
                    res.eval_res.map(|e| lo.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::FOmega(fo) => {
                let res = fo.run_all(input);
                (
                    res.parse_res.map(|p| fo.format_term(&p, method)),
                    res.check_res.map(|c| fo.format_derivation(&c, method)),
                    res.eval_res.map(|e| fo.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
            Self::FOmegaSub(fos) => {
                let res = fos.run_all(input);
                (
                    res.parse_res.map(|p| fos.format_term(&p, method)),
                    res.check_res.map(|c| fos.format_derivation(&c, method)),
                    res.eval_res.map(|e| fos.format_trace(&e, method)),
                    res.err.map(|e| e.to_string()),
                )
            }
        }
    }
}

impl FromStr for AllLanguages {
    type Err = UndefinedLanguage;
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
            _ => Err(UndefinedLanguage::new(s)),
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
