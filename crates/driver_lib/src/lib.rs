use check::Typecheck;
use derivation::{ProgramDerivation, TypingDerivation};
use errors::{FileAccess, driver_error::DriverError};
use eval::{Eval, eval_main};
use grammar::LanguageDescribe;
use latex::LatexFmt;
use parse::{GroupParse, Parse};
use syntax::{language::Language, program::Program};
use trace::EvalTrace;

use std::{fs::File, io::Write, path::PathBuf};

pub mod cli;
pub mod format;
mod formattable;
pub mod languages;

use cli::{Args, Command};
use format::FormatMethod;
use languages::AllLanguages;

#[derive(Clone)]
pub struct Driver;

impl Driver {
    pub fn run_cli(&self) -> Result<(), DriverError> {
        let args = <Args as clap::Parser>::parse();
        let input = if matches!(args.cmd, Command::Grammar) {
            "".to_owned()
        } else {
            let src = args.source.get_source()?;
            src
        };
        let res = args
            .lang
            .dispatch_run(&self, &args.out_method, &args.cmd, input)?;
        match args.out_file {
            None => {
                println!("{res}");
                Ok(())
            }
            Some(out) => self.write_to_file(res, out),
        }
    }

    pub fn run_format<L>(
        &self,
        method: &FormatMethod,
        cmd: &Command,
        input: String,
    ) -> Result<String, DriverError>
    where
        L: Language + LanguageDescribe,
        L::Term: GroupParse
            + LatexFmt
            + Typecheck<Term = L::Term, Type = L::Type, Deriv = TypingDerivation<L::Term, L::Type>>
            + Eval<Term = L::Term, Value = L::Value>,
        L::Type: GroupParse + LatexFmt,
        L::Value: LatexFmt,
    {
        match cmd {
            Command::Parse => self.parse_format::<L>(input, method),
            Command::Check => self.check_format::<L>(input, method),
            Command::Evaluate => self.eval_format::<L>(input, method),
            Command::Grammar => Ok(self.grammar_format::<L>(method)),
        }
    }

    pub fn run_all_lang(
        &self,
        input: String,
        lang: &AllLanguages,
        method: &FormatMethod,
    ) -> (
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    ) {
        let parse_res = match lang.dispatch_run(&self, method, &Command::Parse, input.clone()) {
            Ok(p) => p,
            Err(err) => return (None, None, None, Some(err.to_string())),
        };
        let check_res = match lang.dispatch_run(&self, method, &Command::Check, input.clone()) {
            Ok(ty) => ty,
            Err(err) => return (None, None, None, Some(err.to_string())),
        };

        let eval_res = match lang.dispatch_run(&self, method, &Command::Check, input.clone()) {
            Ok(v) => v,
            Err(err) => return (None, None, None, Some(err.to_string())),
        };
        (Some(parse_res), Some(check_res), Some(eval_res), None)
    }

    pub fn parse<L>(&self, input: String) -> Result<Program<L::Term, L::Type>, DriverError>
    where
        L: Language,
        L::Term: GroupParse,
        L::Type: GroupParse,
    {
        let parsed = <Program<L::Term, L::Type>>::parse(input)?;
        Ok(parsed)
    }

    pub fn parse_format<L>(
        &self,
        input: String,
        method: &FormatMethod,
    ) -> Result<String, DriverError>
    where
        L: Language,
        L::Term: GroupParse + LatexFmt,
        L::Type: GroupParse + LatexFmt,
    {
        let parsed = self.parse::<L>(input)?;
        Ok(method.format(&parsed))
    }

    pub fn check<L>(
        &self,
        input: String,
    ) -> Result<ProgramDerivation<L::Term, L::Type>, DriverError>
    where
        L: Language,
        L::Term: GroupParse
            + Typecheck<Term = L::Term, Type = L::Type, Deriv = TypingDerivation<L::Term, L::Type>>,
        L::Type: GroupParse,
    {
        let parsed = self.parse::<L>(input)?;
        let checked = parsed.check_start()?;
        Ok(checked)
    }

    pub fn check_format<L>(
        &self,
        input: String,
        method: &FormatMethod,
    ) -> Result<String, DriverError>
    where
        L: Language,
        L::Term: GroupParse
            + Typecheck<Term = L::Term, Type = L::Type, Deriv = TypingDerivation<L::Term, L::Type>>
            + LatexFmt,
        L::Type: GroupParse + LatexFmt,
    {
        let checked = self.check::<L>(input)?;
        Ok(method.format(&checked))
    }

    pub fn eval<L>(&self, input: String) -> Result<EvalTrace<L::Term, L::Value>, DriverError>
    where
        L: Language,
        L::Term: GroupParse + Eval<Term = L::Term, Value = L::Value>,
        L::Type: GroupParse,
    {
        let parsed = self.parse::<L>(input)?;
        let evaled = eval_main(parsed)?;
        Ok(evaled)
    }

    pub fn eval_format<L>(
        &self,
        input: String,
        method: &FormatMethod,
    ) -> Result<String, DriverError>
    where
        L: Language,
        L::Term: GroupParse + Eval<Term = L::Term, Value = L::Value> + LatexFmt,
        L::Type: GroupParse,
        L::Value: LatexFmt,
    {
        let evaled = self.eval::<L>(input)?;
        Ok(method.format(&evaled))
    }

    pub fn grammar_format<L>(&self, method: &FormatMethod) -> String
    where
        L: LanguageDescribe,
    {
        method.format(&L::grammars())
    }

    pub fn write_to_file(&self, res: String, path: PathBuf) -> Result<(), DriverError> {
        let mut file =
            File::create(path).map_err(|err| FileAccess::new("open file for writing", err))?;
        file.write_all(res.as_bytes())
            .map_err(|err| FileAccess::new("write to file", err))?;
        Ok(())
    }
}
