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
            type Tuple = (#(#tuples,)*);
        }
    }.into()
}

#[proc_macro]
pub fn impl_uncons_for_tuple(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
    let args = parser.parse(input.clone()).unwrap();

    let mut tuples = Vec::new();
    let mut ty_indexes = Vec::new();
    let mut tails = Vec::new();

    for index in 1..=args.len() {
        let ty = format_ident!("T{}", index);
        tuples.push(ty);
    }

    for index in 2..=args.len() {
        let ty = format_ident!("T{}", index);
        let ty_index = format_ident!("t{}", index);
        ty_indexes.push(ty_index);
        tails.push(ty);
    }

    if args.len() == 1 {
        return quote!{
            impl <T1> crate::UnCons<T1> for (T1,) {
                type NextIt = T1;
                type Tail = ();

                #[allow(non_snake_case)]
                fn uncons(self) -> (Option<T1>, Self::Tail) {
                    let (t1,) = self;
                    (Some(t1), ())
                }
            }
        }.into();
    }

    let head_ty = tuples.first();
    let second_ty = tuples.get(1);

    quote!{
        impl <#(#tuples,)*> crate::UnCons<#head_ty> for (#(#tuples,)*) {
            type NextIt = #second_ty;
            type Tail = (#(#tails,)*);

            #[allow(non_snake_case)]
            fn uncons(self) -> (Option<#head_ty>, Self::Tail) {
                let (t1, #(#ty_indexes,) *) = self;
                (Some(t1), (#(#ty_indexes,) *))
            }
        }
    }.into()
}

#[proc_macro]
pub fn impl_into_hcons_for_tuple(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
    let args = parser.parse(input.clone()).unwrap();

    let mut tuples = Vec::new();
    let mut tup_indexes = Vec::new();

    // let last_ty = format_ident!("T{}", args.len());
    let mut hcons = quote!(crate::HEmpty<T>);
    let mut returns = quote!(crate::HEmpty { _phantom: core::marker::PhantomData::default() });

    for index in 1..=args.len() {
        let ty = format_ident!("t{}", index);
        tuples.push(quote!(T));
        tup_indexes.push(ty);
    }

    for index in (2..=args.len()).rev() {
        let ty = format_ident!("t{}", index);
        hcons = quote!(crate::HCons<T, #hcons>);
        returns = quote!(crate::HCons(#ty, #returns));
    }

    quote!{
        impl <T> core::convert::From<(#(#tuples,)*)> for crate::HCons<T, #hcons> {
            fn from(value: (#(#tuples,)*)) -> Self {
                let (#(#tup_indexes,)*) = value;
                crate::HCons(t1, #returns)
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

#[proc_macro]
pub fn impl_iterator_for_tuple(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
    let args = parser.parse(input.clone()).unwrap();

    let mut tuples = Vec::new();
    let mut slices = Vec::new();
    let tuple_len = Index::from(args.len()); 

    for _ in 1..=args.len() {
        tuples.push(quote!(T));
    }

    for index in 0..args.len() {
        let callee = format_ident!("t{}", index);
        slices.push(callee);
    }

    quote!{
        impl <T> crate::TupleIntoIterator<T> for (#(#tuples,)*) {
            type Output = TupleIter<T>;
        
            fn into_iter(self) -> Self::Output {
                TupleIter {
                    data: Rc::<[MaybeUninit<T>; #tuple_len]>::new(unsafe {
                        core::mem::transmute_copy(&self)
                    }),
                    cur: 0,
                }
            }
        }

        impl <T> crate::TupleIterator<T> for (#(#tuples,)*) {
            type Output = TupleIter<T>;
        
            fn iter(&self) -> Self::Output {
                TupleIter {
                    data: Rc::<[MaybeUninit<T>; #tuple_len]>::new(unsafe {
                        core::mem::transmute_copy(self)
                    }),
                    cur: 0,
                }
            }
        }
    }.into()
}
