use syn::punctuated::Punctuated;
use syn::FnArg;
use syn::GenericArgument;
use syn::GenericParam;
use syn::Ident;
use syn::Path;
use syn::PathSegment;
use syn::TypeParam;
use syn::TypeParamBound;

/// Find all of the types in the function arguments and return them as a GenericArgument type
/// to be used as `T` in `Into<T>`
pub(crate) fn get_arg_generic_types<T>(
    args: Punctuated<FnArg, T>,
) -> impl Iterator<Item = GenericArgument> {
    let generc_args = args
        .into_iter()
        // we only generate generic arguments for types that are not `self`
        .filter(|arg_type| match arg_type {
            FnArg::Receiver(_) => false,
            FnArg::Typed(_) => true,
        })
        .map(|x| match x {
            FnArg::Typed(v) => v,
            _ => panic!(),
        })
        .map(|arg_type| {
            //
            GenericArgument::Type(*arg_type.ty)
        });

    generc_args
}

pub(crate) fn make_path_segments(
    generic_arg_iter: impl Iterator<Item = GenericArgument>,
) -> impl Iterator<Item = PathSegment> {
    let segments = generic_arg_iter.into_iter().map(|x| {
        let mut type_arg = Punctuated::new();
        type_arg.push(x);

        let path_segments = syn::PathSegment {
            ident: proc_macro2::Ident::new("Into", proc_macro2::Span::call_site()),
            arguments: syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: syn::token::Lt(proc_macro2::Span::call_site()),
                gt_token: syn::token::Gt(proc_macro2::Span::call_site()),
                args: type_arg,
            }),
        };
        path_segments
    });

    segments
}

pub(crate) fn make_paths(
    segments: impl Iterator<Item = PathSegment>,
) -> impl Iterator<Item = Path> {
    segments
        .into_iter()
        // first, we need to punctuate all of the types
        .map(|path_segment: PathSegment| {
            let mut punc = Punctuated::new();
            punc.push(path_segment);
            punc
        })
        // then map each of the punctuated path segments into a complete path
        .map(|punc_path_segment| Path {
            leading_colon: None,
            segments: punc_path_segment,
        })
}

type TraitBound = Punctuated<TypeParamBound, syn::token::Add>;

pub(crate) fn make_trait_bounds(
    paths: impl Iterator<Item = Path>,
) -> impl Iterator<Item = TraitBound> {
    paths
        .into_iter()
        .map(|path| {
            syn::TypeParamBound::Trait(syn::TraitBound {
                paren_token: None,
                modifier: syn::TraitBoundModifier::None,
                lifetimes: None,
                path,
            })
        })
        .map(|x| {
            let mut punc = Punctuated::new();
            punc.push(x);
            punc
        })
}

pub(crate) fn make_generic_parameter(
    trait_bounds: impl Iterator<Item = TraitBound>,
    types: impl Iterator<Item = Ident>,
) -> impl Iterator<Item = GenericParam> {
    types
        .zip(trait_bounds)
        .map(|(generic_type, trait_bound)| syn::TypeParam {
            attrs: vec![],
            ident: generic_type,
            colon_token: None,
            bounds: trait_bound,
            eq_token: None,
            default: None,
        })
        .map(|type_param: TypeParam| GenericParam::Type(type_param))
}

pub(crate) fn make_arguments_generic<T: Clone>(
    arguments: &mut Punctuated<FnArg, T>,
) -> impl Iterator<Item = Ident> {
    println!("getting generic arguments");

    let arg_clone: Punctuated<_, _> = (*arguments).clone();
    let new_idents = arg_clone
        .into_iter()
        .filter(|arg_type| match arg_type {
            FnArg::Receiver(_) => false,
            FnArg::Typed(_) => true,
        })
        .map(|arg_type| match arg_type {
            FnArg::Typed(v) => v,
            _ => panic!(),
        })
        .map(|pat: syn::PatType| match *pat.pat {
            syn::Pat::Ident(pat_ident) => pat_ident.ident,
            _ => panic!("Argument variable was not an ident"),
        })
        .map(|ident: Ident| {
            let mut ident_lower = ident.to_string();
            ident_lower.insert_str(0, "__");
            let new_ident = ident_lower.to_uppercase();

            Ident::new(&new_ident, ident.span())
        });

    arguments
        .iter_mut()
        .filter(|arg_type| match arg_type {
            FnArg::Receiver(_) => false,
            FnArg::Typed(_) => true,
        })
        .map(|arg_type| match arg_type {
            FnArg::Typed(v) => v,
            _ => panic!(),
        })
        .zip(new_idents.clone())
        .for_each(|(pat, new_ident): (_, Ident)| {
            let new_type = new_type(new_ident);

            pat.ty = Box::new(new_type);
        });

    new_idents
}

fn new_type(ident: Ident) -> syn::Type {
    let mut segments = Punctuated::new();
    segments.push(PathSegment {
        ident,
        arguments: syn::PathArguments::None,
    });
    let type_path = syn::TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments,
        },
    };
    syn::Type::Path(type_path)
}
