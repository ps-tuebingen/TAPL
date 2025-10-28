use crate::{LatexConfig, LatexFmt};
use grammar::Symbol;

impl LatexFmt for Symbol {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;

        let sym_str = match self {
            Symbol::Many(sym) => format!("\\overline{{ {} }}", sym.to_latex(conf)),
            Symbol::Str(s) => s.to_latex(conf),
            Symbol::Int(i) => i.to_string(),
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
            Symbol::Subscript { sym, ind } => {
                format!("{}_{}", sym.to_latex(conf), ind.to_latex(conf))
            }
            Symbol::Seq(syms) => syms
                .iter()
                .map(|sym| sym.to_latex(conf))
                .collect::<Vec<_>>()
                .join(" "),
        };
        format!("{env_start}{sym_str}{env_end}")
    }
}
