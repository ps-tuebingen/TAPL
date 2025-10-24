use check::Typecheck;
use derivations::Derivation;
use errors::{FileAccess, NoTyping, driver_error::DriverError};
use eval::{Eval, eval_main};
use grammar::LanguageDescribe;
use languages::{
    AllLanguages, BoundedQuantification, Exceptions, Existential, FOmega, FOmegaSub, LambdaOmega,
    Recursive, References, Stlc, Subtypes, SystemF, TypedArithmetic,
};
use latex::LatexFmt;
use parser::{GroupParse, Parse};
use syntax::{language::Language, program::Program};
use trace::EvalTrace;

use std::{fs::File, io::Write, path::PathBuf};

pub mod cli;
pub mod format;
mod formattable;

use cli::{Args, Command};
use format::FormatMethod;

#[derive(Clone)]
pub struct Driver;

impl Driver {
    pub fn run_cli(&self) -> Result<(), DriverError> {
        let args = <Args as clap::Parser>::parse();
        let input = if matches!(args.cmd, Command::Grammar) {
            "".to_owned()
        } else {
            args.source.get_source()?
        };
        let res = dispatch_run(&args.lang, self, &args.method(), &args.cmd, input)?;
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
        L::Term: GroupParse + LatexFmt + Typecheck<Lang = L> + Eval<Lang = L>,
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

    pub fn run_lang(
        &self,
        input: String,
        lang: &AllLanguages,
        cmd: &Command,
        method: &FormatMethod,
    ) -> Result<String, String> {
        dispatch_run(lang, self, method, cmd, input).map_err(|err| err.to_string())
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
        let parse_res = match dispatch_run(lang, self, method, &Command::Parse, input.clone()) {
            Ok(p) => p,
            Err(err) => return (None, None, None, Some(err.to_string())),
        };
        let check_res = match dispatch_run(lang, self, method, &Command::Check, input.clone()) {
            Ok(ty) => ty,
            Err(err) => return (None, None, None, Some(err.to_string())),
        };

        let eval_res = match dispatch_run(lang, self, method, &Command::Check, input.clone()) {
            Ok(v) => v,
            Err(err) => return (None, None, None, Some(err.to_string())),
        };
        (Some(parse_res), Some(check_res), Some(eval_res), None)
    }

    pub fn parse<L>(&self, input: String) -> Result<Program<L>, DriverError>
    where
        L: Language,
        L::Term: GroupParse,
        L::Type: GroupParse,
    {
        let parsed = <Program<L>>::parse(input)?;
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

    pub fn check<L>(&self, input: String) -> Result<Derivation<L>, DriverError>
    where
        L: Language,
        L::Term: GroupParse + Typecheck<Lang = L>,
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
        L::Term: GroupParse + Typecheck<Lang = L> + LatexFmt,
        L::Type: GroupParse + LatexFmt,
    {
        let checked = self.check::<L>(input)?;
        Ok(method.format(&checked))
    }

    pub fn eval<L>(&self, input: String) -> Result<EvalTrace<L>, DriverError>
    where
        L: Language,
        L::Term: GroupParse + Eval<Lang = L>,
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
        L::Term: GroupParse + Eval<Lang = L> + LatexFmt,
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

pub fn dispatch_run(
    lang: &AllLanguages,
    driver: &Driver,
    method: &FormatMethod,
    cmd: &Command,
    input: String,
) -> Result<String, DriverError> {
    match lang {
        AllLanguages::TypedArithmetic => driver.run_format::<TypedArithmetic>(method, cmd, input),
        AllLanguages::Stlc => driver.run_format::<Stlc>(method, cmd, input),
        AllLanguages::Exceptions => driver.run_format::<Exceptions>(method, cmd, input),
        AllLanguages::References => driver.run_format::<References>(method, cmd, input),
        AllLanguages::Existential => driver.run_format::<Existential>(method, cmd, input),
        AllLanguages::Recursive => driver.run_format::<Recursive>(method, cmd, input),
        AllLanguages::Subtypes => driver.run_format::<Subtypes>(method, cmd, input),
        AllLanguages::SystemF => driver.run_format::<SystemF>(method, cmd, input),
        AllLanguages::BoundedQuantification => {
            driver.run_format::<BoundedQuantification>(method, cmd, input)
        }
        AllLanguages::LambdaOmega => driver.run_format::<LambdaOmega>(method, cmd, input),
        AllLanguages::FOmega => driver.run_format::<FOmega>(method, cmd, input),
        AllLanguages::FOmegaSub => driver.run_format::<FOmegaSub>(method, cmd, input),
        _ => Err(NoTyping::new(&lang.to_string()).into()),
    }
}
