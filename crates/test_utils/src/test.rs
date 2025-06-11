use super::test_result::TestResult;

pub trait Test<'a> {
    fn name(&self) -> String;
    fn run(&self) -> TestResult;
}

pub trait TestConfig: for<'a> serde::Deserialize<'a> {
    fn set_name(&mut self, name: String);
    fn set_contents(&mut self, contents: String);

    fn name(&self) -> &str;
    fn contents(&self) -> &str;
}
