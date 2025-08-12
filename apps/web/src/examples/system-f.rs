pub const C2: &str = include_str!("../../../../examples/system-f/c2/c2.f");

pub const NOT: &str = include_str!("../../../../examples/system-f/not/not.f");

pub const CTIMES: &str = include_str!("../../../../examples/system-f/ctimes/ctimes.f");

pub const C0: &str = include_str!("../../../../examples/system-f/c0/c0.f");

pub const ID: &str = include_str!("../../../../examples/system-f/id/id.f");

pub const TRU: &str = include_str!("../../../../examples/system-f/tru/tru.f");

pub const NIL: &str = include_str!("../../../../examples/system-f/nil/nil.f");

pub const CONS: &str = include_str!("../../../../examples/system-f/cons/cons.f");

pub const CEXP: &str = include_str!("../../../../examples/system-f/cexp/cexp.f");

pub const DOUBLE: &str = include_str!("../../../../examples/system-f/double/double.f");

pub const ISNIL: &str = include_str!("../../../../examples/system-f/isnil/isnil.f");

pub const CSUCC: &str = include_str!("../../../../examples/system-f/csucc/csucc.f");

pub const QUADRUPLE: &str = include_str!("../../../../examples/system-f/quadruple/quadruple.f");

pub const SELFAPP: &str = include_str!("../../../../examples/system-f/selfapp/selfapp.f");

pub const CPLUS: &str = include_str!("../../../../examples/system-f/cplus/cplus.f");

pub const C1: &str = include_str!("../../../../examples/system-f/c1/c1.f");

pub const FLS: &str = include_str!("../../../../examples/system-f/fls/fls.f");

pub fn system-f_all() -> Vec<&'static str> { 
    vec![
        C2,
        NOT,
        CTIMES,
        C0,
        ID,
        TRU,
        NIL,
        CONS,
        CEXP,
        DOUBLE,
        ISNIL,
        CSUCC,
        QUADRUPLE,
        SELFAPP,
        CPLUS,
        C1,
        FLS,
    ]
}
