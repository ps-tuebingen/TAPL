use check::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use derivation::{ProgramDerivation, TypingDerivation};
use eval::{Eval, eval_main};
use grammar::GrammarDescribe;
use grammar::LanguageDescribe;
use latex::{LatexConfig, LatexFmt};
use parse::{GroupParse, Parse};
use syntax::{
    language::Language,
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
