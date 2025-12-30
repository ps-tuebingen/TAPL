use crate::utils::{get_enum_variants, get_lang_attr, map_variants};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn generate_generate_constraints(
    input: TokenStream,
    target: proc_macro2::TokenStream,
) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let lang_val = get_lang_attr(&derive_input.attrs);
    let variants = get_enum_variants(&derive_input.data);
    let gen_variants = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.generate_constraints(state), }
    });
    let output = quote! {
        #[automatically_derived]
        impl inference::GenerateConstraints for #ident {
            type Target = #target;
            type Lang = #lang_val;
            fn generate_constraints(&self,state:&mut inference::GenState<#lang_val>) -> Self::Target{
                match self {
                #(#gen_variants)*
                }
            }

        }
    };
    output.into()
}

pub fn generate_generate_constraints_term(input: TokenStream) -> TokenStream {
    generate_generate_constraints(
        input,
        quote! {<Self::Lang as syntax::language::Language>::Type},
    )
}

pub fn generate_generate_constraints_type(input: TokenStream) -> TokenStream {
    generate_generate_constraints(input, quote! {inference::constraints::KindOrVar})
}

pub fn generate_solve_constraint(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    let ident = derive_input.ident;
    let lang_val = get_lang_attr(&derive_input.attrs);
    let variants = get_enum_variants(&derive_input.data);
    let eq_variants = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.solve_equality(rhs,state), }
    });
    let sub_variants = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.solve_subtyping(sup,state), }
    });

    let ind_variants = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.solve_index(ind,ind_ty,state), }
    });

    let rec_variants = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.solve_record(label,label_ty,state), }
    });

    let variant_variants = map_variants(&variants, |var| {
        let ident = &var.ident;
        quote! {Self::#ident(inner) => inner.solve_variant(label,label_ty,state), }
    });

    let output = quote! {
        #[automatically_derived]
        impl inference::SolveConstraint for #ident {
            type Lang = #lang_val;
            fn solve_equality(
                self,
                rhs:<Self::Lang as syntax::language::Language>::Type,
                state:&mut inference::SolveState<Self::Lang>
            ) -> Result<(),errors::inference_error::InferenceError> {
                match self{
                    #(#eq_variants)*
                }
            }

            fn solve_subtyping(
                self,
                sup:<Self::Lang as syntax::language::Language>::Type,
                state:&mut inference::SolveState<Self::Lang>
            )->Result<(),errors::inference_error::InferenceError>{
                match self{
                    #(#sub_variants)*
                }
            }

            fn solve_index(
                self,
                ind:usize,
                ind_ty:<Self::Lang as syntax::language::Language>::Type,
                state:&mut inference::SolveState<Self::Lang>
            )-> Result<(),errors::inference_error::InferenceError> {
                match self{
                    #(#ind_variants)*
                }
            }

            fn solve_record(
                self,
                label:syntax::Label,
                label_ty:<Self::Lang as syntax::language::Language>::Type,
                state:&mut inference::SolveState<Self::Lang>
            ) -> Result<(),errors::inference_error::InferenceError> {
                match self{
                    #(#rec_variants)*
                }
            }

            fn solve_variant(
                self,
                label:syntax::Label,
                label_ty:<Self::Lang as syntax::language::Language>::Type,
                state:&mut inference::SolveState<Self::Lang>
                ) -> Result<(),errors::inference_error::InferenceError> {
                match self{
                    #(#variant_variants)*
                }
            }

        }
    };
    output.into()
}
