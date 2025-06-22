use super::test_result::TestResult;

pub struct TestInclusions {
    pub reparse: bool,
    pub check: bool,
    pub eval: bool,
    pub derivation_buss: bool,
    pub derivation_frac: bool,
    pub trace: bool,
}

impl TestInclusions {
    pub fn num_tests(&self) -> usize {
        let mut num = 1;
        if self.reparse {
            num += 1;
        }
        if self.check {
            num += 1;
        }
        if self.eval {
            num += 1;
        }
        if self.derivation_buss {
            num += 1;
        }
        if self.derivation_frac {
            num += 1;
        }
        if self.trace {
            num += 1;
        }
        num
    }
}

pub trait Test<T> {
    fn name(&self) -> String;
    fn run(&self) -> TestResult<T>;
}

pub trait TestConfig: for<'a> serde::Deserialize<'a> {
    fn set_contents(&mut self, contents: String);

    fn ty(&self) -> &str;
    fn evaluated(&self) -> &str;

    fn name(&self) -> &str;
    fn contents(&self) -> &str;
}

impl Default for TestInclusions {
    fn default() -> TestInclusions {
        TestInclusions {
            reparse: true,
            check: true,
            eval: true,
            derivation_buss: true,
            derivation_frac: true,
            trace: true,
        }
    }
}
