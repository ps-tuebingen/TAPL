use crate::{GroupParse, Rule, pair_span};
use errors::{UnknownKeyword, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::{
    language::Language,
    span::Span,
    types::{Bool, Bot, Nat, Unit},
};

/// Helper struct to parse primitive types
/// each field is `None` when `Lang` does not include the given type and `Some` when it does
pub struct StringTy<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    /// [`Nat`]
    nat: Option<Lang::Type>,
    /// [`Bool`]
    bool: Option<Lang::Type>,
    /// [`Unit`]
    unit: Option<Lang::Type>,
    /// [`Bot`]
    bot: Option<Lang::Type>,
}

impl<Lang> StringTy<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    /// Create `Self` with no allowed types
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nat: None,
            bool: None,
            unit: None,
            bot: None,
        }
    }

    /// Add [`Bot`] to allowed types, with given span
    #[must_use]
    pub fn with_bot(self, span: Span) -> Self
    where
        Bot<Lang>: Into<Lang::Type>,
    {
        Self {
            bot: Some(Bot::new(span).into()),
            bool: self.bool,
            unit: self.unit,
            nat: self.nat,
        }
    }

    /// add [`Nat`] to allowed types with given apsn
    #[must_use]
    pub fn with_nat(self, span: Span) -> Self
    where
        Nat<Lang>: Into<Lang::Type>,
    {
        Self {
            bot: self.bot,
            nat: Some(Nat::new(span).into()),
            bool: self.bool,
            unit: self.unit,
        }
    }

    /// Add [`Bool`] to allowed types, with given span
    #[must_use]
    pub fn with_bool(self, span: Span) -> Self
    where
        Bool<Lang>: Into<Lang::Type>,
    {
        Self {
            bot: self.bot,
            nat: self.nat,
            bool: Some(Bool::new(span).into()),
            unit: self.unit,
        }
    }

    /// Add [`Unit`] to allowed types
    #[must_use]
    pub fn with_unit(self, span: Span) -> Self
    where
        Unit<Lang>: Into<Lang::Type>,
    {
        Self {
            bot: self.bot,
            nat: self.nat,
            bool: self.bool,
            unit: Some(Unit::new(span).into()),
        }
    }

    /// Parse a string type from a pair
    /// behaves like [`crate::Parse::from_pair`] but requires `self` to already be configured
    /// # Errors
    /// returns an error if the parsed value does not correspond to a primitive type
    /// or when `self` does not allow this type
    pub fn from_pair(self, p: &Pair<'_, Rule>) -> Result<Lang::Type, ParserError> {
        let span = pair_span(&p);
        let err = UnknownKeyword::new(p.as_str(), span).into();
        match p.as_str().to_lowercase().trim() {
            "bot" => self.bot.map_or_else(|| Err(err), Ok),
            "nat" => self.nat.map_or_else(|| Err(err), Ok),
            "bool" => self.bool.map_or_else(|| Err(err), Ok),
            "unit" => self.unit.map_or_else(|| Err(err), Ok),
            _ => Err(err),
        }
    }
}

impl<Lang> Default for StringTy<Lang>
where
    Lang: Language,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn default() -> Self {
        Self::new()
    }
}
