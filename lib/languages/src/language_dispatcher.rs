use crate::stlc::Stlc;
use parser::{GroupParse, Parse};
use std::{any::Any, collections::HashMap, path::PathBuf};
use syntax::{Name, language::Language, program::Program};

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

pub struct Dispatcher<Lang>
where
    Lang: Language,
{
    sources: HashMap<PathBuf, String>,
    parsed: HashMap<PathBuf, Program<Lang>>,
    checked: HashMap<SourceLocation, Lang::Type>,
    evaluated: HashMap<SourceLocation, Lang::Value>,
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
}

pub trait DispatchLanguage {
    fn run_command(&mut self, cmd: Command, source: PathBuf) -> Box<dyn Any>;
}

impl<Lang> DispatchLanguage for Dispatcher<Lang>
where
    Lang: Language + 'static,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn run_command(&mut self, cmd: Command, source: PathBuf) -> Box<dyn Any> {
        match cmd {
            Command::Parse => match self.parsed.get(&source) {
                None => {
                    let parsed = Program::<Lang>::parse("".to_string()).unwrap();
                    self.parsed.insert(source, parsed.clone());
                    Box::new(parsed) as Box<dyn Any>
                }
                Some(res) => Box::new(res.clone()) as Box<dyn Any>,
            },
            _ => todo!(),
        }
    }
}

fn create_dispatcher(lang_str: &str) -> Box<dyn DispatchLanguage> {
    match lang_str {
        "Stlc" => Box::new(Dispatcher::<Stlc>::new()) as Box<dyn DispatchLanguage>,
        _ => todo!(),
    }
}
