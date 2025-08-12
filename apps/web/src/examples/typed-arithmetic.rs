pub const IF_NESTED: &str = include_str!("../../../../examples/typed-arithmetic/if_nested/if_nested.arith");

pub const ISZZ: &str = include_str!("../../../../examples/typed-arithmetic/iszz/iszz.arith");

pub const IFTHENELSE: &str = include_str!("../../../../examples/typed-arithmetic/ifthenelse/ifthenelse.arith");

pub fn typed-arithmetic_all() -> Vec<&'static str> { 
    vec![
        IF_NESTED,
        ISZZ,
        IFTHENELSE,
    ]
}
