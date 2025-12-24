use crate::{
    BoundedQuantification, Exceptions, Existential, FOmega, FOmegaSub, LambdaOmega, Recursive,
    References, Stlc, Subtypes, SystemF, TypedArithmetic, UntypedArithmetic, UntypedLambda,
};
use check::Typecheck;
use derivations::Derivation;
use errors::{FileAccess, UndefinedLanguage, language_error::LanguageError};
use eval::{Eval, eval_main};
use parser::{GroupParse, Parse};
use std::{
    any::Any,
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};
use syntax::{Name, language::Language, program::Program};
use trace::EvalTrace;

pub enum Command {
    Parse,
    Evaluate,
    Check,
    Grammar,
}

pub struct SourceLocation {
    path: PathBuf,
    def_name: Name,
}

#[derive(Clone)]
pub struct Dispatcher<Lang>
where
    Lang: Language,
{
    sources: HashMap<PathBuf, String>,
    parsed: HashMap<PathBuf, Program<Lang>>,
    checked: HashMap<PathBuf, Derivation<Lang>>,
    evaluated: HashMap<PathBuf, EvalTrace<Lang>>,
}

impl<Lang> Dispatcher<Lang>
where
    Lang: Language,
{
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            parsed: HashMap::new(),
            checked: HashMap::new(),
            evaluated: HashMap::new(),
        }
    }

    pub fn source(&mut self, source_path: &Path) -> Result<String, LanguageError> {
        let source_buf = source_path.to_path_buf();
        match self.sources.get(&source_buf) {
            Some(src) => Ok(src.clone()),
            None => {
                let source_contents =
                    read_to_string(source_path).map_err(|err| FileAccess::new("Load file", err))?;
                self.sources.insert(source_buf, source_contents.clone());
                Ok(source_contents)
            }
        }
    }

    pub fn parsed(&mut self, source_path: &Path) -> Result<Program<Lang>, LanguageError>
    where
        Lang::Term: GroupParse,
        Lang::Type: GroupParse,
    {
        match self.parsed.get(source_path) {
            Some(p) => Ok(p.clone()),
            None => {
                let source = self.source(source_path)?;
                let prog = Program::<Lang>::parse(source)?;
                self.parsed.insert(source_path.to_path_buf(), prog.clone());
                Ok(prog)
            }
        }
    }

    pub fn evaluated(&mut self, source_path: &Path) -> Result<EvalTrace<Lang>, LanguageError>
    where
        Lang::Term: GroupParse + Eval<Lang = Lang>,
        Lang::Type: GroupParse,
    {
        let source_buf = source_path.to_path_buf();
        match self.evaluated.get(&source_buf) {
            Some(trace) => Ok(trace.clone()),
            None => {
                let parsed = self.parsed(source_path)?;
                let evaled = eval_main(parsed)?;
                self.evaluated.insert(source_buf, evaled.clone());
                Ok(evaled)
            }
        }
    }

    pub fn checked(&mut self, source_path: &Path) -> Result<Derivation<Lang>, LanguageError>
    where
        Lang::Term: GroupParse + Typecheck<Lang = Lang>,
        Lang::Type: GroupParse,
    {
        let source_buf = source_path.to_path_buf();
        match self.checked.get(&source_buf) {
            Some(checked) => Ok(checked.clone()),
            None => {
                let parsed = self.parsed(source_path)?;
                let checked = parsed.check_start()?;
                self.checked.insert(source_buf, checked.clone());
                Ok(checked)
            }
        }
    }
}

pub trait DispatchLanguage {
    fn run_command(&mut self, cmd: Command, source: PathBuf) -> Box<dyn Any>;
    fn is_lang(&self, lang: &str) -> bool;
}

impl<Lang> DispatchLanguage for Dispatcher<Lang>
where
    Lang: Language + 'static,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn run_command(&mut self, cmd: Command, source: PathBuf) -> Box<dyn Any> {
        todo!()
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
