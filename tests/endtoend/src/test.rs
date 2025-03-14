use super::errors::Error;
use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

pub struct Test {
    pub source_file: PathBuf,
    pub source_str: String,
    pub config: TestConfig,
}

#[derive(serde::Deserialize)]
pub struct TestConfig {
    pub expected: String,
}

pub enum TestResult {
    Success,
    Fail(String),
}

impl TestResult {
    pub fn report(self, test_name: &str) {
        match self {
            TestResult::Success => println!("Test {test_name}.....\x1b[32mok\x1b[39m"),
            TestResult::Fail(msg) => println!("Test {test_name}.....\x1b[31mfail\n\t{msg}\x1b[39m"),
        }
    }
}

pub fn load_dir(dir: &PathBuf, src_ext: &str) -> Result<Vec<Test>, Error> {
    let mut tests = vec![];
    for entry in read_dir(dir).map_err(|_| Error::ReadDir(dir.clone()))? {
        let entry = entry.map_err(|_| Error::ReadDir(dir.clone()))?;
        let path = entry.path();
        let stem = path
            .file_stem()
            .ok_or(Error::FileName(path.clone()))?
            .to_str()
            .ok_or(Error::FileName(path.clone()))?
            .to_owned();

        let mut source_file = path.join(stem);
        source_file.set_extension(src_ext);

        let mut config_file = source_file.clone();
        config_file.set_extension("toml");
        let config_contents =
            read_to_string(&config_file).map_err(|_| Error::ReadFile(config_file.clone()))?;
        let config = basic_toml::from_str(&config_contents)
            .map_err(|_| Error::ParseToml(config_file.clone()))?;

        let contents = read_to_string(&source_file).map_err(|_| Error::ReadFile(source_file))?;

        tests.push(Test {
            source_file: path,
            source_str: contents,
            config,
        });
    }

    Ok(tests)
}
