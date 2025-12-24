use super::format::FormatMethod;
use check::Typecheck;
use derivations::Derivation;
use errors::{FileAccess, language_error::LanguageError};
use eval::{Eval, eval_main};
use grammar::{LanguageDescribe, LanguageGrammar};
use latex::LatexFmt;
use parser::{GroupParse, Parse};
use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};
use syntax::{language::Language, program::Program};
use trace::EvalTrace;

#[derive(Clone)]
pub struct Dispatcher<Lang>
where
    Lang: Language,
{
    sources: HashMap<PathBuf, String>,
    parsed: HashMap<PathBuf, Program<Lang>>,
    checked: HashMap<PathBuf, Derivation<Lang>>,
    evaluated: HashMap<PathBuf, EvalTrace<Lang>>,
    grammar: Option<LanguageGrammar>,
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
            grammar: None,
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

    pub fn format_parsed(
        &mut self,
        source_path: &Path,
        method: FormatMethod,
    ) -> Result<String, LanguageError>
    where
        Lang::Term: GroupParse + LatexFmt,
        Lang::Type: GroupParse + LatexFmt,
    {
        let parsed = self.parsed(source_path)?;
        Ok(method.format(&parsed))
    }

    pub fn format_checked(
        &mut self,
        source_path: &Path,
        method: FormatMethod,
    ) -> Result<String, LanguageError>
    where
        Lang::Term: GroupParse + Typecheck<Lang = Lang> + LatexFmt,
        Lang::Type: GroupParse + LatexFmt,
    {
        let checked = self.checked(source_path)?;
        Ok(method.format(&checked))
    }

    pub fn format_evaluated(
        &mut self,
        source_path: &Path,
        method: FormatMethod,
    ) -> Result<String, LanguageError>
    where
        Lang::Term: GroupParse + Eval<Lang = Lang> + LatexFmt,
        Lang::Type: GroupParse,
        Lang::Value: LatexFmt,
    {
        let evaluated = self.evaluated(source_path)?;
        Ok(method.format(&evaluated))
    }

    pub fn format_grammar(&mut self, source_path: &Path, method: FormatMethod) -> String
    where
        Lang: LanguageDescribe,
    {
        if let Some(gram) = &self.grammar {
            method.format(gram)
        } else {
            let grammar = Lang::grammars();
            let res = method.format(&grammar);
            self.grammar = Some(grammar);
            res
        }
    }
}
