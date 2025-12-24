use crate::{
    BoundedQuantification, Exceptions, Existential, FOmega, FOmegaSub, LambdaOmega, Recursive,
    References, Stlc, Subtypes, SystemF, TypedArithmetic, UntypedArithmetic, UntypedLambda,
};
use check::Typecheck;
use errors::{UndefinedLanguage, language_error::LanguageError};
use eval::Eval;
use latex::LatexFmt;
use parser::GroupParse;
use std::path::PathBuf;
use syntax::language::Language;

mod command;
mod dispatcher;
mod format;
pub use command::Command;
use dispatcher::Dispatcher;
use format::FormatMethod;

pub trait DispatchLanguage {
    fn run_format(
        &mut self,
        cmd: Command,
        source_path: PathBuf,
        method: FormatMethod,
    ) -> Result<String, LanguageError>;
    fn is_lang(&self, lang: &str) -> bool;
}

impl<Lang> DispatchLanguage for Dispatcher<Lang>
where
    Lang: Language + 'static,
    Lang::Term: GroupParse + LatexFmt + Typecheck<Lang = Lang> + Eval<Lang = Lang>,
    Lang::Type: GroupParse + LatexFmt,
    Lang::Value: LatexFmt,
{
    fn run_format(
        &mut self,
        cmd: Command,
        source_path: PathBuf,
        method: FormatMethod,
    ) -> Result<String, LanguageError> {
        match cmd {
            Command::Parse => self.format_parsed(&source_path, method),
            Command::Evaluate => self.format_evaluated(&source_path, method),
            Command::Check => self.format_checked(&source_path, method),
            Command::Grammar => todo!(),
        }
    }

    fn is_lang(&self, lang: &str) -> bool {
        lang.trim() == Lang::id()
    }
}

pub fn create_dispatcher(lang: &str) -> Result<Box<dyn DispatchLanguage>, LanguageError> {
    match lang.to_lowercase().trim() {
        "untyped-arithmetic" => {
            Ok(Box::new(Dispatcher::<UntypedArithmetic>::new()) as Box<dyn DispatchLanguage>)
        }
        "untyped-lambda" => {
            Ok(Box::new(Dispatcher::<UntypedLambda>::new()) as Box<dyn DispatchLanguage>)
        }
        "typed-arithmetic" => {
            Ok(Box::new(Dispatcher::<TypedArithmetic>::new()) as Box<dyn DispatchLanguage>)
        }
        "stlc" => Ok(Box::new(Dispatcher::<Stlc>::new()) as Box<dyn DispatchLanguage>),
        "references" => Ok(Box::new(Dispatcher::<References>::new()) as Box<dyn DispatchLanguage>),
        "exceptions" => Ok(Box::new(Dispatcher::<Exceptions>::new()) as Box<dyn DispatchLanguage>),
        "subtypes" => Ok(Box::new(Dispatcher::<Subtypes>::new()) as Box<dyn DispatchLanguage>),
        "recursive" => Ok(Box::new(Dispatcher::<Recursive>::new()) as Box<dyn DispatchLanguage>),
        "existential" => {
            Ok(Box::new(Dispatcher::<Existential>::new()) as Box<dyn DispatchLanguage>)
        }
        "system-f" => Ok(Box::new(Dispatcher::<SystemF>::new()) as Box<dyn DispatchLanguage>),
        "bounded-quantification" => {
            Ok(Box::new(Dispatcher::<BoundedQuantification>::new()) as Box<dyn DispatchLanguage>)
        }
        "lambda-omega" => {
            Ok(Box::new(Dispatcher::<LambdaOmega>::new()) as Box<dyn DispatchLanguage>)
        }
        "f-omega" => Ok(Box::new(Dispatcher::<FOmega>::new()) as Box<dyn DispatchLanguage>),
        "f-omega-sub" => Ok(Box::new(Dispatcher::<FOmegaSub>::new()) as Box<dyn DispatchLanguage>),
        _ => Err(UndefinedLanguage::new(lang).into()),
    }
}
