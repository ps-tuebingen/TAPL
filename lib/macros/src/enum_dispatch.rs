use proc_macro::TokenStream;
use syn::{Ident, Token};

struct EnumDispatchArgs {
    enum_ty: Ident,
    sep: Token![,],
    fun: Ident,
}

pub fn generate_enum_dispatch(input: TokenStream) -> TokenStream {
    todo!()
}
