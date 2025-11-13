use crate::{LatexConfig, LatexFmt};
use grammar::symbols::SpecialChar;

impl LatexFmt for SpecialChar {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();

        let val = match self {
            Self::Number => "int",
            Self::Lambda => "\\lambda",
            Self::Mu => "\\mu",
            Self::Gamma => "\\Gamma",
            Self::Forall => "\\forall",
            Self::Exists => "\\exists",
            Self::Top => "\\top",
            Self::Bot => "\\bot",
            Self::Dot => ".",
            Self::Colon => ":",
            Self::DoubleColon => "::",
            Self::Exclamation => "!",
            Self::LessColon => "<:",
            Self::Comma => ",",
            Self::Equals => "=",
            Self::Plus => "+",
            Self::Times => "\\times",
            Self::Empty => "",
            Self::Arrow => "\\rightarrow",
            Self::DoubleArrow => "\\Rightarrow",
            Self::Mapsto => "\\mapsto",
            Self::Space => "\\, ",
            Self::ColonEq => ":=",
            Self::BrackO => "\\{{",
            Self::BrackC => "\\}}",
            Self::ParenO => "(",
            Self::ParenC => ")",
            Self::SqBrackO => "[",
            Self::SqBrackC => "]",
            Self::AngBrackO => "<",
            Self::AngBrackC => ">",
            Self::Star => "*",
            Self::LessEq => "\\leq",
            Self::Ellipses => "\\dots",
            Self::Pipe => "|",
            Self::Element => "\\in",
        };
        format!("{env_start}{val}{env_end}")
    }
}
