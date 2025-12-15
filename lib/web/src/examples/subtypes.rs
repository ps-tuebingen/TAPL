//Automatically generated file, run `cargo run -p xtask` to regenerate
pub const COUNTERCLASS: &str = include_str!("../../../../examples/subtypes/counterclass/counterclass.sub");

pub const REFNAT: &str = include_str!("../../../../examples/subtypes/refnat/refnat.sub");

pub const NEWCOUNTER: &str = include_str!("../../../../examples/subtypes/newcounter/newcounter.sub");

pub const NEWINSTRCOUNTER: &str = include_str!("../../../../examples/subtypes/newinstrcounter/newinstrcounter.sub");

pub const VARIANT: &str = include_str!("../../../../examples/subtypes/variant/variant.sub");

pub const COUNTER: &str = include_str!("../../../../examples/subtypes/counter/counter.sub");

pub const SETCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/setcounterclass/setcounterclass.sub");

pub const INSTRCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/instrcounterclass/instrcounterclass.sub");

pub const NEWRESETCOUNTER: &str = include_str!("../../../../examples/subtypes/newresetcounter/newresetcounter.sub");

pub const BACKUPCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/backupcounterclass/backupcounterclass.sub");

pub const VARIANTCASE: &str = include_str!("../../../../examples/subtypes/variantcase/variantcase.sub");

pub const LIST: &str = include_str!("../../../../examples/subtypes/list/list.sub");

pub const NEWSETCOUNTER: &str = include_str!("../../../../examples/subtypes/newsetcounter/newsetcounter.sub");

pub const DECCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/deccounterclass/deccounterclass.sub");

pub const LAMBDAREC: &str = include_str!("../../../../examples/subtypes/lambdarec/lambdarec.sub");

pub const NEWDECCOUNTER: &str = include_str!("../../../../examples/subtypes/newdeccounter/newdeccounter.sub");

pub const RESETCOUNTERCLASS: &str = include_str!("../../../../examples/subtypes/resetcounterclass/resetcounterclass.sub");

pub const DOWNCAST: &str = include_str!("../../../../examples/subtypes/downcast/downcast.sub");

pub fn subtypes_all() -> Vec<(&'static str,&'static str)> {
    vec![
        ("Counterclass", COUNTERCLASS),
        ("Refnat", REFNAT),
        ("Newcounter", NEWCOUNTER),
        ("Newinstrcounter", NEWINSTRCOUNTER),
        ("Variant", VARIANT),
        ("Counter", COUNTER),
        ("Setcounterclass", SETCOUNTERCLASS),
        ("Instrcounterclass", INSTRCOUNTERCLASS),
        ("Newresetcounter", NEWRESETCOUNTER),
        ("Backupcounterclass", BACKUPCOUNTERCLASS),
        ("Variantcase", VARIANTCASE),
        ("List", LIST),
        ("Newsetcounter", NEWSETCOUNTER),
        ("Deccounterclass", DECCOUNTERCLASS),
        ("Lambdarec", LAMBDAREC),
        ("Newdeccounter", NEWDECCOUNTER),
        ("Resetcounterclass", RESETCOUNTERCLASS),
        ("Downcast", DOWNCAST),
    ]
}
