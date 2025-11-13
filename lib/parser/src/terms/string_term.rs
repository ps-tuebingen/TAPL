use crate::{GroupParse, Rule};
use errors::{UnknownKeyword, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::{
    language::Language,
    terms::{False, Num, True, Unit},
};

/// Helper struct to parse primitive types (unit,zero,...)
/// if a field is present it can be parsed, otherwise it returns an error
/// each language needs to configure depending on the available terms
pub struct StringTerm<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    /// `Lang` contains [`Unit`]
    unit: Option<Lang::Term>,
    /// `Lang` contains [`Num`] (0)
    zero: Option<Lang::Term>,
    /// `Lang` contains [`False`]
    fls: Option<Lang::Term>,
    /// `Lang` contains [`True`]
    tru: Option<Lang::Term>,
}

impl<Lang> StringTerm<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    /// Create a new `Self` with no terms added
    #[must_use]
    pub const fn new() -> Self {
        Self {
            unit: None,
            zero: None,
            fls: None,
            tru: None,
        }
    }

    /// Add [`Unit`] to `Self`
    #[must_use]
    pub fn with_unit(self) -> Self
    where
        Unit<Lang>: Into<Lang::Term>,
    {
        Self {
            unit: Some(Unit::new().into()),
            zero: self.zero,
            fls: self.fls,
            tru: self.tru,
        }
    }

    /// Add [`Num`](0) to `Self`
    #[must_use]
    pub fn with_zero(self) -> Self
    where
        Num<Lang>: Into<Lang::Term>,
    {
        Self {
            unit: self.unit,
            zero: Some(Num::new(0).into()),
            fls: self.fls,
            tru: self.tru,
        }
    }

    /// Add [`False`] to `Self`
    #[must_use]
    pub fn with_false(self) -> Self
    where
        False<Lang>: Into<Lang::Term>,
    {
        Self {
            unit: self.unit,
            zero: self.zero,
            fls: Some(False::new().into()),
            tru: self.tru,
        }
    }

    /// Add [`True`] to `Self`
    #[must_use]
    pub fn with_true(self) -> Self
    where
        True<Lang>: Into<Lang::Term>,
    {
        Self {
            unit: self.unit,
            zero: self.zero,
            fls: self.fls,
            tru: Some(True::new().into()),
        }
    }

    /// Parse a string term from a pair
    /// similar to [`crate::Parse::from_pair`] but requires self to be configured first
    /// # Errors
    /// returns an error if the pair does not correspond to a primitive term
    /// or if the term is not part of `Self`
    pub fn from_pair(self, p: &Pair<'_, Rule>) -> Result<Lang::Term, ParserError> {
        let err = UnknownKeyword::new(p.as_str()).into();
        match p.as_str().to_lowercase().trim() {
            "unit" => self.unit.map_or_else(|| Err(err), Ok),
            "zero" => self.zero.map_or_else(|| Err(err), Ok),
            "false" => self.fls.map_or_else(|| Err(err), Ok),
            "true" => self.tru.map_or_else(|| Err(err), Ok),
            _ => Err(err),
        }
    }
}

impl<Lang> Default for StringTerm<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn default() -> Self {
        Self::new()
    }
}
