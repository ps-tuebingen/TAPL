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
            FormatMethod::Simple => t.to_string(),
            FormatMethod::LatexBusStripped => t.to_latex(&mut LatexConfig::new()),
            FormatMethod::LatexBusDoc => t.to_document(&mut LatexConfig::new()),
            FormatMethod::LatexFracStripped => t.to_latex(&mut LatexConfig::new_frac()),
            FormatMethod::LatexFracDoc => t.to_document(&mut LatexConfig::new_frac()),
            FormatMethod::Debug => format!("{t:?}"),
        }
    }
}

impl FromStr for FormatMethod {
    type Err = DriverError;
    fn from_str(s: &str) -> Result<FormatMethod, DriverError> {
        let s = s.replace(['-', '_'], "");
        match s.to_lowercase().trim() {
            "print" | "simple" => Ok(FormatMethod::Simple),
            "latex" | "buss" | "latexbuss" => Ok(FormatMethod::LatexBusDoc),
            "latexstripped" | "bussstripped" => Ok(FormatMethod::LatexBusStripped),
            "frac" | "fracarray" => Ok(FormatMethod::LatexFracDoc),
            "fracstripped" | "fracarraystripped" | "latexfrac" => {
                Ok(FormatMethod::LatexFracStripped)
            }
            "debug" | "dbg" => Ok(FormatMethod::Debug),
            _ => Err(DriverError::UndefinedFormatMethod(s.to_owned())),
        }
    }
}

impl fmt::Display for FormatMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormatMethod::Simple => f.write_str("simple"),
            FormatMethod::LatexBusStripped => f.write_str("buss-stripped"),
            FormatMethod::LatexBusDoc => f.write_str("buss"),
            FormatMethod::LatexFracStripped => f.write_str("frac-stripped"),
            FormatMethod::LatexFracDoc => f.write_str("frac"),
            FormatMethod::Debug => f.write_str("debug"),
        }
    }
}
