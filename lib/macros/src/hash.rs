use crate::utils::get_struct_fields;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type, parse_macro_input};

pub fn generate_hash_no_span(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let (impl_generics, type_generics, where_clause) = derive_input.generics.split_for_impl();
    let fields = get_struct_fields(&derive_input.data);
    let fields_hash = fields
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
                    Some(seg) if seg.ident == "HashMap" => {
                        quote! { for|(k,v) in  self.#field.iter(){
                            k.hash(state);
                            v.hash(state);
                        }; }
                    }

                    _ => quote! { self.#field.hash(state);  },
                }
            } else {
                quote! { self.#field.hash(state);  }
            }
        });

    quote! {
        #[automatically_derived]
        impl #impl_generics ::std::hash::Hash for #ident #type_generics #where_clause{
            fn hash<H: ::std::hash::Hasher>(&self,state:&mut H) {
                #(#fields_hash)*
            }
        }

    }
    .into()
}
