use crate::{GroupParse, ParsableLanguage, Rule};
use errors::{UnknownKeyword, parse_error::ParserError};
use pest::iterators::Pair;
use syntax::types::{Bool, Bot, Nat, Unit};

pub struct StringTy<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    nat: Option<Lang::Type>,
    bool: Option<Lang::Type>,
    unit: Option<Lang::Type>,
    bot: Option<Lang::Type>,
}

impl<Lang> StringTy<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    pub fn new() -> StringTy<Lang> {
        StringTy {
            nat: None,
            bool: None,
            unit: None,
            bot: None,
        }
    }

    pub fn with_bot(self) -> StringTy<Lang>
    where
        Bot<Lang>: Into<Lang::Type>,
    {
        StringTy {
            bot: Some(Bot::new().into()),
            bool: self.bool,
            unit: self.unit,
            nat: self.nat,
        }
    }

    pub fn with_nat(self) -> StringTy<Lang>
    where
        Nat<Lang>: Into<Lang::Type>,
    {
        StringTy {
            bot: self.bot,
            nat: Some(Nat::new().into()),
            bool: self.bool,
            unit: self.unit,
        }
    }

    pub fn with_bool(self) -> StringTy<Lang>
    where
        Bool<Lang>: Into<Lang::Type>,
    {
        StringTy {
            bot: self.bot,
            nat: self.nat,
            bool: Some(Bool::new().into()),
            unit: self.unit,
        }
    }

    pub fn with_unit(self) -> StringTy<Lang>
    where
        Unit<Lang>: Into<Lang::Type>,
    {
        StringTy {
            bot: self.bot,
            nat: self.nat,
            bool: self.bool,
            unit: Some(Unit::new().into()),
        }
    }

    pub fn from_pair(self, p: Pair<'_, Rule>) -> Result<Lang::Type, ParserError> {
        let err = UnknownKeyword::new(p.as_str()).into();
        match p.as_str().to_lowercase().trim() {
            "bot" => {
                if let Some(b) = self.bot {
                    Ok(b)
                } else {
                    Err(err)
                }
            }
            "nat" => {
                if let Some(n) = self.nat {
                    Ok(n)
                } else {
                    Err(err)
                }
            }
            "bool" => {
                if let Some(b) = self.bool {
                    Ok(b)
                } else {
                    Err(err)
                }
            }
            "unit" => {
                if let Some(u) = self.unit {
                    Ok(u)
                } else {
                    Err(err)
                }
            }
            _ => Err(err),
        }
    }
}

impl<Lang> Default for StringTy<Lang>
where
    Lang: ParsableLanguage,
    Lang::Term: GroupParse,
    Lang::Type: GroupParse,
{
    fn default() -> StringTy<Lang> {
        StringTy::new()
    }
}
