use crate::formattable::Formattable;
use errors::driver_error::DriverError;
use latex::LatexConfig;
use std::{fmt, str::FromStr};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn format<T>(&self, t: &T) -> String
    where
        T: Formattable,
    {
        match self {
            Self::Simple => t.to_string(),
            Self::LatexBusStripped => t.to_latex(&mut LatexConfig::new()),
            Self::LatexBusDoc => t.to_document(&mut LatexConfig::new()),
            Self::LatexFracStripped => t.to_latex(&mut LatexConfig::new_frac()),
            Self::LatexFracDoc => t.to_document(&mut LatexConfig::new_frac()),
            Self::Debug => format!("{t:?}"),
        }
    }
}

impl FromStr for FormatMethod {
    type Err = DriverError;
    fn from_str(s: &str) -> Result<Self, DriverError> {
        let s = s.replace(['-', '_'], "");
        match s.to_lowercase().trim() {
            "print" | "simple" => Ok(Self::Simple),
            "latex" | "buss" | "latexbuss" => Ok(Self::LatexBusDoc),
            "latexstripped" | "bussstripped" => Ok(Self::LatexBusStripped),
            "frac" | "fracarray" => Ok(Self::LatexFracDoc),
            "fracstripped" | "fracarraystripped" | "latexfrac" => Ok(Self::LatexFracStripped),
            "debug" | "dbg" => Ok(Self::Debug),
            _ => Err(DriverError::UndefinedFormatMethod(s.clone())),
        }
    }
}

impl fmt::Display for FormatMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Simple => f.write_str("simple"),
            Self::LatexBusStripped => f.write_str("buss-stripped"),
            Self::LatexBusDoc => f.write_str("buss"),
            Self::LatexFracStripped => f.write_str("frac-stripped"),
            Self::LatexFracDoc => f.write_str("frac"),
            Self::Debug => f.write_str("debug"),
        }
    }
}
