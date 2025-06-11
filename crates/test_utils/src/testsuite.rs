use super::{errors::Error, test::Test, test_result::TestResult};

pub trait TestSuite {
    fn name(&self) -> String;
    fn load(&self) -> Result<Vec<Box<dyn Test>>, Error>;

    fn run_all(&self) -> Result<usize, Error> {
        println!("Running Test Suite {}\n", self.name());
        let tests = self.load()?;
        let num_tests = tests.len();
        let mut num_fail = 0;
        for test in tests {
            let result = test.run();
            result.report(&test.name());
            if matches!(result, TestResult::Fail(_)) {
                num_fail += 1
            }
        }
        println!();
        println!(
            "\tRan {} tests: {} success, {} fail\n",
            num_tests,
            num_tests - num_fail,
            num_fail
        );
        Ok(num_fail)
    }
}
