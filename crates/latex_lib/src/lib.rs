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
    pub include_envs: bool,
    pub use_frac_array: bool,
}

impl LatexConfig {
    pub fn new() -> LatexConfig {
        LatexConfig::default()
    }

    pub fn new_frac() -> LatexConfig {
        LatexConfig {
            include_envs: true,
            use_frac_array: true,
        }
    }
}

pub trait LatexFmt {
    fn to_latex(&self, conf: &mut LatexConfig) -> String;
    fn to_document(&self, conf: &mut LatexConfig) -> String {
        let mut out = "".to_owned();
        out += "\\documentclass{article}\n";
        out += "\\usepackage{bussproofs}\n";
        out += "\\usepackage{amsmath}\n";
        out += "\\begin{document}\n";
        out += &self.to_latex(conf);
        out += "\\end{document}\n";
        out
    }
}

impl Default for LatexConfig {
    fn default() -> LatexConfig {
        LatexConfig {
            include_envs: true,
            use_frac_array: false,
        }
    }
}
