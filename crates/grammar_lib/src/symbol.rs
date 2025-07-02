#[derive(Clone)]
pub enum Symbol {
    Empty,
    Many(Box<Symbol>),

    Terminal(String),
    Keyword(String),
    Term,
    Type,
    Kind,
    Value,

    Variable,
    Typevariable,
    Label,
    Location,
    Number,

    Prefixed {
        prefix: String,
        inner: Box<Symbol>,
    },
    Delim {
        delim_open: char,
        inner: Box<Symbol>,
        delim_close: char,
    },
    Separated {
        fst: Box<Symbol>,
        separator: String,
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
    pub fn term(t: &str) -> Symbol {
        Symbol::Terminal(t.to_owned())
    }

    pub fn lam_untyped(inner: Symbol) -> Symbol {
        Symbol::Prefixed {
            prefix: "\\".to_owned(),
            inner: Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Variable),
                separator: ".".to_owned(),
                snd: Box::new(inner),
            }),
        }
    }

    pub fn ty_annot(sym: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(sym),
            separator: ":".to_owned(),
            snd: Box::new(Symbol::Type),
        }
    }

    pub fn kind_annot(sym: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(sym),
            separator: "::".to_owned(),
            snd: Box::new(Symbol::Kind),
        }
    }

    pub fn subty_annot(sym: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(sym),
            separator: "<:".to_owned(),
            snd: Box::new(Symbol::Type),
        }
    }

    pub fn lam(annot: Symbol, body: Symbol) -> Symbol {
        Symbol::Prefixed {
            prefix: "\\".to_owned(),
            inner: Box::new(Symbol::Separated {
                fst: Box::new(annot),
                separator: ".".to_owned(),
                snd: Box::new(body),
            }),
        }
    }

    pub fn mu() -> Symbol {
        Symbol::Prefixed {
            prefix: "mu".to_owned(),
            inner: Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Variable),
                separator: ".".to_owned(),
                snd: Box::new(Symbol::Type),
            }),
        }
    }

    pub fn pack(inner: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Delim {
                delim_open: '{',
                inner: Box::new(Symbol::Separated {
                    fst: Box::new(inner),
                    separator: ",".to_owned(),
                    snd: Box::new(Symbol::Type),
                }),
                delim_close: '}',
            }),
            separator: "as".to_owned(),
            snd: Box::new(Symbol::Type),
        }
    }

    pub fn unpack() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Prefixed {
                prefix: "let".to_owned(),
                inner: Box::new(Symbol::Separated {
                    fst: Box::new(Symbol::Delim {
                        delim_open: '{',
                        inner: Box::new(Symbol::Separated {
                            fst: Box::new(Symbol::Typevariable),
                            separator: ",".to_owned(),
                            snd: Box::new(Symbol::Variable),
                        }),
                        delim_close: '}',
                    }),
                    separator: "=".to_owned(),
                    snd: Box::new(Symbol::Term),
                }),
            }),
            separator: "in".to_owned(),
            snd: Box::new(Symbol::Term),
        }
    }

    pub fn lett() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Prefixed {
                prefix: "let".to_owned(),
                inner: Box::new(Symbol::Delim {
                    delim_open: '(',
                    inner: Box::new(Symbol::Separated {
                        fst: Box::new(Symbol::Variable),
                        separator: "=".to_owned(),
                        snd: Box::new(Symbol::Term),
                    }),
                    delim_close: ')',
                }),
            }),
            separator: "in".to_owned(),
            snd: Box::new(Symbol::Term),
        }
    }

    pub fn ift() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Prefixed {
                prefix: "if".to_owned(),
                inner: Box::new(Symbol::Delim {
                    delim_open: '{',
                    inner: Box::new(Symbol::Term),
                    delim_close: '}',
                }),
            }),
            separator: "else".to_owned(),
            snd: Box::new(Symbol::Delim {
                delim_open: '{',
                inner: Box::new(Symbol::Term),
                delim_close: '}',
            }),
        }
    }

    pub fn dereft() -> Symbol {
        Symbol::Prefixed {
            prefix: "!".to_owned(),
            inner: Box::new(Symbol::Term),
        }
    }

    pub fn tryt() -> Symbol {
        Symbol::Prefixed {
            prefix: "try".to_owned(),
            inner: Box::new(Symbol::Delim {
                delim_open: '{',
                inner: Box::new(Symbol::Term),
                delim_close: '}',
            }),
        }
    }

    pub fn try_catch() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::tryt()),
            separator: "catch".to_owned(),
            snd: Box::new(Symbol::Delim {
                delim_open: '{',
                inner: Box::new(Symbol::Term),
                delim_close: '}',
            }),
        }
    }

    pub fn dot(op: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Term),
            separator: ".".to_owned(),
            snd: Box::new(op),
        }
    }

    pub fn ctor(ctor: &str, ty_arg: Option<Symbol>, args: Vec<Symbol>) -> Symbol {
        let mut inner = Symbol::Empty;

        for arg in args {
            inner = Symbol::Separated {
                fst: Box::new(inner),
                separator: ",".to_owned(),
                snd: Box::new(arg),
            };
        }

        let mut prefix_inner = Box::new(Symbol::Delim {
            delim_open: '(',
            inner: Box::new(inner),
            delim_close: ')',
        });

        if let Some(ty) = ty_arg {
            prefix_inner = Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Delim {
                    delim_open: '[',
                    inner: Box::new(ty),
                    delim_close: ']',
                }),
                separator: "".to_owned(),
                snd: prefix_inner,
            })
        }

        Symbol::Prefixed {
            prefix: ctor.to_owned(),
            inner: prefix_inner,
        }
    }

    pub fn case(pts: Vec<Symbol>) -> Symbol {
        Symbol::Case {
            bound: Box::new(Symbol::Term),
            patterns: pts,
        }
    }

    pub fn ctor_pt(ctor: &str, num_args: usize) -> Symbol {
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
            delim_open: '<',
            inner: Box::new(Symbol::Many(Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Label),
                separator: ",".to_owned(),
                snd: Box::new(inner),
            }))),
            delim_close: '>',
        }
    }

    pub fn tuple(inner: Symbol) -> Symbol {
        Symbol::Delim {
            delim_open: '(',
            inner: Box::new(Symbol::Many(Box::new(inner))),
            delim_close: ')',
        }
    }

    pub fn pair(inner: Symbol) -> Symbol {
        Symbol::Delim {
            delim_open: '{',
            inner: Box::new(Symbol::Separated {
                fst: Box::new(inner.clone()),
                separator: ",".to_owned(),
                snd: Box::new(inner),
            }),
            delim_close: '}',
        }
    }

    pub fn record(inner: Symbol) -> Symbol {
        Symbol::Delim {
            delim_open: '{',
            inner: Box::new(Symbol::Many(Box::new(Symbol::Separated {
                fst: Box::new(Symbol::Label),
                separator: ",".to_owned(),
                snd: Box::new(inner),
            }))),
            delim_close: '}',
        }
    }

    pub fn sum_ty() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Type),
            separator: "+".to_owned(),
            snd: Box::new(Symbol::Type),
        }
    }

    pub fn product_ty() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Type),
            separator: "x".to_owned(),
            snd: Box::new(Symbol::Type),
        }
    }

    pub fn fun_ty() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Type),
            separator: "->".to_owned(),
            snd: Box::new(Symbol::Type),
        }
    }

    pub fn app(fun: Symbol, arg: Symbol) -> Symbol {
        Symbol::Separated {
            fst: Box::new(fun),
            separator: " ".to_owned(),
            snd: Box::new(arg),
        }
    }

    pub fn assign() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Term),
            separator: ":=".to_owned(),
            snd: Box::new(Symbol::Term),
        }
    }

    pub fn cast() -> Symbol {
        Symbol::Separated {
            fst: Box::new(Symbol::Term),
            separator: "as".to_owned(),
            snd: Box::new(Symbol::Type),
        }
    }

    pub fn forall_ty(annot: Symbol) -> Symbol {
        Symbol::Prefixed {
            prefix: "forall".to_owned(),
            inner: Box::new(Symbol::Separated {
                fst: Box::new(annot),
                separator: ".".to_owned(),
                snd: Box::new(Symbol::Type),
            }),
        }
    }

    pub fn exists_ty(annot: Symbol) -> Symbol {
        Symbol::Prefixed {
            prefix: "exists".to_owned(),
            inner: Box::new(Symbol::Separated {
                fst: Box::new(annot),
                separator: ".".to_owned(),
                snd: Box::new(Symbol::Type),
            }),
        }
    }
}
