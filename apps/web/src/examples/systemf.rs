pub const C2: &str = include_str!("../../../../examples/systemf/c2/c2.f");

pub const NOT: &str = include_str!("../../../../examples/systemf/not/not.f");

pub const CTIMES: &str = include_str!("../../../../examples/systemf/ctimes/ctimes.f");

pub const C0: &str = include_str!("../../../../examples/systemf/c0/c0.f");

pub const ID: &str = include_str!("../../../../examples/systemf/id/id.f");

pub const TRU: &str = include_str!("../../../../examples/systemf/tru/tru.f");

pub const NIL: &str = include_str!("../../../../examples/systemf/nil/nil.f");

pub const CONS: &str = include_str!("../../../../examples/systemf/cons/cons.f");

pub const CEXP: &str = include_str!("../../../../examples/systemf/cexp/cexp.f");

pub const DOUBLE: &str = include_str!("../../../../examples/systemf/double/double.f");

pub const ISNIL: &str = include_str!("../../../../examples/systemf/isnil/isnil.f");

pub const CSUCC: &str = include_str!("../../../../examples/systemf/csucc/csucc.f");

pub const QUADRUPLE: &str = include_str!("../../../../examples/systemf/quadruple/quadruple.f");

pub const SELFAPP: &str = include_str!("../../../../examples/systemf/selfapp/selfapp.f");

pub const CPLUS: &str = include_str!("../../../../examples/systemf/cplus/cplus.f");

pub const C1: &str = include_str!("../../../../examples/systemf/c1/c1.f");

pub const FLS: &str = include_str!("../../../../examples/systemf/fls/fls.f");

pub fn systemf_all() -> Vec<&'static str> { 
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
