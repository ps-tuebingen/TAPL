use crate::{config::TestConfig, test_result::TestResult, tests::Test};
use inference::{GenerateConstraints, ProgTypes, SolveConstraint, infer_types};
use syntax::{language::Language, program::Program};

/// Tests type inferring a program
/// The program is in the given languge ([`syntax::language::Language`])
pub struct InferenceTest<Lang>
where
    Lang: Language,
{
    /// The name of the program
    name: String,
    /// The program to infer
    prog: Program<Lang>,
    /// The expected type(s)
    expected: String,
}

impl<Lang> InferenceTest<Lang>
where
    Lang: Language,
{
    /// Create a new inference test from name, program and expected str
    pub fn new(name: &str, prog: Program<Lang>, exp: &str) -> InferenceTest<Lang> {
        InferenceTest {
            name: name.to_owned(),
            prog,
            expected: exp.to_owned(),
        }
    }
}

impl<Lang> Test for InferenceTest<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Lang::Type: GenerateConstraints<Lang = Lang> + SolveConstraint<Lang = Lang>,
{
    type Result = ProgTypes<Lang>;
    type Input = Program<Lang>;

    fn name(&self) -> String {
        format!("Inferring {}", self.name)
    }

    fn run(&self) -> TestResult<ProgTypes<Lang>> {
        let inferred = match infer_types(&self.prog) {
            Ok(c) => c,
            Err(err) => return TestResult::from_err(err),
        };

        let checked_str = inferred.main_ty.to_string();
        if checked_str == self.expected {
            TestResult::Success(inferred)
        } else {
            TestResult::Fail(format!(
                "Result!=Expected:\n\tresult:   {checked_str}\n\texpected: {}",
                self.expected
            ))
        }
    }

    fn from_conf(conf: &TestConfig, prog: Self::Input) -> Option<Self> {
        if !conf.include_inference() {
            None
        } else {
            Some(InferenceTest {
                name: conf.name.clone(),
                expected: conf.ty.clone(),
                prog,
            })
        }
    }
}
