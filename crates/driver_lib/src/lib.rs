use check::Typecheck;
use derivation::{ProgramDerivation, TypingDerivation};
use eval::{Eval, eval_main};
use parse::{GroupParse, Parse, errors::ParserError};
use syntax::{language::Language, program::Program};
use trace::EvalTrace;

mod format;

pub struct Driver;

impl Driver {
    pub fn parse<L>(&self, input: String) -> Result<Program<L::Term, L::Type>, ParserError>
    where
        L: Language,
        L::Term: GroupParse,
        L::Type: GroupParse,
    {
        Program::<L::Term, L::Type>::parse(input)
    }

    pub fn check<L>(
        &self,
        input: String,
    ) -> Result<ProgramDerivation<L::Term, L::Type>, Box<dyn std::error::Error>>
    where
        L: Language,
        L::Term: GroupParse
            + Typecheck<Term = L::Term, Type = L::Type, Deriv = TypingDerivation<L::Term, L::Type>>,
        L::Type: GroupParse,
    {
        let parsed = self.parse::<L>(input)?;
        parsed
            .check_start()
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
    }

    pub fn eval<L>(
        &self,
        input: String,
    ) -> Result<EvalTrace<L::Term, L::Value>, Box<dyn std::error::Error>>
    where
        L: Language,
        L::Term: GroupParse + Eval<Term = L::Term, Value = L::Value>,
        L::Type: GroupParse,
    {
        let parsed = self.parse::<L>(input)?;
        eval_main(parsed).map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
    }
}

/*impl FromStr for AllLanguages {
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
}*/
