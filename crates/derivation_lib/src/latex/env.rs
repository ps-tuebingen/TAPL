use super::{LatexConfig, LatexFmt};
use syntax::{env::Environment, types::Type};

impl<Ty> LatexFmt for Environment<Ty>
where
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let mut out = "".to_owned();

        out += &self
            .var_bindings
            .iter()
            .map(|(var, ty)| format!("{var}:{}", ty.to_latex(conf)))
            .collect::<Vec<String>>()
            .join(", ");

        if !out.is_empty() {
            out += ";";
        }

        out += &self
            .tyvar_bindings
            .iter()
            .map(|(tyvar, knd)| format!("{tyvar}::{}", knd.to_latex(conf)))
            .collect::<Vec<String>>()
            .join(", ");

        if !out.is_empty() {
            out += ";";
        }

        out += &self
            .tyvar_super
            .iter()
            .map(|(tyvar, sup)| format!("{tyvar} <: {}", sup.to_latex(conf)))
            .collect::<Vec<String>>()
            .join(", ");

        if !out.is_empty() {
            out += " \\bar";
        }

        out += &self
            .location_bindings
            .iter()
            .map(|(loc, ty)| format!("{loc}:{}", ty.to_latex(conf)))
            .collect::<Vec<String>>()
            .join(", ");

        out
    }
}
