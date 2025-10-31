mod keywords;
mod special_char;

pub use keywords::Keyword;
pub use special_char::SpecialChar;
use std::{fmt, iter::Iterator};

/// Symbols used to define [`crate::Rule`],[`crate::Grammar`] and [`crate::DerivationRule`]
/// These can be printed either as textual (e.g. Gamma, T,Ty,etc)
/// or as latex (e.g. `\Gamma`,`\tau`,etc)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Symbol {
    /// any number of the same symbol
    /// usually indicated by overline or ...
    /// e.g. clauses in a pattern match
    Many(Box<Symbol>),

    /// A constant String
    Str(String),
    /// A constant integer
    Int(i64),
    /// A Keyword symbol
    Keyword(Keyword),
    /// A special character
    SpecialChar(SpecialChar),
    /// Symbol for terms (`t`)
    Term,
    /// Symbol for types (`ty`/`tau`)
    Type,
    /// Symbol for kinds (`K`/`kappa`)
    Kind,
    /// Symbol for values (`V`)
    Value,

    /// Symbol for variables
    Variable,
    /// Symbol for type variables
    Typevariable,
    /// Symbols for labels (e.g. in sum types)
    Label,
    /// Symbols for locations (e.g. in references)
    Location,

    /// Symbol with index
    Subscript {
        /// the symbol with subscript
        sym: Box<Symbol>,
        /// the index
        ind: Box<Symbol>,
    },

    /// List of sequential symbols
    Seq(Vec<Symbol>),
}

impl Symbol {
    /// Create [`Symbol::Many`] with a given inner symbol
    pub fn many<S>(inner: S) -> Symbol
    where
        S: Into<Symbol>,
    {
        Symbol::Many(Box::new(inner.into()))
    }

    /// Create [`Symbol::Subscript`] with given symbol and subscript
    pub fn sub<S1, S2>(sym: S1, ind: S2) -> Symbol
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        Symbol::Subscript {
            sym: Box::new(sym.into()),
            ind: Box::new(ind.into()),
        }
    }

    /// Wrap S into parentheses
    /// (inner)
    pub fn paren<S>(inner: S) -> Symbol
    where
        S: Into<Symbol>,
    {
        vec![
            SpecialChar::ParenO.into(),
            inner.into(),
            SpecialChar::ParenC.into(),
        ]
        .into()
    }

    /// Wrap S ino brackets
    /// { inner }
    pub fn brack<S>(inner: S) -> Symbol
    where
        S: Into<Symbol>,
    {
        vec![
            SpecialChar::BrackO.into(),
            inner.into(),
            SpecialChar::BrackC.into(),
        ]
        .into()
    }

    /// wrap S in square brackets
    /// [inner]
    pub fn sqbrack<S>(inner: S) -> Symbol
    where
        S: Into<Symbol>,
    {
        vec![
            SpecialChar::SqBrackO.into(),
            inner.into(),
            SpecialChar::SqBrackC.into(),
        ]
        .into()
    }

    /// Wrap S into angled brackets
    /// <inner>
    pub fn angbrack<S>(inner: S) -> Symbol
    where
        S: Into<Symbol>,
    {
        vec![
            SpecialChar::AngBrackO.into(),
            inner.into(),
            SpecialChar::AngBrackC.into(),
        ]
        .into()
    }

    /// Separate fst and snd by comma
    /// fst, snd
    pub fn comma_sep<S1, S2>(fst: S1, snd: S2) -> Symbol
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        vec![fst.into(), SpecialChar::Comma.into(), snd.into()].into()
    }

    /// Separate fst and snd by arrow
    /// fst -> snd
    pub fn arrow<S1, S2>(fst: S1, snd: S2) -> Symbol
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        vec![fst.into(), SpecialChar::Arrow.into(), snd.into()].into()
    }

    /// Separate fst and snd by mapsto
    /// fst --> snd
    pub fn mapto<S1, S2>(fst: S1, snd: S2) -> Symbol
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        vec![fst.into(), SpecialChar::Mapsto.into(), snd.into()].into()
    }

    /// Separate fst and snd by colon
    /// fst : snd
    pub fn colon_sep<S1, S2>(fst: S1, snd: S2) -> Symbol
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        vec![fst.into(), SpecialChar::Colon.into(), snd.into()].into()
    }

    /// Separate fst and snd by double colon
    /// fst :: snd
    pub fn double_colon_sep<S1, S2>(fst: S1, snd: S2) -> Symbol
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        vec![fst.into(), SpecialChar::DoubleColon.into(), snd.into()].into()
    }

    pub fn less_colon_sep<S1, S2>(fst: S1, snd: S2) -> Symbol
    where
        S1: Into<Symbol>,
        S2: Into<Symbol>,
    {
        vec![fst.into(), SpecialChar::LessColon.into(), snd.into()].into()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Many(sym) => write!(f, "{sym},..."),
            Symbol::Keyword(kw) => kw.fmt(f),
            Symbol::SpecialChar(sc) => sc.fmt(f),
            Symbol::Term => f.write_str("t"),
            Symbol::Type => f.write_str("T"),
            Symbol::Kind => f.write_str("K"),
            Symbol::Value => f.write_str("v"),
            Symbol::Variable => f.write_str("x"),
            Symbol::Typevariable => f.write_str("X"),
            Symbol::Label => f.write_str("lb"),
            Symbol::Location => f.write_str("loc"),
            Symbol::Subscript { sym, ind } => write!(f, "{sym}_{ind}"),
            Symbol::Seq(syms) => write!(
                f,
                "{}",
                syms.iter()
                    .map(|sym| sym.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
            Symbol::Str(s) => write!(f, "{s}"),
            Symbol::Int(i) => write!(f, "{i}"),
        }
    }
}

impl From<Vec<Symbol>> for Symbol {
    fn from(syms: Vec<Symbol>) -> Symbol {
        let mut sym_list = vec![];
        for sym in syms {
            if let Symbol::Seq(ls) = sym {
                sym_list.extend(ls);
            } else {
                sym_list.push(sym)
            }
        }
        Symbol::Seq(sym_list)
    }
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Symbol {
        Symbol::Str(s.to_owned())
    }
}

impl From<i64> for Symbol {
    fn from(i: i64) -> Symbol {
        Symbol::Int(i)
    }
}
