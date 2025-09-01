use super::{LatexConfig, LatexFmt};
use syntax::language::Language;
use trace::{EvalStep, EvalTrace};

impl<Lang> LatexFmt for EvalStep<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let source_str = self.source.to_latex(conf);
        let target_str = self.target.to_latex(conf);
        format!(
            "{source_str} \\\\ \\mapsto_{{{}}} \\\\ {target_str}",
            self.rule
        )
    }
}

impl<Lang> LatexFmt for EvalTrace<Lang>
where
    Lang: Language,
    Lang::Term: LatexFmt,
    Lang::Value: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let math_env = if conf.include_envs {
            ("\\[", "\\]")
        } else {
            ("", "")
        };
        conf.include_envs = false;
        let step_strs = self
            .steps
            .iter()
            .map(|step| {
                format!(
                    "{} \\\\ \\mapsto_{{{}}} \\\\ ",
                    step.source.to_latex(conf),
                    step.rule
                )
            })
            .collect::<Vec<String>>();
        format!(
            "{}\\begin{{array}}{{c c}}{} \\\\ {}\\end{{array}}{}",
            math_env.0,
            step_strs.join("\\\\"),
            self.val().to_latex(conf),
            math_env.1
        )
    }
}
