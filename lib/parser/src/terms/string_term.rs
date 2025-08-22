use crate::{GroupParse, ParsableLanguage, Rule};
use errors::{UnknownKeyword, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::terms::{False, Num, True, Unit};

pub struct StringTerm<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    unit: Option<Lang::Term>,
    zero: Option<Lang::Term>,
    fls: Option<Lang::Term>,
    tru: Option<Lang::Term>,
}

impl<Lang> StringTerm<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    pub fn new() -> StringTerm<Lang> {
        StringTerm {
            unit: None,
            zero: None,
            fls: None,
            tru: None,
        }
    }

    pub fn with_unit(self) -> StringTerm<Lang>
    where
        Unit<Lang>: Into<Lang::Term>,
    {
        StringTerm {
            unit: Some(Unit::new().into()),
            zero: self.zero,
            fls: self.fls,
            tru: self.tru,
        }
    }

    pub fn with_zero(self) -> StringTerm<Lang>
    where
        Num<Lang>: Into<Lang::Term>,
    {
        StringTerm {
            unit: self.unit,
            zero: Some(Num::new(0).into()),
            fls: self.fls,
            tru: self.tru,
        }
    }

    pub fn with_false(self) -> StringTerm<Lang>
    where
        False<Lang>: Into<Lang::Term>,
    {
        StringTerm {
            unit: self.unit,
            zero: self.zero,
            fls: Some(False::new().into()),
            tru: self.tru,
        }
    }

    pub fn with_true(self) -> StringTerm<Lang>
    where
        True<Lang>: Into<Lang::Term>,
    {
        StringTerm {
            unit: self.unit,
            zero: self.zero,
            fls: self.fls,
            tru: Some(True::new().into()),
        }
    }

    pub fn from_pair(self, p: Pair<'_, Rule>) -> Result<Lang::Term, ParserError> {
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

impl<Lang> Default for StringTerm<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn default() -> StringTerm<Lang> {
        StringTerm::new()
    }
}
