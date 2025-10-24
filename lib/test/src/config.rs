use errors::{DirAccess, FileAccess, Toml, test_error::TestError};
use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

/// configuration for a single test in `examples/`
/// Parsed from the correspongding .toml file for each example
#[derive(serde::Deserialize)]
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
}

impl TestConfig {
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
}
