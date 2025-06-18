use crate::{
    errors::{ParserError, UnknownKeyword},
    Rule,
};
use pest::iterators::Pair;
use syntax::types::{Bool, Bot, Nat, Type, Unit};

pub struct StringTy<Ty>
where
    Ty: Type,
{
    nat: Option<Ty>,
    bool: Option<Ty>,
    unit: Option<Ty>,
    bot: Option<Ty>,
}

impl<Ty> StringTy<Ty>
where
    Ty: Type,
{
    pub fn new() -> StringTy<Ty> {
        StringTy {
            nat: None,
            bool: None,
            unit: None,
            bot: None,
        }
    }

    pub fn with_bot(self) -> StringTy<Ty>
    where
        Bot: Into<Ty>,
    {
        StringTy {
            bot: Some(Bot::new().into()),
            bool: self.bool,
            unit: self.unit,
            nat: self.nat,
        }
    }

    pub fn with_nat(self) -> StringTy<Ty>
    where
        Nat<Ty>: Into<Ty>,
    {
        StringTy {
            bot: self.bot,
            nat: Some(Nat::new().into()),
            bool: self.bool,
            unit: self.unit,
        }
    }

    pub fn with_bool(self) -> StringTy<Ty>
    where
        Bool<Ty>: Into<Ty>,
    {
        StringTy {
            bot: self.bot,
            nat: self.nat,
            bool: Some(Bool::new().into()),
            unit: self.unit,
        }
    }

    pub fn with_unit(self) -> StringTy<Ty>
    where
        Unit<Ty>: Into<Ty>,
    {
        StringTy {
            bot: self.bot,
            nat: self.nat,
            bool: self.bool,
            unit: Some(Unit::new().into()),
        }
    }

    pub fn from_pair(self, p: Pair<'_, Rule>) -> Result<Ty, ParserError> {
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
