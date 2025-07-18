use check::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use derivation::{ProgramDerivation, TypingDerivation};
use eval::{Eval, eval_main};
use grammar::GrammarDescribe;
use grammar::LanguageDescribe;
use latex::{LatexConfig, LatexFmt};
use parse::{GroupParse, Parse};
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

use errors::LanguageError;
pub use languages::AllLanguages;

#[derive(Default)]
pub enum FormatMethod {
    #[default]
    Simple,
    LatexBusStripped,
    LatexBusDoc,
    LatexFracStripped,
    LatexFracDoc,
    Debug,
}

pub struct AllResults<Lang>
where
    Lang: Language,
{
    pub parse_res: Option<Program<Lang::Term, Lang::Type>>,
    pub check_res: Option<ProgramDerivation<Lang::Term, Lang::Type>>,
    pub eval_res: Option<EvalTrace<Lang::Term, Lang::Value>>,
    pub err: Option<LanguageError>,
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

/*pub trait Language: LanguageDescribe {
    type Term: Term
        + GroupParse
        + SubstTerm<Self::Term, Target = Self::Term>
        + SubstType<Self::Type, Target = Self::Term>
        + Eval<Term = Self::Term, Value = Self::Value>
        + Typecheck<
            Term = Self::Term,
            Type = Self::Type,
            Deriv = TypingDerivation<<Self as Language>::Term, <Self as Language>::Type>,
        > + GrammarDescribe
        + LatexFmt;

    type Type: TypeGroup
        + GroupParse
        + SubstType<Self::Type, Target = Self::Type>
        + Subtypecheck<Self::Type>
        + Normalize<Self::Type>
        + Kindcheck<Self::Type>
        + GrammarDescribe
        + LatexFmt;

    type Value: ValueGroup<Term = Self::Term, Type = Self::Type> + GrammarDescribe + LatexFmt;

    fn parse(&self, input: String) -> Result<Program<Self::Term, Self::Type>, LanguageError> {
        Ok(Program::<Self::Term, Self::Type>::parse(input)?)
    }

    fn check(
        &self,
        input: String,
    ) -> Result<ProgramDerivation<Self::Term, Self::Type>, LanguageError> {
        let parsed = self.parse(input)?;
        Program::<Self::Term, Self::Type>::check_start(&parsed).map_err(|err| err.into())
    }

    fn eval(&self, input: String) -> Result<EvalTrace<Self::Term, Self::Value>, LanguageError> {
        let parsed = self.parse(input)?;
        eval_main(parsed).map_err(|err| err.into())
    }

    fn run_all(&self, input: String) -> AllResults<Self>
    where
        Self: Sized,
    {
        let mut res = AllResults::<Self>::default();
        let parsed = match self.parse(input) {
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

        let evaled = match eval_main(parsed) {
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
            FormatMethod::LatexBusStripped => deriv.to_latex(&mut Default::default()),
            FormatMethod::LatexBusDoc => deriv.to_document(&mut Default::default()),
            FormatMethod::LatexFracStripped => deriv.to_latex(&mut LatexConfig::new_frac()),
            FormatMethod::LatexFracDoc => deriv.to_document(&mut LatexConfig::new_frac()),
            FormatMethod::Debug => format!("{:?}", deriv.tys()),
        }
    }

    fn format_prog(&self, prog: &Program<Self::Term, Self::Type>, method: &FormatMethod) -> String {
        match method {
            FormatMethod::Simple => prog.to_string(),
            FormatMethod::LatexBusStripped => prog.to_latex(&mut Default::default()),
            FormatMethod::LatexBusDoc => prog.to_document(&mut Default::default()),
            FormatMethod::LatexFracStripped => prog.to_latex(&mut LatexConfig::new_frac()),
            FormatMethod::LatexFracDoc => prog.to_document(&mut LatexConfig::new_frac()),
            FormatMethod::Debug => format!("{prog:?}",),
        }
    }

    fn format_trace(
        &self,
        tr: &EvalTrace<Self::Term, Self::Value>,
        method: &FormatMethod,
    ) -> String {
        match method {
            FormatMethod::Simple => tr.val().to_string(),
            FormatMethod::LatexBusStripped => tr.to_latex(&mut Default::default()),
            FormatMethod::LatexBusDoc => tr.to_document(&mut Default::default()),
            FormatMethod::LatexFracStripped => tr.to_latex(&mut LatexConfig::new_frac()),
            FormatMethod::LatexFracDoc => tr.to_document(&mut Default::default()),
            FormatMethod::Debug => format!("{:?}", tr.val()),
        }
    }

    fn lang_grammar(&self, method: &FormatMethod) -> String {
        let grammars = Self::grammars();
        match method {
            FormatMethod::Simple => grammars.to_string(),
            FormatMethod::LatexBusStripped => grammars.to_latex(&mut Default::default()),
            FormatMethod::LatexBusDoc => grammars.to_document(&mut Default::default()),
            FormatMethod::LatexFracStripped => grammars.to_latex(&mut LatexConfig::new_frac()),
            FormatMethod::LatexFracDoc => grammars.to_document(&mut Default::default()),
            FormatMethod::Debug => format!("{grammars:?}",),
        }
    }
}*/
