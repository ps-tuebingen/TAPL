use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

pub struct Test {
    pub source_file: PathBuf,
    pub source_str: String,
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

pub fn load_dir(dir: &PathBuf) -> Result<Vec<Test>, Box<dyn std::error::Error>> {
    let mut tests = vec![];
    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let contents = read_to_string(&path)?;
        tests.push(Test {
            source_file: path,
            source_str: contents,
        });
    }

    Ok(tests)
}
