use syn::{Attribute, Data, Fields, FieldsUnnamed, Ident, Meta, Type, TypePath, Variant};

pub fn get_lang_attr(attrs: &[Attribute]) -> proc_macro2::TokenStream {
    let lang_meta = attrs
        .iter()
        .find(|attr| attr.path().is_ident("Lang"))
        .expect("Type `Lang` needs to be provided")
        .clone()
        .meta;
    match lang_meta {
        Meta::List(ls) => ls.tokens,
        _ => panic!("Wrong format for attribute `Lang`, use `#[Lang(Type)]`"),
    }
}

pub fn get_enum_variants(data: &Data) -> Vec<Variant> {
    let enum_data = match data {
        Data::Struct(_) => panic!("Cannot derive Typecheck for structs"),
        Data::Union(_) => panic!("Cannot derive Typecheck for unions"),
        Data::Enum(en) => en,
    };
    enum_data.variants.iter().cloned().collect()
}

pub fn get_variant_type_name(var: &Variant) -> Ident {
    let fields = match var.fields {
        Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => unnamed,
        _ => panic!("Enum variants that are not tuples are not supported"),
    };
    if fields.len() != 1 {
        panic!("Tuple variants with more than one field are not supported")
    }
    match &fields.first().unwrap().ty {
        Type::Path(TypePath { path, .. }) => path.segments.first().unwrap().ident.clone(),
        _ => panic!("Only type paths are supported in enum variants"),
    }
}
