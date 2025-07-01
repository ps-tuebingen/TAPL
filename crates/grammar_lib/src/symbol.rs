#[derive(Clone)]
pub enum Symbol {
    Empty,
    Terminal(String),
    Term,
    Type,
    Value,
    Variable,
    If,
    App,
    Let,
    Pair,
    Record,
    Tuple,
    Variant,
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
    Arrow,
    Product,
    RecordTy,
    Sum,
    TupleTy,
    VariantTy,
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
}
