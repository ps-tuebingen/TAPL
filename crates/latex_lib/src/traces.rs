use super::{LatexConfig, LatexFmt};
use syntax::{terms::Term, values::Value};
use trace::{EvalStep, EvalTrace};

impl<T> LatexFmt for EvalStep<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let source_str = self.source.to_latex(conf);
        let target_str = self.target.to_latex(conf);
        format!("{source_str} \\mapsto_{{{}}} {target_str}", self.rule)
    }
}

impl<T, V> LatexFmt for EvalTrace<T, V>
where
    T: Term + LatexFmt,
    V: Value + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let math_env = if conf.include_envs {
            ("\\[", "\\]")
        } else {
            ("", "")
        };
        let step_strs = self
            .steps
            .iter()
            .map(|step| step.to_latex(conf))
            .collect::<Vec<String>>();
        format!(
            "{}{}\\\\{}{}",
            math_env.0,
            step_strs.join("\\\\"),
            self.val().to_latex(conf),
            math_env.1
        )
    }
}
