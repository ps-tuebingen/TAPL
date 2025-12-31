use crate::{config::TestConfig, test_result::TestResult, tests::Test};
use inference::{
    GenerateConstraints, SolveConstraint, generate_constraints_program, solve_constraints,
};
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
    type Result = Lang::Type;
    type Input = Program<Lang>;

    fn name(&self) -> String {
        format!("Inferring {}", self.name)
    }

    fn run(&self) -> TestResult<Self::Result> {
        let constrs = generate_constraints_program(&self.prog);

        let solved = match solve_constraints(constrs) {
            Ok(s) => s,
            Err(err) => return TestResult::from_err(err),
        };

        let applied = solved.apply();
        TestResult::from_eq(applied.main_ty, &self.expected)
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
