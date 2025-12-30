use crate::{
    BoundedQuantification, Exceptions, Existential, FOmega, FOmegaSub, LambdaOmega, Recursive,
    References, Stlc, Subtypes, SystemF, TypedArithmetic, UntypedArithmetic, UntypedLambda,
};
use check::Typecheck;
use errors::{UndefinedLanguage, language_error::LanguageError};
use eval::Eval;
use grammar::LanguageDescribe;
use inference::{GenerateConstraints, SolveConstraint};
use latex::LatexFmt;
use parser::GroupParse;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use syntax::language::{Language, LanguageFeatures};

mod command;
mod dispatcher;
mod format;
pub use command::Command;
use dispatcher::Dispatcher;
pub use format::FormatMethod;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Source {
    Path(PathBuf),
    Str(String),
}

pub trait DispatchLanguage {
    /// Run a given command with method and source
    /// # Errors
    /// returns an error if there was an error while running the command
    fn run_format(
        &mut self,
        source: Source,
        cmd: Command,
        method: FormatMethod,
    ) -> Result<String, LanguageError>;

    fn is_lang(&self, lang: &str) -> bool;

    fn features(&self) -> LanguageFeatures;

    fn id(&self) -> &str;

    fn describe(&self) -> &str;

    /// Run all possible commands with given source and format method
    /// # Errors
    /// Returns an error if any one of the commands returns an error
    fn run_all(
        &mut self,
        source: Source,
        method: FormatMethod,
    ) -> Result<HashMap<Command, String>, LanguageError> {
        let mut results = HashMap::new();
        for cmd in Command::all() {
            let cmd_res = self.run_format(source.clone(), cmd, method)?;
            results.insert(cmd, cmd_res);
        }
        Ok(results)
    }
}

impl<Lang> DispatchLanguage for Dispatcher<Lang>
where
    Lang: Language + LanguageDescribe + 'static,
    Lang::Term: GroupParse
        + LatexFmt
        + Typecheck<Lang = Lang>
        + Eval<Lang = Lang>
        + GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Lang::Type:
        GroupParse + LatexFmt + GenerateConstraints<Lang = Lang> + SolveConstraint<Lang = Lang>,
    Lang::Value: LatexFmt,
{
    fn run_format(
        &mut self,
        source: Source,
        cmd: Command,
        method: FormatMethod,
    ) -> Result<String, LanguageError> {
        match cmd {
            Command::Parse => self.format_parsed(source, method),
            Command::Evaluate => self.format_evaluated(source, method),
            Command::Check => self.format_checked(source, method),
            Command::Grammar => Ok(self.format_grammar(method)),
            Command::Infer => self.format_infer(source, method),
        }
    }

    fn is_lang(&self, lang: &str) -> bool {
        lang.trim() == Lang::id()
    }

    fn features(&self) -> LanguageFeatures {
        Lang::features()
    }

    fn id(&self) -> &str {
        Lang::id()
    }

    fn describe(&self) -> &str {
        Lang::describe()
    }
}

/// Create a new dispatcher from a given language as a string
/// # Errors
/// Returns an error if the language could not be parsed
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

impl From<&str> for Source {
    fn from(s: &str) -> Self {
        Self::Str(s.to_string())
    }
}

impl From<String> for Source {
    fn from(s: String) -> Self {
        Self::Str(s)
    }
}

impl From<&Path> for Source {
    fn from(p: &Path) -> Self {
        Self::Path(p.to_path_buf())
    }
}
