use latex::{LatexConfig, LatexFmt};
use std::fmt;

pub trait Formattable: fmt::Display + fmt::Debug + LatexFmt {}

#[derive(Default)]
pub enum FormatMethod {
    #[default]
    Simple,
    LatexBusStripped,
    LatexBusDoc,
    LatexFracStripped,
    LatexFracDoc,
    Debug,
}

impl FormatMethod {
    fn format<T>(&self, t: &T) -> String
    where
        T: Formattable,
    {
        match self {
            FormatMethod::Simple => t.to_string(),
            FormatMethod::LatexBusStripped => t.to_latex(&mut LatexConfig::new()),
            FormatMethod::LatexBusDoc => t.to_document(&mut LatexConfig::new()),
            FormatMethod::LatexFracStripped => t.to_latex(&mut LatexConfig::new_frac()),
            FormatMethod::LatexFracDoc => t.to_document(&mut LatexConfig::new_frac()),
            FormatMethod::Debug => format!("{t:?}"),
        }
    }
}
