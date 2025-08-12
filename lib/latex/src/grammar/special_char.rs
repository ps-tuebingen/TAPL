use crate::{LatexConfig, LatexFmt};
use grammar::symbols::SpecialChar;

impl LatexFmt for SpecialChar {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();

        match self {
            SpecialChar::Number => format!("{env_start}int{env_end}"),
            SpecialChar::Lambda => format!("{env_start}\\lambda{env_end}"),
            SpecialChar::Mu => format!("{env_start}\\mu{env_end}"),
            SpecialChar::Forall => format!("{env_start}\\forall{env_end}"),
            SpecialChar::Exists => format!("{env_start}\\exists{env_end}"),
            SpecialChar::Top => format!("{env_start}\\top{env_end}"),
            SpecialChar::Bot => format!("{env_start}\\top{env_end}"),
            SpecialChar::Dot => format!("{env_start}.{env_end}"),
            SpecialChar::Colon => format!("{env_start}:{env_end}"),
            SpecialChar::DoubleColon => format!("{env_start}::{env_end}"),
            SpecialChar::Exclamation => format!("{env_start}!{env_end}"),
            SpecialChar::LessColon => format!("{env_start}<:{env_end}"),
            SpecialChar::Comma => format!("{env_start},{env_end}"),
            SpecialChar::Equals => format!("{env_start}={env_end}"),
            SpecialChar::Plus => format!("{env_start}+{env_end}"),
            SpecialChar::Times => format!("{env_start}\\times{env_end}"),
            SpecialChar::Empty => "".to_owned(),
            SpecialChar::Arrow => format!("{env_start}\\rightarrow{env_end}"),
            SpecialChar::DoubleArrow => format!("{env_start}\\Rightarrow{env_end}"),
            SpecialChar::Space => format!("{env_start}\\, {env_end}"),
            SpecialChar::ColonEq => format!("{env_start}:={env_end}"),
            SpecialChar::BrackO => format!("{env_start}\\{{{env_end}"),
            SpecialChar::BrackC => format!("{env_start}\\}}{env_end}"),
            SpecialChar::ParenO => format!("{env_start}({env_end}"),
            SpecialChar::ParenC => format!("{env_start}){env_end}"),
            SpecialChar::SqBrackO => format!("{env_start}[{env_end}"),
            SpecialChar::SqBrackC => format!("{env_start}]{env_end}"),
            SpecialChar::AngBrackO => format!("{env_start}<{env_end}"),
            SpecialChar::AngBrackC => format!("{env_start}>{env_end}"),
            SpecialChar::Star => format!("{env_start}*{env_end}"),
        }
    }
}
