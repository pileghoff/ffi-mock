extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::Parse,
    parse_macro_input, parse_str,
    punctuated::Punctuated,
    token::{Comma, Default},
    Expr, FnArg, Ident, Signature, Token, Type, TypeParen,
};

fn get_names(in_sig: &Punctuated<FnArg, Comma>) -> Punctuated<Ident, Comma> {
    let mut res = Punctuated::new();

    in_sig
        .iter()
        .map(|f| match f {
            FnArg::Typed(t) => match *t.pat.clone() {
                syn::Pat::Ident(id) => id.ident,
                _ => unimplemented!(),
            },
            FnArg::Receiver(_) => unimplemented!(),
        })
        .for_each(|f| res.push(f));

    res
}

fn get_types(in_sig: &Punctuated<FnArg, Comma>) -> Punctuated<Type, Comma> {
    let mut res = Punctuated::new();

    in_sig
        .iter()
        .map(|f| match f {
            FnArg::Typed(t) => *t.ty.clone(),
            FnArg::Receiver(_) => unimplemented!(),
        })
        .for_each(|f| res.push(f));

    res
}

#[proc_macro]
pub fn mock(input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as Signature);
    let static_mock_name = format_ident!("STATIC_MOCK_{}", func.ident);
    let extern_name = func.ident;
    let in_types = get_types(&func.inputs);
    let in_names = get_names(&func.inputs);
    let in_sig = func.inputs;
    let out_sig: Type = match func.output {
        syn::ReturnType::Default => parse_str("()").unwrap(),
        syn::ReturnType::Type(_, t) => *t,
    };

    quote!(
        {
            lazy_static! {
                static ref #static_mock_name: Mutex<FunctionMockInner<#in_types, #out_sig>> =
                    Mutex::new(FunctionMockInner::new());
            }

            #[no_mangle]
            pub extern "C" fn #extern_name(#in_sig) -> #out_sig {
                let mut a = #static_mock_name.lock().unwrap();
                a.call_history.push(#in_names);
                a.get_next_return()
            }
            FunctionMock::new(&#static_mock_name)
        }
    )
    .into()
}
