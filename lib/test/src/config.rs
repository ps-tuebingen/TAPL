use errors::{DirAccess, FileAccess, Toml, test_error::TestError};
use std::{
    collections::HashMap,
    fs::{read_dir, read_to_string},
    path::PathBuf,
};
use syntax::language::LanguageFeatures;

/// key for reparsing tests used in the `exclusions` map
pub const KEY_REPARSE: &str = "reparse";
/// key for check tests used in the `exclusions` map
pub const KEY_CHECK: &str = "check";
/// key for derivation (bussproofs) tests used in the `exclusions` map
pub const KEY_BUSS: &str = "derivation_buss";
/// key for derivation (frac+array) tests used in the `exclusions` map
pub const KEY_FRAC: &str = "derivation_frac";
/// key for eval tests used in the `exclusions` map
pub const KEY_EVAL: &str = "eval";
/// key for trace tests used in the `exclusions` map
pub const KEY_TRACE: &str = "trace";
/// key for grammar tests used in the `exclusions` map
pub const KEY_GRAMMAR: &str = "grammar";
/// key for parsing tests used in the `exclusions` map
pub const KEY_PARSE: &str = "parse";

/// configuration for a single test in `examples/`
/// Parsed from the correspongding .toml file for each example
#[derive(Debug, serde::Deserialize)]
pub struct TestConfig {
    /// The name of the test (used for pretty printing during testing)
    pub name: String,
    /// The expected type after checking
    /// default is used when the language is untyped
    /// if it is not given for a typed language an error will be thrown
    #[serde(default)]
    pub ty: String,
    /// The expected value after evaluating
    pub evaluated: String,
    /// The contents of the test
    /// This is not saved in the toml config
    /// Instead this is the file contents of the actual example
    #[serde(default)]
    pub contents: String,
    #[serde(default)]
    pub exclusions: HashMap<String, bool>,
}

impl TestConfig {
    /// Empty configuration with only a name
    pub fn empty(name: &str) -> TestConfig {
        TestConfig {
            name: name.to_owned(),
            ty: "".to_owned(),
            evaluated: "".to_owned(),
            contents: "".to_owned(),
            exclusions: HashMap::new(),
        }
    }

    /// Updates `self` based on a given language's features
    pub fn update_features(&mut self, features: &LanguageFeatures) {
        if !features.typed {
            self.exclusions.insert(KEY_CHECK.to_owned(), true);
        }
        if !features.evaluating {
            self.exclusions.insert(KEY_EVAL.to_owned(), true);
        }
    }

    /// Loads a Test config from a base directory
    /// Assumes the example is located in `dir/dir.src_ext`
    /// and assumes the config is located in `dir/dir.toml`
    pub fn from_dir(dir: &PathBuf, src_ext: &str) -> Result<TestConfig, TestError> {
        let stem = dir
            .file_stem()
            .ok_or(FileAccess::new(
                &format!("get file stem of {dir:?}"),
                "Got None",
            ))?
            .to_str()
            .ok_or(FileAccess::new(
                &format!("get file stem of {dir:?} as string"),
                "Got None",
            ))?
            .to_owned();

        let mut source_file = dir.join(stem.clone());
        source_file.set_extension(src_ext);

        let contents = read_to_string(&source_file)
            .map_err(|err| FileAccess::new(&format!("read file {source_file:?}"), err))?;

        let mut config_file = source_file.clone();
        config_file.set_extension("toml");
        let config_contents = read_to_string(&config_file)
            .map_err(|err| FileAccess::new(&format!("Read File {config_file:?}"), err))?;
        let mut config: TestConfig = basic_toml::from_str(&config_contents)
            .map_err(|err| Toml::new(&config_contents, err))?;
        config.contents = contents;
        Ok(config)
    }

    /// Load all examples from a base directory (usually examples/lang with a given language
    /// in each directory in `base_dir`, the config is loaded using [`Self::from_dir`]
    pub fn load_suite(base_dir: &PathBuf, src_ext: &str) -> Result<Vec<TestConfig>, TestError> {
        let mut tests = vec![];
        for entry in
            read_dir(base_dir).map_err(|err| DirAccess::new(&format!("read {base_dir:?}"), err))?
        {
            let entry_dir = entry
                .map_err(|err| DirAccess::new("Read example directory", err))?
                .path();
            let conf = Self::from_dir(&entry_dir, src_ext)?;
            tests.push(conf);
        }
        tests.sort_by(|tst1, tst2| tst1.name.cmp(&tst2.name));
        Ok(tests)
    }

    /// Does this test include reparsing
    pub fn include_reparse(&self) -> bool {
        matches!(self.exclusions.get(KEY_REPARSE), None | Some(false))
    }

    /// Does this test include type checking
    pub fn include_check(&self) -> bool {
        matches!(self.exclusions.get(KEY_CHECK), None | Some(false))
    }

    /// Does this test include derivations (bussproofs)
    pub fn include_buss(&self) -> bool {
        matches!(self.exclusions.get(KEY_BUSS), None | Some(false))
    }

    /// Does this test include derivations (frac+array)
    pub fn include_frac(&self) -> bool {
        matches!(self.exclusions.get(KEY_FRAC), None | Some(false))
    }

    /// Does this test include evaluation
    pub fn include_eval(&self) -> bool {
        matches!(self.exclusions.get(KEY_EVAL), None | Some(false))
    }

    /// Does this test include evaluation
    pub fn include_trace(&self) -> bool {
        matches!(self.exclusions.get(KEY_TRACE), None | Some(false))
    }

    /// Does this test include grammar
    pub fn include_grammar(&self) -> bool {
        matches!(self.exclusions.get(KEY_GRAMMAR), None | Some(false))
    }

    /// Does this test include parsing
    pub fn include_parse(&self) -> bool {
        matches!(self.exclusions.get(KEY_PARSE), None | Some(false))
    }

    /// The number of tests that are set for this test
    pub fn num_tests(&self) -> usize {
        let inclusions = [
            self.include_parse(),
            self.include_reparse(),
            self.include_check(),
            self.include_buss(),
            self.include_frac(),
            self.include_eval(),
            self.include_trace(),
        ];
        inclusions.iter().filter(|inc| **inc).count()
    }
}
