use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{Ident, punctuated::Punctuated, Token, parse::Parser, Index};

#[proc_macro]
pub fn impl_compose_fn(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
    let args = parser.parse(input.clone()).unwrap();
    let mut ws = Vec::new();
    let mut gens = Vec::new();
    let mut tuples = Vec::new();
    let mut fn_call = Vec::new();

    for index in 1..=args.len() {
        let name = format_ident!("T{index}");
        let arg1 = format_ident!("P{index}");
        let arg2 = format_ident!("P{}", index + 1);
        let call_index = Index::from(index - 1);
        let call = quote!(self.#call_index);

        gens.push(quote!(#name, #arg1));
        ws.push(quote!(#name: Fn(#arg1) -> #arg2));
        tuples.push(quote!(#name));
        fn_call.push(call);
    }


    let fn_body = fn_call.into_iter().skip(1).fold(quote!((self.0)(init)), |acc, it| {
        quote!(#it(#acc))
    });

    let output_type = format_ident!("P{}", args.len() + 1);
    gens.push(quote!(#output_type));

    let result = quote! {
        impl <#(#gens,)*> crate::Compose<P1> for (#(#tuples,)*) where #(#ws,)* {
            type Output = #output_type;
            
            fn compose(self, init: P1) -> Self::Output {
                #fn_body
            }
        }
    };

    result.into()
}

#[proc_macro]
pub fn impl_vars_for_tuple(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
    let args = parser.parse(input.clone()).unwrap();

    let mut tuples = Vec::new();

    let mut hcons = quote!(crate::HEmpty<T>);

    for _ in 1..=args.len() {
        tuples.push(quote!(T));
    }

    for _ in 1..args.len() {
        hcons = quote!(crate::HCons<T, #hcons>);
    }

    quote!{
        impl <T> crate::Vars<T> for (#(#tuples,)*) {
            type Cons = #hcons;
        }
    }.into()
}

#[proc_macro]
pub fn impl_into_hcons_for_tuple(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
    let args = parser.parse(input.clone()).unwrap();

    let mut tuples = Vec::new();

    let mut hcons = quote!(crate::HEmpty<T>);
    let mut returns = quote!(crate::HEmpty::<T> { _phantom: core::marker::PhantomData::<T>::default() });

    for _ in (1..=args.len()).rev() {
        tuples.push(quote!(T));
        
    }

    for index in (1..args.len()).rev() {
        hcons = quote!(crate::HCons<T, #hcons>);
        let idx = syn::Index::from(index);
        returns = quote!(crate::HCons(self.#idx, #returns));
    }

    quote!{
        impl <T> crate::IntoHCons<T, #hcons> for (#(#tuples,)*) {
            fn into_hcons(self) -> crate::HCons<T,#hcons> {
                crate::HCons(self.0, #returns)
            }
        }
    }.into()
}

#[proc_macro]
pub fn impl_hnext_for_tuple(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
    let args = parser.parse(input.clone()).unwrap();

    let mut tuples = Vec::new();

    for _ in (1..=args.len()).rev() {
        tuples.push(quote!(T));
    }

    let Some((_, rest)) = tuples.split_first() else { panic!("Cannot split tuples"); };

    quote!{
        impl <T> crate::HNext<T> for (#(#tuples,)*) {
            type Next = (#(#rest,)*);

            fn value(&self) -> Option<&T> {
                Some(&self.0)
            }
        
            fn next(self) -> Self::Next {
                let (_, rest) = self.uncons();
                return rest;
            }
        }
    }.into()
}