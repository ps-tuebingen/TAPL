use crate::{LatexConfig, LatexFmt};
use grammar::Symbol;

impl LatexFmt for Symbol {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;

        let sym_str = match self {
            Symbol::Many(sym) => format!("\\overline{{ {} }}", sym.to_latex(conf)),
            Symbol::Keyword(kw) => kw.to_latex(conf),
            Symbol::SpecialChar(ch) => ch.to_latex(conf),
            Symbol::Term => "t".to_owned(),
            Symbol::Type => "\\tau".to_owned(),
            Symbol::Kind => "\\kappa".to_owned(),
            Symbol::Value => "v".to_owned(),
            Symbol::Variable => "x".to_owned(),
            Symbol::Typevariable => "X".to_owned(),
            Symbol::Label => "\\text{{label}}".to_owned(),
            Symbol::Location => "l".to_owned(),
            Symbol::Prefixed { prefix, inner } => {
                format!("{} {}", prefix.to_latex(conf), inner.to_latex(conf))
            }
            Symbol::Delim {
                delim_open,
                inner,
                delim_close,
            } => format!(
                "{}{}{}",
                delim_open.to_latex(conf),
                inner.to_latex(conf),
                delim_close.to_latex(conf)
            ),
            Symbol::Separated {
                fst,
                separator,
                snd,
            } => format!(
                "{}{}{}",
                fst.to_latex(conf),
                separator.to_latex(conf),
                snd.to_latex(conf)
            ),
            Symbol::Case { bound, patterns } => format!(
                "\\text{{ case }} {} \\text{{ of }} \\{{ {} \\}}",
                bound.to_latex(conf),
                patterns
                    .iter()
                    .map(|pt| pt.to_latex(conf))
                    .collect::<Vec<String>>()
                    .join(" \\mid")
            ),
            Symbol::Pattern { lhs, rhs } => {
                format!("{} \\Rightarrow {}", lhs.to_latex(conf), rhs.to_latex(conf))
            }
        };
        format!("{env_start}{sym_str}{env_end}")
    }
}
