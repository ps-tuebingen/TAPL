use crate::utils::get_struct_fields;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type, parse_macro_input};

pub fn generate_eq_no_span(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let (impl_generics, type_generics, where_clause) = derive_input.generics.split_for_impl();
    let fields = get_struct_fields(&derive_input.data);
    let fields_eq = fields
        .iter()
        .filter(|(_, ty)| match ty {
            Type::Path(path) => {
                path.path
                    .segments
                    .last()
                    .expect("Could not get type name")
                    .ident
                    != "Span"
            }
            _ => true,
        })
        .map(|(field, ty)| {
            if let Type::Path(path) = ty {
                match path.path.segments.last() {
                    Some(seg) if seg.ident == "Box" || seg.ident == "Rc" => {
                        quote! { *self.#field == *other.#field }
                    }
                    _ => quote! { self.#field == other.#field },
                }
            } else {
                quote! { self.#field == other.#field}
            }
        });

    quote! {
        #[automatically_derived]
        impl #impl_generics PartialEq for #ident #type_generics #where_clause{
            fn eq(&self,other:&Self) -> bool{
                #(#fields_eq)&&*
            }
        }

        impl #impl_generics Eq for #ident #type_generics #where_clause {}

    }
    .into()
}
