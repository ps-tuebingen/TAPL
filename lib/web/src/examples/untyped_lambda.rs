//Automatically generated file, run `cargo run -p xtask` to regenerate
pub const AND: &str = include_str!("../../../../examples/untyped_lambda/and/and.lam");

pub const ISZERO: &str = include_str!("../../../../examples/untyped_lambda/iszero/iszero.lam");

pub const ZERO: &str = include_str!("../../../../examples/untyped_lambda/zero/zero.lam");

pub const SND: &str = include_str!("../../../../examples/untyped_lambda/snd/snd.lam");

pub const PLUS: &str = include_str!("../../../../examples/untyped_lambda/plus/plus.lam");

pub const ONE: &str = include_str!("../../../../examples/untyped_lambda/one/one.lam");

pub const PRD: &str = include_str!("../../../../examples/untyped_lambda/prd/prd.lam");

pub const TIMES: &str = include_str!("../../../../examples/untyped_lambda/times/times.lam");

pub const FALSE: &str = include_str!("../../../../examples/untyped_lambda/false/false.lam");

pub const SUCC: &str = include_str!("../../../../examples/untyped_lambda/succ/succ.lam");

pub const IFTHENELSE: &str = include_str!("../../../../examples/untyped_lambda/ifthenelse/ifthenelse.lam");

pub const ID: &str = include_str!("../../../../examples/untyped_lambda/id/id.lam");

pub const TRU: &str = include_str!("../../../../examples/untyped_lambda/tru/tru.lam");

pub const FST: &str = include_str!("../../../../examples/untyped_lambda/fst/fst.lam");

pub const PAIR: &str = include_str!("../../../../examples/untyped_lambda/pair/pair.lam");

pub fn untyped_lambda_all() -> Vec<(&'static str,&'static str)> {
    vec![
        ("And", AND),
        ("Iszero", ISZERO),
        ("Zero", ZERO),
        ("Snd", SND),
        ("Plus", PLUS),
        ("One", ONE),
        ("Prd", PRD),
        ("Times", TIMES),
        ("False", FALSE),
        ("Succ", SUCC),
        ("Ifthenelse", IFTHENELSE),
        ("Id", ID),
        ("Tru", TRU),
        ("Fst", FST),
        ("Pair", PAIR),
    ]
}
