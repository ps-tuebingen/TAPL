//Automatically generated file, run `cargo run -p xtask` to regenerate
pub const NEWCOUNTER: &str = include_str!("../../../../examples/f_omega_sub/newcounter/newcounter.omega");

pub const SENDGET: &str = include_str!("../../../../examples/f_omega_sub/sendget/sendget.omega");

pub const COUNTERCLASS: &str = include_str!("../../../../examples/f_omega_sub/counterclass/counterclass.omega");

pub fn f_omega_sub_all() -> Vec<(&'static str,&'static str)> {
    vec![
        ("Newcounter", NEWCOUNTER),
        ("Sendget", SENDGET),
        ("Counterclass", COUNTERCLASS),
    ]
}
