mod keywords;
mod special_char;

pub use keywords::Keyword;
pub use special_char::SpecialChar;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Symbol {
    Many(Box<Symbol>),

    Keyword(Keyword),
    SpecialChar(SpecialChar),
    Term,
    Type,
    Kind,
    Value,

    Variable,
    Typevariable,
    Label,
    Location,

    Prefixed {
        prefix: Box<Symbol>,
        inner: Box<Symbol>,
    },
    Delim {
        delim_open: SpecialChar,
        inner: Box<Symbol>,
        delim_close: SpecialChar,
    },
    Separated {
        fst: Box<Symbol>,
        separator: Box<Symbol>,
        snd: Box<Symbol>,
    },
    Case {
        bound: Box<Symbol>,
        patterns: Vec<Symbol>,
    },
    Pattern {
        lhs: Box<Symbol>,
        rhs: Box<Symbol>,
    },
}

impl Symbol {
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

    pub fn ty_annot(sym: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(sym),
            separator: Box::new(SpecialChar::Colon.into()),
            snd: Box::new(Symbol::Type),
        }
    }

    pub fn kind_annot(sym: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(sym),
            separator: Box::new(SpecialChar::DoubleColon.into()),
            snd: Box::new(Symbol::Kind),
        }
    }

    pub fn subty_annot(sym: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(sym),
            separator: Box::new(SpecialChar::LessColon.into()),
            snd: Box::new(Symbol::Type),
        }
    }

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

    pub fn dereft() -> Symbol {
        Symbol::Prefixed {
            prefix: Box::new(SpecialChar::Exclamation.into()),
            inner: Box::new(Symbol::Term),
        }
    }

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

    pub fn dot(op: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Term),
            separator: Box::new(SpecialChar::Dot.into()),
            snd: Box::new(op),
        }
    }

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

    pub fn case(pts: Vec<Symbol>) -> Symbol {
        Symbol::Case {
            bound: Box::new(Symbol::Term),
            patterns: pts,
        }
    }

    pub fn ctor_pt(ctor: Keyword, num_args: usize) -> Symbol {
        Symbol::ctor(
            ctor,
            None,
            (0..num_args).map(|_| Symbol::Variable).collect(),
        )
    }

    pub fn pt(ctor: Symbol, rhs: Symbol) -> Symbol {
        Symbol::Pattern {
            lhs: Box::new(ctor),
            rhs: Box::new(rhs),
        }
    }

    pub fn variant(inner: Symbol) -> Symbol {
        Symbol::Delim {
            delim_open: SpecialChar::AngBrackO,
            inner: Box::new(Symbol::Many(Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Label),
                separator: Box::new(SpecialChar::Comma.into()),
                snd: Box::new(inner),
            }))),
            delim_close: SpecialChar::AngBrackC,
        }
    }

    pub fn tuple(inner: Symbol) -> Symbol {
        Symbol::Delim {
            delim_open: SpecialChar::ParenO,
            inner: Box::new(Symbol::Many(Box::new(inner))),
            delim_close: SpecialChar::ParenC,
        }
    }

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

    pub fn record(inner: Symbol) -> Symbol {
        Symbol::Delim {
            delim_open: SpecialChar::BrackO,
            inner: Box::new(Symbol::Many(Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Label),
                separator: Box::new(SpecialChar::Comma.into()),
                snd: Box::new(inner),
            }))),
            delim_close: SpecialChar::BrackC,
        }
    }

    pub fn sum_ty() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Type),
            separator: Box::new(SpecialChar::Plus.into()),
            snd: Box::new(Symbol::Type),
        }
    }

    pub fn product_ty() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Type),
            separator: Box::new(SpecialChar::Times.into()),
            snd: Box::new(Symbol::Type),
        }
    }

    pub fn fun_ty() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Type),
            separator: Box::new(SpecialChar::Arrow.into()),
            snd: Box::new(Symbol::Type),
        }
    }

    pub fn app(fun: Symbol, arg: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(fun),
            separator: Box::new(SpecialChar::Space.into()),
            snd: Box::new(arg),
        }
    }

    pub fn assign() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Term),
            separator: Box::new(SpecialChar::ColonEq.into()),
            snd: Box::new(Symbol::Term),
        }
    }

    pub fn cast() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Term),
            separator: Box::new(Keyword::As.into()),
            snd: Box::new(Symbol::Type),
        }
    }

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
