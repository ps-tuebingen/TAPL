use super::{Derivation, LatexConfig, LatexFmt};
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for Derivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let mut out = "".to_owned();
        let conc_str = match self.premises.len() {
            0 => format!("\\UnaryInfC{{${}$}}", self.conc.to_latex(conf)),
            1 => format!("\\UnaryInfC{{${}$}}", self.conc.to_latex(conf)),
            2 => format!("\\BinaryInfC{{${}$}}", self.conc.to_latex(conf)),
            3 => format!("\\TrinaryInfC{{${}$}}", self.conc.to_latex(conf)),
            4 => format!("\\QuaternaryInfC{{${}$}}", self.conc.to_latex(conf)),
            5 => format!("\\QuinaryInfC{{${}$}}", self.conc.to_latex(conf)),
            _ => panic!("Derivations with more than 5 premises are not supported"),
        };

        if conf.include_tree_env {
            out += "\\begin{prooftree}\n";
        }

        if self.premises.is_empty() {
            out += "\\AxiomC{\\quad}\n";
        } else {
            let old_inc = conf.include_tree_env;
            conf.include_tree_env = false;
            for prem in self.premises.iter() {
                out += "\t";
                out += &prem.to_latex(conf);
                out += "\n";
            }
            conf.include_tree_env = old_inc;
        }

        out += "\\RightLabel{";
        out += &self.label.to_string();
        out += "}\n";
        out += &conc_str;
        if conf.include_tree_env {
            out += "\n\\end{prooftree}";
        }

        out
    }
}
