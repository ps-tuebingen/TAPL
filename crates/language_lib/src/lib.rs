use check::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use common::parse::Parse;
use derivation::{
    latex::{LatexConfig, LatexFmt},
    Derivation,
};
use eval::{env::EvalEnvironment, values::ValueGroup, Eval};
use syntax::{
    subst::{SubstTerm, SubstType},
    terms::Term,
    types::TypeGroup,
};

pub mod errors;
pub mod languages;

pub use languages::AllLanguages;

#[derive(Default)]
pub enum FormatMethod {
    #[default]
    Simple,
    LatexBus,
    LatexFrac,
    Debug,
}

pub struct AllResults<Lang>
where
    Lang: Language,
{
    pub parse_res: Option<Lang::Term>,
    pub check_res: Option<Derivation<Lang::Term, Lang::Type>>,
    pub eval_res: Option<Lang::Value>,
    pub err: Option<Lang::LanguageError>,
}

impl<Lang> Default for AllResults<Lang>
where
    Lang: Language,
{
    fn default() -> AllResults<Lang> {
        AllResults {
            parse_res: None,
            check_res: None,
            eval_res: None,
            err: None,
        }
    }
}

pub trait Language {
    type Term: Term
        + Parse<ParseError = Self::LanguageError>
        + SubstTerm<Self::Term, Target = Self::Term>
        + SubstType<Self::Type, Target = Self::Term>
        + Eval<
            Env = Self::EvalEnv,
            Value = Self::Value,
            EvalError: Into<<Self as Language>::LanguageError>,
        > + Typecheck<
            Term = Self::Term,
            Type = Self::Type,
            CheckError: Into<<Self as Language>::LanguageError>,
        > + LatexFmt;

    type Type: TypeGroup
        + SubstType<Self::Type, Target = Self::Type>
        + Subtypecheck<Self::Type, CheckError: Into<<Self as Language>::LanguageError>>
        + Normalize<Self::Type>
        + Kindcheck<Self::Type, CheckError: Into<<Self as Language>::LanguageError>>
        + LatexFmt;

    type Value: ValueGroup<Term = Self::Term, Type = Self::Type> + LatexFmt;

    type LanguageError: std::error::Error;

    type EvalEnv: EvalEnvironment<Self::Value>;

    fn parse(&self, input: String) -> Result<Self::Term, Self::LanguageError> {
        Self::Term::parse(input)
    }

    fn check(
        &self,
        input: String,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::LanguageError> {
        let parsed = Self::Term::parse(input)?;
        Self::Term::check_start(&parsed).map_err(|err| err.into())
    }

    fn eval(&self, input: String) -> Result<Self::Value, Self::LanguageError> {
        let parsed = Self::Term::parse(input)?;
        Self::Term::eval_start(parsed).map_err(|err| err.into())
    }

    fn run_all(&self, input: String) -> AllResults<Self>
    where
        Self: Sized,
    {
        let mut res = AllResults::<Self>::default();
        let parsed = match Self::Term::parse(input) {
            Ok(p) => p,
            Err(err) => {
                res.err = Some(err);
                return res;
            }
        };
        res.parse_res = Some(parsed.clone());

        let checked = match parsed.check_start() {
            Ok(ty) => ty,
            Err(err) => {
                res.err = Some(err.into());
                return res;
            }
        };
        res.check_res = Some(checked);

        let evaled = match parsed.eval_start() {
            Ok(v) => v,
            Err(err) => {
                res.err = Some(err.into());
                return res;
            }
        };
        res.eval_res = Some(evaled);
        res
    }

    fn format_derivation(
        &self,
        deriv: &Derivation<Self::Term, Self::Type>,
        method: &FormatMethod,
    ) -> String {
        match method {
            FormatMethod::Simple => deriv.ty().to_string(),
            FormatMethod::LatexBus => deriv.to_latex(&mut Default::default()),
            FormatMethod::LatexFrac => deriv.to_latex(&mut LatexConfig::new_frac()),
            FormatMethod::Debug => format!("{:?}", deriv.ty()),
        }
    }

    fn format_term(&self, term: &Self::Term, method: &FormatMethod) -> String {
        match method {
            FormatMethod::Simple => term.to_string(),
            FormatMethod::LatexBus => term.to_latex(&mut Default::default()),
            FormatMethod::LatexFrac => term.to_latex(&mut LatexConfig::new_frac()),
            FormatMethod::Debug => format!("{:?}", term),
        }
    }

    fn format_value(&self, val: &Self::Value, method: &FormatMethod) -> String {
        match method {
            FormatMethod::Simple => val.to_string(),
            FormatMethod::LatexBus => val.to_latex(&mut Default::default()),
            FormatMethod::LatexFrac => val.to_latex(&mut LatexConfig::new_frac()),
            FormatMethod::Debug => format!("{val:?}"),
        }
    }
}
