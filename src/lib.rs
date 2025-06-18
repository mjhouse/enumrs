use std::collections::HashMap;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use proc_macro_error::abort_call_site;
use quote::quote;
use evalexpr::*;

mod tag;
mod tags;

use tags::Tags;

#[proc_macro_derive(Tagged, attributes(tag))]
#[proc_macro_error::proc_macro_error]
pub fn tagged_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.to_string();

    let body = match input.data {
        syn::Data::Enum(s) => s,
        _ => abort_call_site!(
            format!("{} is not an enum",name)
        )
    };

    let result = body
        .variants
        .iter()
        .map(Tags::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_iter()
        .flat_map(|c| c.0)
        .fold(HashMap::new(), |mut a,t| {
            a.entry(t
                .name
                .clone())
            .or_insert(Vec::new())
            .push(t); 
            a 
        });

    for (_,tags) in result.iter() {
        if let Some(tag) = tags.iter().next() {
            for other in tags.iter() {
                if !other.is_type(tag) {
                    panic!(
                        "{}::{} [{}] wrong value type",
                        name,other.variant,other.name
                    );
                }
            }
        }
    }

    let mut functions: Vec<proc_macro2::TokenStream> = vec![];

    for (func,tags) in result.into_iter() {
        let f: proc_macro2::TokenStream = func.parse().unwrap();
        let e: proc_macro2::TokenStream = name.parse().unwrap();
        let mut v: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
        let mut r: proc_macro2::TokenStream = proc_macro2::TokenStream::new();

        let statements = tags
            .into_iter()
            .map(|t| {
                v = t.variant.clone().trim().parse().unwrap();
                match t.value.unwrap() {
                    Value::String(expr) => {
                        r = "&'static str".parse().unwrap();
                        quote!( #e::#v => Some(#expr) )
                    },
                    Value::Float(expr) => {
                        r = "f64".parse().unwrap();
                        quote!( #e::#v => Some(#expr) )
                    },
                    Value::Int(expr) => {
                        r = "i64".parse().unwrap();
                        quote!( #e::#v => Some(#expr) )
                    },
                    Value::Boolean(expr) => {
                        r = "bool".parse().unwrap();
                        quote!( #e::#v => Some(#expr) )
                    },
                    _ => panic!("Unsupported expression: {}",t.expression)
                }
            })
            .collect::<Vec<_>>();

        let mut default: proc_macro2::TokenStream = "".parse().unwrap();

        if statements.len() < body.variants.len() {
            default = "_ => None".parse().unwrap();
        }

        functions.push(quote!(
            pub fn #f (self) -> Option<#r> {
                match self {
                    #( #statements, )*
                    #default
                }
            }
        ));
    }

    let e: proc_macro2::TokenStream = name.parse().unwrap();

    quote!(
        impl #e {
            #( #functions )*
        }
    ).into()
}