use language::languages::exceptions::Exceptions;
use std::path::PathBuf;
use test_utils::{
    errors::Error,
    paths::{EXAMPLES_PATH, EXCEPTIONS_PATH},
    setup,
    test::TestConfig,
    testsuite::TestSuite,
};

pub struct ExceptionTests {
    source_path: PathBuf,
}

impl TestConfig for ExceptionsConf {
    fn set_name(&mut self, name: String) {
        self.name = name
    }
    fn set_contents(&mut self, contents: String) {
        self.contents = contents
    }

    fn name(&self) -> &str {
        &self.name
    }
    fn contents(&self) -> &str {
        &self.contents
    }
    fn ty(&self) -> &str {
        &self.ty
    }
    fn evaluated(&self) -> &str {
        &self.evaluated
    }
}

#[derive(serde::Deserialize)]
pub struct ExceptionsConf {
    ty: String,
    evaluated: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl ExceptionTests {
    pub fn new(path: PathBuf) -> ExceptionTests {
        ExceptionTests { source_path: path }
    }
}

impl TestSuite for ExceptionTests {
    type Lang = Exceptions;
    type Config = ExceptionsConf;

    fn name(&self) -> &str {
        "Exceptions"
    }

    fn ext(&self) -> &str {
        "ex"
    }

    fn source_dir(&self) -> PathBuf {
        self.source_path.clone()
    }
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = ExceptionTests::new(examples_dir.join(EXCEPTIONS_PATH)).run_all()?;

    let fail_str = if fails > 0 {
        format!("\x1b[31m{fails} fails\x1b[39m")
    } else {
        "0 fails".to_owned()
    };

    println!("Finished running tests with {fail_str}",);
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
