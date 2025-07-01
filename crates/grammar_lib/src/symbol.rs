#[derive(Clone)]
pub enum Symbol {
    Empty,
    Many(Box<Symbol>),
    Separated {
        fst: Box<Symbol>,
        separator: String,
        snd: Box<Symbol>,
    },

    Terminal(String),
    Keyword(String),
    Term,
    Type,
    Value,

    Variable,
    Label,
    Location,

    Assignment {
        lhs: Box<Symbol>,
        rhs: Box<Symbol>,
    },
    Delim {
        delim_open: char,
        inner: Box<Symbol>,
        delim_close: char,
    },
    Lambda {
        annot: Box<Symbol>,
        body: Box<Symbol>,
    },
    Call {
        name: String,
        annot: Box<Symbol>,
        args: Vec<Symbol>,
    },
    Dot {
        body: Box<Symbol>,
        operator: String,
    },
    Case {
        bound: Box<Symbol>,
        patterns: Vec<Symbol>,
    },
    Pattern {
        lhs: String,
        num_vars: usize,
    },
    If,
    Let,
    Try,
    TryCatch,
    Pack,
    Unpack,
    Exists,
}

impl Symbol {
    pub fn term(t: &str) -> Symbol {
        Symbol::Terminal(t.to_owned())
    }

    pub fn call_annot(nm: &str, num_args: usize, arg_sym: Symbol) -> Symbol {
        Symbol::Call {
            name: nm.to_owned(),
            annot: Box::new(Symbol::Type),
            args: (0..num_args).map(|_| arg_sym.clone()).collect(),
        }
    }

    pub fn call(nm: &str, num_args: usize, arg_sym: Symbol) -> Symbol {
        Symbol::Call {
            name: nm.to_owned(),
            annot: Box::new(Symbol::Empty),
            args: (0..num_args).map(|_| arg_sym.clone()).collect(),
        }
    }

    pub fn lam(annot: Symbol, body: Symbol) -> Symbol {
        Symbol::Lambda {
            annot: Box::new(annot),
            body: Box::new(body),
        }
    }

    pub fn dot(op: &str) -> Symbol {
        Symbol::Dot {
            body: Box::new(Symbol::Term),
            operator: op.to_owned(),
        }
    }

    pub fn case(pts: Vec<Symbol>) -> Symbol {
        Symbol::Case {
            bound: Box::new(Symbol::Term),
            patterns: pts,
        }
    }

    pub fn pt(ctor: &str, num_vars: usize) -> Symbol {
        Symbol::Pattern {
            lhs: ctor.to_owned(),
            num_vars,
        }
    }

    pub fn variant(inner: Symbol) -> Symbol {
        Symbol::Delim {
            delim_open: '<',
            inner: Box::new(Symbol::Many(Box::new(Symbol::Assignment {
                lhs: Box::new(Symbol::Label),
                rhs: Box::new(inner),
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
            inner: Box::new(Symbol::Many(Box::new(Symbol::Assignment {
                lhs: Box::new(Symbol::Label),
                rhs: Box::new(inner),
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
        Symbol::Assignment {
            lhs: Box::new(Symbol::Term),
            rhs: Box::new(Symbol::Term),
        }
    }
}
