use super::{Source, format::FormatMethod};
use check::Typecheck;
use derivations::Derivation;
use errors::{FileAccess, language_error::LanguageError};
use eval::{Eval, eval_main};
use grammar::{LanguageDescribe, LanguageGrammar};
use inference::{GenerateConstraints, ProgTypes, SolveConstraint, infer_types};
use latex::LatexFmt;
use parser::{GroupParse, Parse};
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};
use syntax::{language::Language, program::Program};
use trace::EvalTrace;

#[derive(Clone)]
pub struct Dispatcher<Lang>
where
    Lang: Language,
{
    sources: HashMap<PathBuf, String>,
    parsed: HashMap<Source, Program<Lang>>,
    checked: HashMap<Source, Derivation<Lang>>,
    evaluated: HashMap<Source, EvalTrace<Lang>>,
    inferred: HashMap<Source, ProgTypes<Lang>>,
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
            inferred: HashMap::new(),
        }
    }

    pub fn source(&mut self, source: Source) -> Result<String, LanguageError> {
        let source_path = match source {
            Source::Str(s) => return Ok(s),
            Source::Path(p) => p,
        };
        if let Some(src) = self.sources.get(&source_path) {
            return Ok(src.clone());
        }

        let source_contents =
            read_to_string(&source_path).map_err(|err| FileAccess::new("Load file", err))?;
        self.sources.insert(source_path, source_contents.clone());
        Ok(source_contents)
    }

    pub fn parsed(&mut self, source: Source) -> Result<Program<Lang>, LanguageError>
    where
        Lang::Term: GroupParse,
        Lang::Type: GroupParse,
    {
        if let Some(p) = self.parsed.get(&source) {
            return Ok(p.clone());
        }

        let source_str = self.source(source.clone())?;
        let prog = Program::<Lang>::parse(source_str)?;
        self.parsed.insert(source, prog.clone());
        Ok(prog)
    }

    pub fn evaluated(&mut self, source: Source) -> Result<EvalTrace<Lang>, LanguageError>
    where
        Lang::Term: GroupParse + Eval<Lang = Lang>,
        Lang::Type: GroupParse,
    {
        if let Some(tr) = self.evaluated.get(&source) {
            return Ok(tr.clone());
        }
        let parsed = self.parsed(source.clone())?;
        let evaled = eval_main(parsed)?;
        self.evaluated.insert(source, evaled.clone());
        Ok(evaled)
    }

    pub fn checked(&mut self, source: Source) -> Result<Derivation<Lang>, LanguageError>
    where
        Lang::Term: GroupParse + Typecheck<Lang = Lang>,
        Lang::Type: GroupParse,
    {
        if let Some(ch) = self.checked.get(&source) {
            return Ok(ch.clone());
        }

        let parsed = self.parsed(source.clone())?;
        let checked = parsed.check_start()?;
        self.checked.insert(source, checked.clone());
        Ok(checked)
    }

    pub fn format_parsed(
        &mut self,
        source: Source,
        method: FormatMethod,
    ) -> Result<String, LanguageError>
    where
        Lang::Term: GroupParse + LatexFmt,
        Lang::Type: GroupParse + LatexFmt,
    {
        let parsed = self.parsed(source)?;
        Ok(method.format(&parsed))
    }

    pub fn format_checked(
        &mut self,
        source: Source,
        method: FormatMethod,
    ) -> Result<String, LanguageError>
    where
        Lang::Term: GroupParse + Typecheck<Lang = Lang> + LatexFmt,
        Lang::Type: GroupParse + LatexFmt,
    {
        let checked = self.checked(source)?;
        Ok(method.format(&checked))
    }

    pub fn format_evaluated(
        &mut self,
        source: Source,
        method: FormatMethod,
    ) -> Result<String, LanguageError>
    where
        Lang::Term: GroupParse + Eval<Lang = Lang> + LatexFmt,
        Lang::Type: GroupParse,
        Lang::Value: LatexFmt,
    {
        let evaluated = self.evaluated(source)?;
        Ok(method.format(&evaluated))
    }

    pub fn format_grammar(&mut self, method: FormatMethod) -> String
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

    pub fn format_infer(
        &mut self,
        source: Source,
        method: FormatMethod,
    ) -> Result<String, LanguageError>
    where
        Lang::Term: GroupParse + GenerateConstraints<Lang = Lang, Target = Lang::Type>,
        Lang::Type:
            GroupParse + LatexFmt + GenerateConstraints<Lang = Lang> + SolveConstraint<Lang = Lang>,
    {
        if let Some(inferred) = self.inferred.get(&source) {
            return Ok(method.format(inferred));
        }

        let parsed = self.parsed(source.clone())?;
        let inferred = infer_types(&parsed)?;
        self.inferred.insert(source, inferred.clone());
        Ok(method.format(&inferred))
    }
}
