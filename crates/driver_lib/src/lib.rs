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

mod cli;
mod format;
mod formattable;
mod languages;

use cli::{Args, Command};
use format::FormatMethod;

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
            Command::Parse => {
                let parsed = self.parse::<L>(input)?;
                Ok(method.format(&parsed))
            }
            Command::Check => {
                let checked = self.check::<L>(input)?;
                Ok(method.format(&checked))
            }
            Command::Evaluate => {
                let evaled = self.eval::<L>(input)?;
                Ok(method.format(&evaled))
            }
            Command::Grammar => {
                let grammars = <L as LanguageDescribe>::grammars();
                Ok(method.format(&grammars))
            }
        }
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

    pub fn write_to_file(&self, res: String, path: PathBuf) -> Result<(), DriverError> {
        let mut file =
            File::create(path).map_err(|err| FileAccess::new("open file for writing", err))?;
        file.write_all(res.as_bytes())
            .map_err(|err| FileAccess::new("write to file", err))?;
        Ok(())
    }
}
