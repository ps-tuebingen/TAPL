pub enum Symbol {
    Terminal(String),
    Term,
    Type,
    If,
    Call {
        name: String,
        annot: bool,
        num_args: usize,
    },
}

impl Symbol {
    pub fn term(t: &str) -> Symbol {
        Symbol::Terminal(t.to_owned())
    }

    pub fn call_annot(nm: &str, num_args: usize) -> Symbol {
        Symbol::Call {
            name: nm.to_owned(),
            annot: true,
            num_args,
        }
    }

    pub fn call(nm: &str, num_args: usize) -> Symbol {
        Symbol::Call {
            name: nm.to_owned(),
            annot: false,
            num_args,
        }
    }
}
