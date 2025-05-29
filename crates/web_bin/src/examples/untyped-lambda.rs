pub const TIMES: &str = include_str!("../../../../examples/untyped-lambda/times/times.lam");

pub const FALSE: &str = include_str!("../../../../examples/untyped-lambda/false/false.lam");

pub const ONE: &str = include_str!("../../../../examples/untyped-lambda/one/one.lam");

pub const ID: &str = include_str!("../../../../examples/untyped-lambda/id/id.lam");

pub const SND: &str = include_str!("../../../../examples/untyped-lambda/snd/snd.lam");

pub const TRU: &str = include_str!("../../../../examples/untyped-lambda/tru/tru.lam");

pub const PAIR: &str = include_str!("../../../../examples/untyped-lambda/pair/pair.lam");

pub const FST: &str = include_str!("../../../../examples/untyped-lambda/fst/fst.lam");

pub const IFTHENELSE: &str = include_str!("../../../../examples/untyped-lambda/ifthenelse/ifthenelse.lam");

pub const ZERO: &str = include_str!("../../../../examples/untyped-lambda/zero/zero.lam");

pub const ISZERO: &str = include_str!("../../../../examples/untyped-lambda/iszero/iszero.lam");

pub const SUCC: &str = include_str!("../../../../examples/untyped-lambda/succ/succ.lam");

pub const AND: &str = include_str!("../../../../examples/untyped-lambda/and/and.lam");

pub const PLUS: &str = include_str!("../../../../examples/untyped-lambda/plus/plus.lam");

pub const PRD: &str = include_str!("../../../../examples/untyped-lambda/prd/prd.lam");

pub fn untyped-lambda_all() -> Vec<&'static str> { 
    vec![
        TIMES,
        FALSE,
        ONE,
        ID,
        SND,
        TRU,
        PAIR,
        FST,
        IFTHENELSE,
        ZERO,
        ISZERO,
        SUCC,
        AND,
        PLUS,
        PRD,
    ]
}
