use crate::{LatexConfig, LatexFmt};
use grammar::symbols::SpecialChar;

impl LatexFmt for SpecialChar {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();

        let val = match self {
            SpecialChar::Number => "int",
            SpecialChar::Lambda => "\\lambda",
            SpecialChar::Mu => "\\mu",
            SpecialChar::Gamma => "\\Gamma",
            SpecialChar::Forall => "\\forall",
            SpecialChar::Exists => "\\exists",
            SpecialChar::Top => "\\top",
            SpecialChar::Bot => "\\bot",
            SpecialChar::Dot => ".",
            SpecialChar::Colon => ":",
            SpecialChar::DoubleColon => "::",
            SpecialChar::Exclamation => "!",
            SpecialChar::LessColon => "<:",
            SpecialChar::Comma => ",",
            SpecialChar::Equals => "=",
            SpecialChar::Plus => "+",
            SpecialChar::Times => "\\times",
            SpecialChar::Empty => "",
            SpecialChar::Arrow => "\\rightarrow",
            SpecialChar::DoubleArrow => "\\Rightarrow",
            SpecialChar::Mapsto => "\\mapsto",
            SpecialChar::Space => "\\, ",
            SpecialChar::ColonEq => ":=",
            SpecialChar::BrackO => "\\{{",
            SpecialChar::BrackC => "\\}}",
            SpecialChar::ParenO => "(",
            SpecialChar::ParenC => ")",
            SpecialChar::SqBrackO => "[",
            SpecialChar::SqBrackC => "]",
            SpecialChar::AngBrackO => "<",
            SpecialChar::AngBrackC => ">",
            SpecialChar::Star => "*",
            SpecialChar::LessEq => "\\leq",
            SpecialChar::Ellipses => "\\dots",
            SpecialChar::Pipe => "|",
            SpecialChar::Element => "\\in",
        };
        format!("{env_start}{val}{env_end}")
    }
}
