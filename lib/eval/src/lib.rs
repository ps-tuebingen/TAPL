pub mod terms;

use errors::eval_error::EvalError;
use syntax::{eval_context::EvalContext, language::Language, program::Program};
use trace::EvalTrace;

pub trait Eval: Sized {
    type Lang: Language;

    fn eval_start(self) -> Result<EvalTrace<Self::Lang>, EvalError> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError>;
}

pub fn eval_main<Lang>(prog: Program<Lang>) -> Result<EvalTrace<Lang>, EvalError>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
{
    let mut ctx = EvalContext::<Lang>::from_prog(&prog);
    prog.main.eval(&mut ctx)
}
