use check::{errors::CheckError, Kindcheck, Normalize, Subtypecheck, Typecheck};
use derivation::{ProgramDerivation, TypingDerivation};
use eval::Eval;
use latex::{LatexConfig, LatexFmt};
use parse::{errors::ParserError, GroupParse, Parse};
use syntax::{
    program::Program,
    subst::{SubstTerm, SubstType},
    terms::Term,
    types::TypeGroup,
    values::ValueGroup,
};
use trace::EvalTrace;

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
    pub parse_res: Option<Program<Lang::Term, Lang::Type>>,
    pub check_res: Option<ProgramDerivation<Lang::Term, Lang::Type>>,
    pub eval_res: Option<EvalTrace<Lang::Term, Lang::Value>>,
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
        + GroupParse
        + SubstTerm<Self::Term, Target = Self::Term>
        + SubstType<Self::Type, Target = Self::Term>
        + Eval<Term = Self::Term, Value = Self::Value>
        + Typecheck<
            Term = Self::Term,
            Type = Self::Type,
            Deriv = TypingDerivation<<Self as Language>::Term, <Self as Language>::Type>,
        > + LatexFmt;

    type Type: TypeGroup
        + GroupParse
        + SubstType<Self::Type, Target = Self::Type>
        + Subtypecheck<Self::Type>
        + Normalize<Self::Type>
        + Kindcheck<Self::Type>
        + LatexFmt;

    type Value: ValueGroup<Term = Self::Term, Type = Self::Type> + LatexFmt;

    type LanguageError: std::error::Error + From<ParserError> + From<CheckError<Self::Type>>;

    fn parse(&self, input: String) -> Result<Program<Self::Term, Self::Type>, Self::LanguageError> {
        Ok(Program::<Self::Term, Self::Type>::parse(input)?)
    }

    fn check(
        &self,
        input: String,
    ) -> Result<ProgramDerivation<Self::Term, Self::Type>, Self::LanguageError> {
        let parsed = self.parse(input)?;
        Program::<Self::Term, Self::Type>::check_start(&parsed).map_err(|err| err.into())
    }

    fn eval(
        &self,
        input: String,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::LanguageError> {
        let parsed = Self::Term::parse(input)?;
        Self::Term::eval_start(parsed).map_err(|err| err.into())
    }

    fn run_all(&self, input: String) -> AllResults<Self>
    where
        Self: Sized,
    {
        let mut res = AllResults::<Self>::default();
        let parsed = match self.parse(input) {
            Ok(p) => p,
            Err(err) => {
                res.err = Some(err.into());
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
        deriv: &ProgramDerivation<Self::Term, Self::Type>,
        method: &FormatMethod,
    ) -> String {
        match method {
            FormatMethod::Simple => deriv
                .tys()
                .iter()
                .map(|(nm, ty)| format!("{nm}:{ty}"))
                .collect::<Vec<String>>()
                .join("\n"),
            FormatMethod::LatexBus => deriv.to_latex(&mut Default::default()),
            FormatMethod::LatexFrac => deriv.to_latex(&mut LatexConfig::new_frac()),
            FormatMethod::Debug => format!("{:?}", deriv.tys()),
        }
    }

    fn format_prog(&self, prog: &Program<Self::Term, Self::Type>, method: &FormatMethod) -> String {
        match method {
            FormatMethod::Simple => prog.to_string(),
            FormatMethod::LatexBus => prog.to_latex(&mut Default::default()),
            FormatMethod::LatexFrac => prog.to_latex(&mut LatexConfig::new_frac()),
            FormatMethod::Debug => format!("{:?}", prog),
        }
    }

    fn format_trace(
        &self,
        tr: &EvalTrace<Self::Term, Self::Value>,
        method: &FormatMethod,
    ) -> String {
        match method {
            FormatMethod::Simple => tr.val().to_string(),
            FormatMethod::LatexBus => tr.to_latex(&mut Default::default()),
            FormatMethod::LatexFrac => tr.to_latex(&mut LatexConfig::new_frac()),
            FormatMethod::Debug => format!("{:?}", tr.val()),
        }
    }
}
