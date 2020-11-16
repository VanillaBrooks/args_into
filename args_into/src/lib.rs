mod body;
mod generics;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn args_into(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(item).expect("Could not parse syntax tree");

    let mut fn_item = if let syn::Item::Fn(x) = item {
        x
    } else {
        panic!("macro was not applied to a function")
    };

    let mut block = fn_item.block;
    let sig = fn_item.sig.clone();
    let mut inputs = fn_item.sig.inputs.clone();

    let generics = sig.generics;
    let mut types = generics.params;

    let type_arg = generics::get_arg_generic_types(fn_item.sig.inputs.clone());

    let path_segments = generics::make_path_segments(type_arg);

    let paths = generics::make_paths(path_segments);

    let bounds = generics::make_trait_bounds(paths);

    let new_generic_types = generics::make_arguments_generic(&mut inputs);

    let generic_parameters = generics::make_generic_parameter(bounds, new_generic_types);

    body::generate_body_insert_statements(inputs.clone())
        .for_each(|stmt| block.stmts.insert(0, stmt));

    generic_parameters.for_each(|x| types.push(x));

    // set the values back to what they previously were
    fn_item.sig.inputs = inputs;
    fn_item.block = block;
    fn_item.sig.generics.params = types;

    let x = quote! {
        #[allow(non_camel_case_types)]
        #fn_item
    };

    x.into()
}

//attrs: Vec<Attribute>
//vis: Visibility
//sig: Signature
//block: Box<Block>
