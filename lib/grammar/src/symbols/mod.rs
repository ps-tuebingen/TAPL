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

    /// Create a symbol for an untyped lambda expression
    /// with a given symbol for the body
    /// lambda var.inner
    pub fn lam_untyped(inner: Symbol) -> Symbol {
        vec![
            SpecialChar::Lambda.into(),
            Symbol::Variable,
            SpecialChar::Dot.into(),
            inner,
        ]
        .into()
    }

    /// Create a symbol type annotating a given symbol
    /// sym : Type
    pub fn ty_annot(sym: Symbol) -> Symbol {
        vec![sym, SpecialChar::Colon.into(), Symbol::Type].into()
    }

    /// Create a symbol kind annotating a given symbol
    /// sym :: Kind
    pub fn kind_annot(sym: Symbol) -> Symbol {
        vec![sym, SpecialChar::DoubleColon.into(), Symbol::Kind].into()
    }

    /// Create a symbol subtype annotating a given symbol
    /// sym <: Type
    pub fn subty_annot(sym: Symbol) -> Symbol {
        vec![sym, SpecialChar::LessColon.into(), Symbol::Type].into()
    }

    /// Create a symbol for a lambda abstraction
    /// with given annotation symbol and body symbol
    /// lambda annot.body
    pub fn lam(annot: Symbol, body: Symbol) -> Symbol {
        vec![
            SpecialChar::Lambda.into(),
            annot,
            SpecialChar::Dot.into(),
            body,
        ]
        .into()
    }

    /// Crate a symbol for a mu type
    /// mu var.Type
    pub fn mu() -> Symbol {
        vec![
            SpecialChar::Mu.into(),
            Symbol::Variable,
            SpecialChar::Dot.into(),
            Symbol::Type,
        ]
        .into()
    }

    /// Create a symbol for a packed symbol (existentials)
    ///  {inner,Type} as Type
    pub fn pack(inner: Symbol) -> Symbol {
        vec![
            SpecialChar::BrackO.into(),
            inner,
            SpecialChar::Comma.into(),
            Symbol::Type,
            SpecialChar::BrackC.into(),
        ]
        .into()
    }

    /// Create a symbol for an unpack term
    /// let {TypeVar,Var} = Term in Term
    pub fn unpack() -> Symbol {
        vec![
            SpecialChar::BrackO.into(),
            Symbol::Typevariable,
            SpecialChar::Comma.into(),
            Symbol::Variable,
            SpecialChar::BrackC.into(),
            SpecialChar::Equals.into(),
            Symbol::Term,
            Keyword::In.into(),
            Symbol::Term,
        ]
        .into()
    }

    /// Create a symbol for a let binding
    /// let (var = Term) in Term
    pub fn lett() -> Symbol {
        vec![
            SpecialChar::ParenO.into(),
            Symbol::Variable,
            SpecialChar::Equals.into(),
            Symbol::Term,
            SpecialChar::ParenC.into(),
            Keyword::In.into(),
            Symbol::Term,
        ]
        .into()
    }

    /// Create a symbol for an if expression
    /// if { Term } else { Term }
    pub fn ift() -> Symbol {
        vec![
            Keyword::If.into(),
            SpecialChar::BrackO.into(),
            Symbol::Term,
            SpecialChar::BrackC.into(),
            Keyword::Else.into(),
            SpecialChar::BrackO.into(),
            Symbol::Term,
            SpecialChar::BrackC.into(),
        ]
        .into()
    }

    /// Crate a symbol for a dereference term
    /// !Term
    pub fn dereft() -> Symbol {
        vec![SpecialChar::Exclamation.into(), Symbol::Term].into()
    }

    /// Create a symbol for a try term
    /// Try { Term }
    pub fn tryt() -> Symbol {
        vec![
            Keyword::Try.into(),
            SpecialChar::BrackO.into(),
            Symbol::Term,
            SpecialChar::BrackC.into(),
        ]
        .into()
    }

    /// Crate a symbol for a try-catch term
    /// Try { Term } Catch { Term }
    pub fn try_catch() -> Symbol {
        vec![
            Keyword::Try.into(),
            SpecialChar::BrackO.into(),
            Symbol::Term,
            SpecialChar::BrackC.into(),
            Keyword::Catch.into(),
            SpecialChar::BrackO.into(),
            Symbol::Term,
            SpecialChar::BrackC.into(),
        ]
        .into()
    }

    // Crate a symbol for dot notation
    // Term.op (e.g. Term.fst())
    pub fn dot(op: Symbol) -> Symbol {
        vec![Symbol::Term, SpecialChar::Dot.into(), op].into()
    }

    /// Create a symbol for a constructor
    /// with given constructor name, optional type argument and constructor arguments
    /// ctor(arg1,arg2,...) without type argument
    /// ctor[ty_arg](arg1,arg2,...) with type argument
    pub fn ctor(ctor: Keyword, ty_arg: Option<Symbol>, args: Vec<Symbol>) -> Symbol {
        let mut seq = vec![ctor.into()];

        if let Some(arg) = ty_arg {
            seq.push(SpecialChar::SqBrackO.into());
            seq.push(arg);
            seq.push(SpecialChar::SqBrackC.into());
        }

        seq.push(SpecialChar::ParenO.into());

        let empt = args.is_empty();
        for arg in args {
            seq.push(arg);
            seq.push(SpecialChar::Comma.into());
        }
        if !empt {
            seq.remove(seq.len() - 1);
        }
        seq.push(SpecialChar::ParenC.into());
        seq.into()
    }

    /// Create a symbol for a case expression with givven patterns
    /// case Term of { pts }
    pub fn case(pts: Vec<Symbol>) -> Symbol {
        let mut seq = vec![
            Keyword::Case.into(),
            Symbol::Term,
            Keyword::Of.into(),
            SpecialChar::BrackO.into(),
        ];
        seq.extend(pts);
        seq.push(SpecialChar::SqBrackC.into());
        seq.into()
    }

    // Create a symbol for a constructor pattern
    // with given contstructor and number of arguments
    // ctor(x1,x2,...,x_(num_args))
    pub fn ctor_pt(ctor: Keyword, num_args: usize) -> Symbol {
        Symbol::ctor(
            ctor,
            None,
            (0..num_args).map(|_| Symbol::Variable).collect(),
        )
    }

    /// Create a symbol for a pattern with given constructor and right-hand side
    /// ctor => rhs
    pub fn pt(ctor: Symbol, rhs: Symbol) -> Symbol {
        vec![ctor, SpecialChar::DoubleArrow.into(), rhs].into()
    }

    /// Create a symbol for a variant term/type
    /// with given inner symbol on the rhs of labels
    /// <label=inner,...>
    pub fn variant(inner: Symbol) -> Symbol {
        vec![
            SpecialChar::AngBrackO.into(),
            Symbol::Many(Box::new(
                vec![Symbol::Label, SpecialChar::Equals.into(), inner].into(),
            )),
            SpecialChar::AngBrackC.into(),
        ]
        .into()
    }

    /// Create a symbol for a tuple term/type
    /// with a given inner symbol (usually term/type)
    /// (inner,...)
    pub fn tuple(inner: Symbol) -> Symbol {
        vec![
            SpecialChar::ParenO.into(),
            Symbol::Many(Box::new(inner)),
            SpecialChar::ParenC.into(),
        ]
        .into()
    }

    /// Crate a symbol for a pair term/type
    /// with given inner symbol (usually term/type)
    /// [inner,inner...]
    pub fn pair(inner: Symbol) -> Symbol {
        vec![
            SpecialChar::BrackO.into(),
            Symbol::Many(Box::new(inner)),
            SpecialChar::BrackC.into(),
        ]
        .into()
    }

    /// Create a symbol for a record term/type
    /// with given inner symbol (usually term/type)
    /// { Label = inner,...}
    pub fn record(inner: Symbol) -> Symbol {
        vec![
            SpecialChar::BrackO.into(),
            Symbol::Many(Box::new(
                vec![Symbol::Label, SpecialChar::Equals.into(), inner].into(),
            )),
            SpecialChar::BrackC.into(),
        ]
        .into()
    }

    /// Create a label for a sum type
    /// Type + Type
    pub fn sum_ty() -> Symbol {
        vec![Symbol::Type, SpecialChar::Plus.into(), Symbol::Type].into()
    }

    /// Crate a symbol for a product type
    /// Type x Type
    pub fn product_ty() -> Symbol {
        vec![Symbol::Type, SpecialChar::Times.into(), Symbol::Type].into()
    }

    /// Create a symbol for a function type
    /// Type -> Type
    pub fn fun_ty() -> Symbol {
        vec![Symbol::Type, SpecialChar::Arrow.into(), Symbol::Type].into()
    }

    /// Create a symbol for an application with given function and argument
    /// fun arg
    pub fn app(fun: Symbol, arg: Symbol) -> Symbol {
        vec![fun, arg].into()
    }

    /// Create a symbol for an assignment Term
    /// Term := Term
    pub fn assign() -> Symbol {
        vec![Symbol::Term, SpecialChar::ColonEq.into(), Symbol::Term].into()
    }

    /// Create a symbol for a type cast
    /// Term as Type
    pub fn cast() -> Symbol {
        vec![Symbol::Term, Keyword::As.into(), Symbol::Term].into()
    }

    /// Create a symbol for a universal type
    /// with given symbol annotating the bound variable
    /// forall annot.Type
    pub fn forall_ty(annot: Symbol) -> Symbol {
        vec![
            SpecialChar::Forall.into(),
            annot,
            SpecialChar::Dot.into(),
            Symbol::Type,
        ]
        .into()
    }

    /// Create a symbol for an existential type
    /// with given symbol annotating the bound variable
    /// exists annot.Type
    pub fn exists_ty(annot: Symbol) -> Symbol {
        vec![
            SpecialChar::Exists.into(),
            annot,
            SpecialChar::Dot.into(),
            Symbol::Type,
        ]
        .into()
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
        Symbol::Seq(syms)
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
