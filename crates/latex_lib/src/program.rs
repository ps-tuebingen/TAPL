use crate::{LatexConfig, LatexFmt};
use syntax::{program::Program, terms::Term, types::Type};

impl<T, Ty> LatexFmt for Program<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = if conf.include_envs {
            ("\\[", "\\]")
        } else {
            ("", "")
        };

        conf.include_envs = false;

        let main_str = if let Some(ref main) = self.main {
            main.to_latex(conf)
        } else {
            "".to_owned()
        };
        format!(
            "{env_start}{} {}{env_end}",
            self.definitions
                .iter()
                .map(|def| def.to_latex(conf))
                .collect::<Vec<String>>()
                .join("\n\\\\\n"),
            main_str
        )
    }
}
