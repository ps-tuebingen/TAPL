use crate::{Conclusion, Derivation};

mod conclusion;
mod derivation;
mod env;
mod kind;
mod terms;
mod types;
mod untyped;
mod values;

pub struct LatexConfig {
    include_tree_env: bool,
    use_frac_array: bool,
}

pub trait LatexFmt {
    fn to_latex(&self, conf: &mut LatexConfig) -> String;
    fn to_document(&self, conf: &mut LatexConfig) -> String {
        let mut out = "".to_owned();
        out += "\\documentclass{article}\n";
        out += "\\usepackage[paperheight=226in,paperwidth=226in,margin=0in,landscape]{geometry}";
        out += "\\usepackage{bussproofs}\n";
        out += "\\begin{document}\n";
        out += &self.to_latex(conf);
        out += "\\end{document}\n";
        out
    }
}

impl Default for LatexConfig {
    fn default() -> LatexConfig {
        LatexConfig {
            include_tree_env: true,
            use_frac_array: false,
        }
    }
}
