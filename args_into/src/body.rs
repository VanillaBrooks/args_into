use syn::punctuated::Punctuated;
use syn::FnArg;
use syn::Pat;
use syn::Stmt;

use quote::quote;

pub(crate) fn generate_body_insert_statements<T>(
    inputs: Punctuated<FnArg, T>,
) -> impl Iterator<Item = Stmt> {
    // first get the names of all the arguments
    inputs
        .into_iter()
        .filter(|arg_type| match arg_type {
            FnArg::Receiver(_) => false,
            FnArg::Typed(_) => true,
        })
        .map(|arg_type| match arg_type {
            FnArg::Typed(v) => v,
            _ => panic!(),
        })
        .map(|pat| {
            //
            match *pat.pat {
                Pat::Ident(id) => id.ident,
                _ => panic!(),
            }
        })
        .map(|ident| {
            let tokens = quote! {let #ident = #ident.into();};
            syn::parse(tokens.into()).unwrap()
        })
}
