use super::test_result::TestResult;

pub trait Test {
    fn name(&self) -> String;
    fn run(&self) -> TestResult;
}
