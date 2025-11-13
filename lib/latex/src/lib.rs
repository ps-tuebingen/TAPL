mod conclusion;
mod definition;
mod derivations;
mod env;
mod grammar;
mod kind;
mod program;
mod terms;
mod traces;
mod types;
mod untyped;
mod values;

#[derive(Debug)]
pub struct LatexConfig {
    pub include_envs: bool,
    pub use_frac_array: bool,
}

impl LatexConfig {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn new_frac() -> Self {
        Self {
            include_envs: true,
            use_frac_array: true,
        }
    }

    #[must_use]
    pub fn mathenv_strs(&self) -> (String, String) {
        if self.include_envs {
            ("\\[".to_owned(), "\\]".to_owned())
        } else {
            (String::new(), String::new())
        }
    }
}

pub trait LatexFmt {
    fn to_latex(&self, conf: &mut LatexConfig) -> String;
    fn to_document(&self, conf: &mut LatexConfig) -> String {
        let mut out = String::new();
        out += "\\documentclass{article}\n";
        out += "\\usepackage{bussproofs}\n";
        out += "\\usepackage{amsmath}\n";
        out += "\\begin{document}\n";
        out += &self.to_latex(conf);
        out += "\n\\end{document}\n";
        out
    }
}

impl LatexFmt for String {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (start, end) = conf.mathenv_strs();
        format!("{}\\text{{{}}}{}", start, self.replace('_', "\\_"), end)
    }
}

impl Default for LatexConfig {
    fn default() -> Self {
        Self {
            include_envs: true,
            use_frac_array: false,
        }
    }
}
