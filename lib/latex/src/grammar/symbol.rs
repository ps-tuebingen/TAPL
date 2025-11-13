use crate::{LatexConfig, LatexFmt};
use grammar::Symbol;

impl LatexFmt for Symbol {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;

        let sym_str = match self {
            Self::Many(sym) => format!("\\overline{{ {} }}", sym.to_latex(conf)),
            Self::Str(s) => s.to_latex(conf),
            Self::Int(i) => i.to_string(),
            Self::Keyword(kw) => kw.to_latex(conf),
            Self::SpecialChar(ch) => ch.to_latex(conf),
            Self::Term => "t".to_owned(),
            Self::Type => "\\tau".to_owned(),
            Self::Kind => "\\kappa".to_owned(),
            Self::Value => "v".to_owned(),
            Self::Variable => "x".to_owned(),
            Self::Typevariable => "X".to_owned(),
            Self::Label => "\\text{{label}}".to_owned(),
            Self::Location => "l".to_owned(),
            Self::Subscript { sym, ind } => {
                format!("{}_{{{}}}", sym.to_latex(conf), ind.to_latex(conf))
            }
            Self::Seq(syms) => syms
                .iter()
                .map(|sym| sym.to_latex(conf))
                .collect::<Vec<_>>()
                .join(" "),
        };
        format!("{env_start}{sym_str}{env_end}")
    }
}
