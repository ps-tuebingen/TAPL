use language::languages::subtypes::Subtypes;
use std::path::PathBuf;
use test_utils::{
    errors::Error,
    paths::{EXAMPLES_PATH, SUBTYPES_PATH},
    setup,
    test::TestConfig,
    testsuite::TestSuite,
};

pub struct SubtypesTests {
    source_path: PathBuf,
}

#[derive(serde::Deserialize)]
pub struct SubtypesConf {
    ty: String,
    evaluated: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    contents: String,
}

impl TestConfig for SubtypesConf {
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

impl SubtypesTests {
    pub fn new(path: PathBuf) -> SubtypesTests {
        SubtypesTests { source_path: path }
    }
}

impl TestSuite for SubtypesTests {
    type Config = SubtypesConf;
    type Lang = Subtypes;

    fn name(&self) -> &str {
        "Subtypes"
    }

    fn ext(&self) -> &str {
        "sub"
    }

    fn source_dir(&self) -> PathBuf {
        self.source_path.clone()
    }
}

fn main() -> Result<(), Error> {
    setup()?;

    let examples_dir = PathBuf::from(EXAMPLES_PATH);
    let fails = SubtypesTests::new(examples_dir.join(SUBTYPES_PATH)).run_all()?;
    let fail_str = if fails > 0 {
        format!("\x1b[31m{fails} fails\x1b[39m")
    } else {
        "0 fails".to_owned()
    };
    println!("Finished running tests with {fail_str}");
    if fails > 0 {
        panic!("Not all tests finished successfully");
    }
    Ok(())
}
