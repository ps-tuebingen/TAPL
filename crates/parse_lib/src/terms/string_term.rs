use crate::Rule;
use errors::{UnknownKeyword, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::terms::{False, Num, Term, True, Unit};

pub struct StringTerm<T>
where
    T: Term,
{
    unit: Option<T>,
    zero: Option<T>,
    fls: Option<T>,
    tru: Option<T>,
}

impl<T> StringTerm<T>
where
    T: Term,
{
    pub fn new() -> StringTerm<T> {
        StringTerm {
            unit: None,
            zero: None,
            fls: None,
            tru: None,
        }
    }

    pub fn with_unit(self) -> StringTerm<T>
    where
        Unit<T>: Into<T>,
    {
        StringTerm {
            unit: Some(Unit::new().into()),
            zero: self.zero,
            fls: self.fls,
            tru: self.tru,
        }
    }

    pub fn with_zero(self) -> StringTerm<T>
    where
        Num<T>: Into<T>,
    {
        StringTerm {
            unit: self.unit,
            zero: Some(Num::new(0).into()),
            fls: self.fls,
            tru: self.tru,
        }
    }

    pub fn with_false(self) -> StringTerm<T>
    where
        False<T>: Into<T>,
    {
        StringTerm {
            unit: self.unit,
            zero: self.zero,
            fls: Some(False::new().into()),
            tru: self.tru,
        }
    }

    pub fn with_true(self) -> StringTerm<T>
    where
        True<T>: Into<T>,
    {
        StringTerm {
            unit: self.unit,
            zero: self.zero,
            fls: self.fls,
            tru: Some(True::new().into()),
        }
    }

    pub fn from_pair(self, p: Pair<'_, Rule>) -> Result<T, ParserError> {
        let err = UnknownKeyword::new(p.as_str()).into();
        match p.as_str().to_lowercase().trim() {
            "unit" => {
                if let Some(u) = self.unit {
                    Ok(u)
                } else {
                    Err(err)
                }
            }
            "zero" => {
                if let Some(z) = self.zero {
                    Ok(z)
                } else {
                    Err(err)
                }
            }
            "false" => {
                if let Some(f) = self.fls {
                    Ok(f)
                } else {
                    Err(err)
                }
            }
            "true" => {
                if let Some(t) = self.tru {
                    Ok(t)
                } else {
                    Err(err)
                }
            }
            _ => Err(err),
        }
    }
}

impl<T> Default for StringTerm<T>
where
    T: Term,
{
    fn default() -> StringTerm<T> {
        StringTerm::new()
    }
}
