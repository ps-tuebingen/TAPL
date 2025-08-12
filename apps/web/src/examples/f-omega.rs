pub const LISTADT: &str = include_str!("../../../../examples/f-omega/listadt/listadt.omega");

pub const UNPACKLIST: &str = include_str!("../../../../examples/f-omega/unpacklist/unpacklist.omega");

pub const UNPACKPAIR: &str = include_str!("../../../../examples/f-omega/unpackpair/unpackpair.omega");

pub const PAIRADT: &str = include_str!("../../../../examples/f-omega/pairadt/pairadt.omega");

pub fn f-omega_all() -> Vec<&'static str> { 
    vec![
        LISTADT,
        UNPACKLIST,
        UNPACKPAIR,
        PAIRADT,
    ]
}
