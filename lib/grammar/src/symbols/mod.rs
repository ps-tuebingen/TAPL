mod keywords;
mod special_char;

pub use keywords::Keyword;
pub use special_char::SpecialChar;
use std::fmt;

/// Symbols used to define [`crate::Rule`],[`crate::Grammar`] and [`crate::DerivationRule`]
/// These can be printed either as textual (e.g. Gamma, T,Ty,etc)
/// or as latex (e.g. `\Gamma`,`\tau`,etc)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Symbol {
    /// any number of the same symbol
    /// usually indicated by overline or ...
    /// e.g. clauses in a pattern match
    Many(Box<Symbol>),

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

    /// A Symbol with a prefix
    Prefixed {
        /// The prefix of the symbol
        prefix: Box<Symbol>,
        /// The symbol being prefixed
        inner: Box<Symbol>,
    },
    /// A Symbol inside delimiters (e.g. parentheses)
    Delim {
        /// The opening delimiter (e.g. `(`)
        delim_open: SpecialChar,
        /// The symbol inside delimiters
        inner: Box<Symbol>,
        /// The closing delimiter (e.g. `)`)
        delim_close: SpecialChar,
    },
    /// Two symbols being separated by a symbol
    Separated {
        /// The first symbol
        fst: Box<Symbol>,
        /// The separator between them
        separator: Box<Symbol>,
        /// The second symbol
        snd: Box<Symbol>,
    },
    /// Symbol for case expressions
    Case {
        /// The bound symbol (e.g. `case t of ...`)
        bound: Box<Symbol>,
        /// Symbols for each pattern
        patterns: Vec<Symbol>,
    },
    /// Symbol for patterns (e.g. `Nil => t`)
    Pattern {
        /// The left hand side of the pattern
        lhs: Box<Symbol>,
        /// The right hand side of the pattern
        rhs: Box<Symbol>,
    },
}

impl Symbol {
    /// Create a symbol for an untyped lambda expression
    /// with a given symbol for the body
    /// lambda var.inner
    pub fn lam_untyped(inner: Symbol) -> Symbol {
        Symbol::Prefixed {
            prefix: Box::new(SpecialChar::Lambda.into()),
            inner: Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Variable),
                separator: Box::new(SpecialChar::Dot.into()),
                snd: Box::new(inner),
            }),
        }
    }

    /// Create a symbol type annotating a given symbol
    /// sym : Type
    pub fn ty_annot(sym: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(sym),
            separator: Box::new(SpecialChar::Colon.into()),
            snd: Box::new(Symbol::Type),
        }
    }

    /// Create a symbol kind annotating a given symbol
    /// sym :: Kind
    pub fn kind_annot(sym: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(sym),
            separator: Box::new(SpecialChar::DoubleColon.into()),
            snd: Box::new(Symbol::Kind),
        }
    }

    /// Create a symbol subtype annotating a given symbol
    /// sym <: Type
    pub fn subty_annot(sym: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(sym),
            separator: Box::new(SpecialChar::LessColon.into()),
            snd: Box::new(Symbol::Type),
        }
    }

    /// Create a symbol for a lambda abstraction
    /// with given annotation symbol and body symbol
    /// lambda annot.body
    pub fn lam(annot: Symbol, body: Symbol) -> Symbol {
        Symbol::Prefixed {
            prefix: Box::new(SpecialChar::Lambda.into()),
            inner: Box::new(Symbol::Separated {
                fst: Box::new(annot),
                separator: Box::new(SpecialChar::Dot.into()),
                snd: Box::new(body),
            }),
        }
    }

    /// Crate a symbol for a mu type
    /// mu var.Type
    pub fn mu() -> Symbol {
        Symbol::Prefixed {
            prefix: Box::new(SpecialChar::Mu.into()),
            inner: Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Variable),
                separator: Box::new(SpecialChar::Dot.into()),
                snd: Box::new(Symbol::Type),
            }),
        }
    }

    /// Create a symbol for a packed symbol (existentials)
    ///  {inner,Type} as Type
    pub fn pack(inner: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Delim {
                delim_open: SpecialChar::BrackO,
                inner: Box::new(Symbol::Separated {
                    fst: Box::new(inner),
                    separator: Box::new(SpecialChar::Comma.into()),
                    snd: Box::new(Symbol::Type),
                }),
                delim_close: SpecialChar::BrackC,
            }),
            separator: Box::new(Keyword::As.into()),
            snd: Box::new(Symbol::Type),
        }
    }

    /// Create a symbol for an unpack term
    /// let {TypeVar,Var} = Term in Term
    pub fn unpack() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Prefixed {
                prefix: Box::new(Keyword::Let.into()),
                inner: Box::new(Symbol::Separated {
                    fst: Box::new(Symbol::Delim {
                        delim_open: SpecialChar::BrackO,
                        inner: Box::new(Symbol::Separated {
                            fst: Box::new(Symbol::Typevariable),
                            separator: Box::new(SpecialChar::Comma.into()),
                            snd: Box::new(Symbol::Variable),
                        }),
                        delim_close: SpecialChar::BrackC,
                    }),
                    separator: Box::new(SpecialChar::Equals.into()),
                    snd: Box::new(Symbol::Term),
                }),
            }),
            separator: Box::new(Keyword::In.into()),
            snd: Box::new(Symbol::Term),
        }
    }

    /// Create a symbol for a let binding
    /// let (var = Term) in Term
    pub fn lett() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Prefixed {
                prefix: Box::new(Keyword::Let.into()),
                inner: Box::new(Symbol::Delim {
                    delim_open: SpecialChar::ParenO,
                    inner: Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Variable),
                        separator: Box::new(SpecialChar::Equals.into()),
                        snd: Box::new(Symbol::Term),
                    }),
                    delim_close: SpecialChar::ParenC,
                }),
            }),
            separator: Box::new(Keyword::In.into()),
            snd: Box::new(Symbol::Term),
        }
    }

    /// Create a symbol for an if expression
    /// if { Term } else { Term }
    pub fn ift() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Prefixed {
                prefix: Box::new(Keyword::If.into()),
                inner: Box::new(Symbol::Delim {
                    delim_open: SpecialChar::BrackO,
                    inner: Box::new(Symbol::Term),
                    delim_close: SpecialChar::BrackC,
                }),
            }),
            separator: Box::new(Keyword::Else.into()),
            snd: Box::new(Symbol::Delim {
                delim_open: SpecialChar::BrackO,
                inner: Box::new(Symbol::Term),
                delim_close: SpecialChar::BrackC,
            }),
        }
    }

    /// Crate a symbol for a dereference term
    /// !Term
    pub fn dereft() -> Symbol {
        Symbol::Prefixed {
            prefix: Box::new(SpecialChar::Exclamation.into()),
            inner: Box::new(Symbol::Term),
        }
    }

    /// Create a symbol for a try term
    /// Try { Term }
    pub fn tryt() -> Symbol {
        Symbol::Prefixed {
            prefix: Box::new(Keyword::Try.into()),
            inner: Box::new(Symbol::Delim {
                delim_open: SpecialChar::BrackO,
                inner: Box::new(Symbol::Term),
                delim_close: SpecialChar::BrackC,
            }),
        }
    }

    /// Crate a symbol for a try-catch term
    /// Try { Term } Catch { Term }
    pub fn try_catch() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::tryt()),
            separator: Box::new(Keyword::Catch.into()),
            snd: Box::new(Symbol::Delim {
                delim_open: SpecialChar::BrackO,
                inner: Box::new(Symbol::Term),
                delim_close: SpecialChar::BrackC,
            }),
        }
    }

    // Crate a symbol for dot notation
    // Term.op (e.g. Term.fst())
    pub fn dot(op: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Term),
            separator: Box::new(SpecialChar::Dot.into()),
            snd: Box::new(op),
        }
    }

    /// Create a symbol for a constructor
    /// with given constructor name, optional type argument and constructor arguments
    /// ctor(arg1,arg2,...) without type argument
    /// ctor[ty_arg](arg1,arg2,...) with type argument
    pub fn ctor(ctor: Keyword, ty_arg: Option<Symbol>, args: Vec<Symbol>) -> Symbol {
        let mut inner = SpecialChar::Empty.into();

        for arg in args {
            if inner == SpecialChar::Empty.into() {
                inner = arg;
            } else {
                inner = Symbol::Separated {
                    fst: Box::new(inner),
                    separator: Box::new(SpecialChar::Comma.into()),
                    snd: Box::new(arg),
                };
            }
        }

        let mut prefix_inner = Box::new(Symbol::Delim {
            delim_open: SpecialChar::ParenO,
            inner: Box::new(inner),
            delim_close: SpecialChar::ParenC,
        });

        if let Some(ty) = ty_arg {
            prefix_inner = Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Delim {
                    delim_open: SpecialChar::SqBrackO,
                    inner: Box::new(ty),
                    delim_close: SpecialChar::SqBrackC,
                }),
                separator: Box::new(SpecialChar::Empty.into()),
                snd: prefix_inner,
            })
        }

        Symbol::Prefixed {
            prefix: Box::new(ctor.into()),
            inner: prefix_inner,
        }
    }

    /// Create a symbol for a case expression with givven patterns
    /// case Term of { pts }
    pub fn case(pts: Vec<Symbol>) -> Symbol {
        Symbol::Case {
            bound: Box::new(Symbol::Term),
            patterns: pts,
        }
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
        Symbol::Pattern {
            lhs: Box::new(ctor),
            rhs: Box::new(rhs),
        }
    }

    /// Create a symbol for a variant term/type
    /// with given inner symbol on the rhs of labels
    /// <label=inner,...>
    pub fn variant(inner: Symbol) -> Symbol {
        Symbol::Delim {
            delim_open: SpecialChar::AngBrackO,
            inner: Box::new(Symbol::Many(Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Label),
                separator: Box::new(SpecialChar::Equals.into()),
                snd: Box::new(inner),
            }))),
            delim_close: SpecialChar::AngBrackC,
        }
    }

    /// Create a symbol for a tuple term/type
    /// with a given inner symbol (usually term/type)
    /// (inner,...)
    pub fn tuple(inner: Symbol) -> Symbol {
        Symbol::Delim {
            delim_open: SpecialChar::ParenO,
            inner: Box::new(Symbol::Many(Box::new(inner))),
            delim_close: SpecialChar::ParenC,
        }
    }

    /// Crate a symbol for a pair term/type
    /// with given inner symbol (usually term/type)
    /// [inner,inner...]
    pub fn pair(inner: Symbol) -> Symbol {
        Symbol::Delim {
            delim_open: SpecialChar::BrackO,
            inner: Box::new(Symbol::Separated {
                fst: Box::new(inner.clone()),
                separator: Box::new(SpecialChar::Comma.into()),
                snd: Box::new(inner),
            }),
            delim_close: SpecialChar::BrackC,
        }
    }

    /// Create a symbol for a record term/type
    /// with given inner symbol (usually term/type)
    /// { Label = inner,...}
    pub fn record(inner: Symbol) -> Symbol {
        Symbol::Delim {
            delim_open: SpecialChar::BrackO,
            inner: Box::new(Symbol::Many(Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Label),
                separator: Box::new(SpecialChar::Equals.into()),
                snd: Box::new(inner),
            }))),
            delim_close: SpecialChar::BrackC,
        }
    }

    /// Create a label for a sum type
    /// Type + Type
    pub fn sum_ty() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Type),
            separator: Box::new(SpecialChar::Plus.into()),
            snd: Box::new(Symbol::Type),
        }
    }

    /// Crate a symbol for a product type
    /// Type x Type
    pub fn product_ty() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Type),
            separator: Box::new(SpecialChar::Times.into()),
            snd: Box::new(Symbol::Type),
        }
    }

    /// Create a symbol for a function type
    /// Type -> Type
    pub fn fun_ty() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Type),
            separator: Box::new(SpecialChar::Arrow.into()),
            snd: Box::new(Symbol::Type),
        }
    }

    /// Create a symbol for an application with given function and argument
    /// fun arg
    pub fn app(fun: Symbol, arg: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(fun),
            separator: Box::new(SpecialChar::Space.into()),
            snd: Box::new(arg),
        }
    }

    /// Create a symbol for an assignment Term
    /// Term := Term
    pub fn assign() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Term),
            separator: Box::new(SpecialChar::ColonEq.into()),
            snd: Box::new(Symbol::Term),
        }
    }

    /// Create a symbol for a type cast
    /// Term as Type
    pub fn cast() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Term),
            separator: Box::new(Keyword::As.into()),
            snd: Box::new(Symbol::Type),
        }
    }

    /// Create a symbol for a universal type
    /// with given symbol annotating the bound variable
    /// forall annot.Type
    pub fn forall_ty(annot: Symbol) -> Symbol {
        Symbol::Prefixed {
            prefix: Box::new(SpecialChar::Forall.into()),
            inner: Box::new(Symbol::Separated {
                fst: Box::new(annot),
                separator: Box::new(SpecialChar::Dot.into()),
                snd: Box::new(Symbol::Type),
            }),
        }
    }

    /// Create a symbol for an existential type
    /// with given symbol annotating the bound variable
    /// exists annot.Type
    pub fn exists_ty(annot: Symbol) -> Symbol {
        Symbol::Prefixed {
            prefix: Box::new(SpecialChar::Exists.into()),
            inner: Box::new(Symbol::Separated {
                fst: Box::new(annot),
                separator: Box::new(SpecialChar::Dot.into()),
                snd: Box::new(Symbol::Type),
            }),
        }
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
            Symbol::Prefixed { prefix, inner } => write!(f, "{prefix} {inner}"),
            Symbol::Delim {
                delim_open,
                inner,
                delim_close,
            } => write!(f, "{delim_open} {inner} {delim_close}"),
            Symbol::Separated {
                fst,
                separator,
                snd,
            } => write!(f, "{fst} {separator} {snd}"),
            Symbol::Case { bound, patterns } => write!(
                f,
                "case {bound} of {{ {} }}",
                patterns
                    .iter()
                    .map(|pt| pt.to_string())
                    .collect::<Vec<String>>()
                    .join(" | ")
            ),
            Symbol::Pattern { lhs, rhs } => write!(f, "{lhs} => {rhs}"),
        }
    }
}
