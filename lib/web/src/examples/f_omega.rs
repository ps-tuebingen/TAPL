//Automatically generated file, run `cargo run -p xtask` to regenerate
pub const PAIRADT: &str = include_str!("../../../../examples/f_omega/pairadt/pairadt.omega");

pub const UNPACKLIST: &str = include_str!("../../../../examples/f_omega/unpacklist/unpacklist.omega");

pub const LISTADT: &str = include_str!("../../../../examples/f_omega/listadt/listadt.omega");

pub const UNPACKPAIR: &str = include_str!("../../../../examples/f_omega/unpackpair/unpackpair.omega");

pub fn f_omega_all() -> Vec<(&'static str,&'static str)> {
    vec![
        ("Pairadt", PAIRADT),
        ("Unpacklist", UNPACKLIST),
        ("Listadt", LISTADT),
        ("Unpackpair", UNPACKPAIR),
    ]
}
