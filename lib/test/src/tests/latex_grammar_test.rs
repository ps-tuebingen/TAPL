use crate::{config::TestConfig, paths::LATEX_OUT, test_result::TestResult, tests::Test};
use grammar::LanguageDescribe;
use latex::LatexFmt;
use std::marker::PhantomData;
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
use syntax::language::Language;

/// Tests printing and compiling a language ([`syntax::language::Language`]) grammar
pub struct LatexTestGrammar<L>
where
    L: Language,
{
    /// The name of the language
    name: String,
    /// phantom data to save the language type
    phantom: PhantomData<L>,
}

impl<L> LatexTestGrammar<L>
where
    L: Language,
{
    /// Crate a new grammar test from a given name
    pub fn new(name: &str) -> LatexTestGrammar<L> {
        LatexTestGrammar {
            name: name.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<L> Test for LatexTestGrammar<L>
where
    L: Language + LanguageDescribe,
{
    type Result = ();
    type Input = ();

    fn name(&self) -> String {
        format!("Generating Latex for Grammar of {}", self.name)
    }

    fn run(&self) -> TestResult<()> {
        let grammar_src = L::grammars().to_document(&mut Default::default());
        let rule_src = L::rules().to_document(&mut Default::default());

        let mut out_path_grammar = PathBuf::from(LATEX_OUT).join(format!("{}_grammar", self.name));
        let mut out_path_rules = PathBuf::from(LATEX_OUT).join(format!("{}_rules", self.name));
        out_path_grammar.set_extension("tex");
        out_path_rules.set_extension("tex");

        let mut out_file_grammar = match File::create(&out_path_grammar) {
            Ok(f) => f,
            Err(err) => return TestResult::from_err(err),
        };
        let mut out_file_rules = match File::create(&out_path_rules) {
            Ok(f) => f,
            Err(err) => return TestResult::from_err(err),
        };

        if let Err(err) = out_file_grammar.write_all(grammar_src.as_bytes()) {
            return TestResult::from_err(err);
        };
        if let Err(err) = out_file_rules.write_all(rule_src.as_bytes()) {
            return TestResult::from_err(err);
        };

        fn latex_cmd(out_path: PathBuf) -> Command {
            let mut latex_cmd = Command::new("xelatex");
            latex_cmd.arg("-halt-on-error");
            latex_cmd.arg(format!("-output-directory={LATEX_OUT}"));
            latex_cmd.arg("-inteteraction=nonstopmode");
            latex_cmd.arg(out_path);
            latex_cmd.stdout(Stdio::null());
            latex_cmd.stderr(Stdio::null());
            latex_cmd
        };

        let mut latex_cmd_grammar = latex_cmd(out_path_grammar);
        let mut latex_cmd_rules = latex_cmd(out_path_rules);

        match latex_cmd_grammar.status() {
            Err(err) => return TestResult::from_err(err),
            Ok(exit) if !exit.success() => {
                return TestResult::Fail("xelatex exited with non-zero exit status".to_owned());
            }
            _ => (),
        }
        match latex_cmd_rules.status() {
            Err(err) => return TestResult::from_err(err),
            Ok(exit) if !exit.success() => {
                return TestResult::Fail("xelatex exited with non-zero exit status".to_owned());
            }
            _ => (),
        }
        TestResult::Success(())
    }

    fn from_conf(conf: &TestConfig, _: Self::Input) -> Option<Self> {
        if !conf.include_grammar() {
            None
        } else {
            Some(LatexTestGrammar {
                name: conf.name.clone(),
                phantom: PhantomData,
            })
        }
    }
}
