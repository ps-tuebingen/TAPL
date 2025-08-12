pub const SENDGET: &str = include_str!("../../../../examples/f-omega-sub/sendget/sendget.omega");

pub const COUNTERCLASS: &str = include_str!("../../../../examples/f-omega-sub/counterclass/counterclass.omega");

pub const SENDRESET: &str = include_str!("../../../../examples/f-omega-sub/sendreset/sendreset.omega");

pub const SENDINC: &str = include_str!("../../../../examples/f-omega-sub/sendinc/sendinc.omega");

pub const NEWCOUNTER: &str = include_str!("../../../../examples/f-omega-sub/newcounter/newcounter.omega");

pub const RESETCOUNTERCLASS: &str = include_str!("../../../../examples/f-omega-sub/resetcounterclass/resetcounterclass.omega");

pub fn f-omega-sub_all() -> Vec<&'static str> { 
    vec![
        SENDGET,
        COUNTERCLASS,
        SENDRESET,
        SENDINC,
        NEWCOUNTER,
        RESETCOUNTERCLASS,
    ]
}
