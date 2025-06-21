#![doc = include_str!("../README.md")]

use evalexpr::{
    ContextWithMutableVariables, DefaultNumericTypes, HashMapContext, Value as EValue,
    eval_with_context,
};
use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::{collections::HashSet, mem};
use syn::{DeriveInput, parse_macro_input, parse_str, spanned::Spanned};

macro_rules! fail {
    ($span:expr, $($tts:tt)*) => {
        proc_macro_error::diagnostic!($span, proc_macro_error::Level::Error, $($tts)*).abort()
    };
    ($($tts:tt)*) => {
        proc_macro_error::abort!(proc_macro2::Span::call_site(), $($tts)*)
    };
}

macro_rules! parse {
    ($span:expr, $val:expr) => {
        match $val.parse::<TokenStream2>() {
            Ok(s) => s,
            Err(e) => fail!($span, format!("Could not parse '{}': \"{}\"", $val, e)),
        }
    };
    ($val:expr) => {
        parse!(proc_macro2::Span::call_site(), $val)
    };
}

#[derive(Debug, Clone)]
enum Value {
    String(String),
    Float(f64),
    Integer(i64),
    Boolean(bool),
    None,
}

impl Value {
    pub fn value_tokens(&self, tag: &Tag) -> TokenStream2 {
        match self {
            Value::String(v) => quote!(#v),
            Value::Float(v) => quote!(#v),
            Value::Integer(v) => quote!(#v),
            Value::Boolean(v) => quote!(#v),
            _ => fail!(
                tag.span,
                format!("Could not parse tag value '{}'", tag.name)
            ),
        }
    }

    pub fn return_type_string(&self, tag: &Tag) -> &'static str {
        match self {
            Value::String(_) => "&'static str",
            Value::Float(_) => "f64",
            Value::Integer(_) => "i64",
            Value::Boolean(_) => "bool",
            _ => fail!(tag.span, format!("Could not parse tag value")),
        }
    }

    pub fn return_type(&self, tag: &Tag) -> TokenStream2 {
        parse!(tag.span, self.return_type_string(tag))
    }
}

impl From<EValue> for Value {
    fn from(value: EValue) -> Self {
        match value {
            EValue::String(v) => Self::String(v),
            EValue::Float(v) => Self::Float(v),
            EValue::Int(v) => Self::Integer(v),
            EValue::Boolean(v) => Self::Boolean(v),
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone)]
struct Tag {
    name: String,
    variant: String,
    value: Value,
    expression: String,
    span: proc_macro2::Span,
}

fn eval(tag: &Tag, context: &mut HashMapContext<DefaultNumericTypes>) -> Value {
    match eval_with_context(&tag.expression, context) {
        Ok(value) => {
            // try to add the value to context
            context
                .set_value(tag.name.clone(), value.clone())
                .expect("Could not update context");

            // convert their value type to ours
            value.into()
        }
        Err(e) => fail!(tag.span, format!("Could not evaluate: {}", e)),
    }
}

fn get_tag(attribute: &syn::Attribute) -> Option<Tag> {
    if attribute.path().is_ident("tag") {
        let attr_span = attribute.span();

        if let syn::Meta::List(list) = &attribute.meta {
            // get all tokens as String
            let parts: Vec<(String, proc_macro2::Span)> = list
                .tokens
                .clone()
                .into_iter()
                .map(|t| (t.to_string(), t.span()))
                .filter(|(s, _)| !s.is_empty())
                .collect();

            // if there is no first token then fail
            if parts.len() < 1 {
                fail!(attr_span, format!("No name for tag"));
            }

            let name = &parts[0].0;
            let span = &parts[0].1;

            // if the first token is not a valid name then fail
            if parse_str::<syn::Ident>(name).is_err() {
                fail!(span, format!("Invalid tag name: '{}'", name));
            }

            // if the first token is followed by anything except ',' then fail.
            if parts.len() > 1 && parts[1].0 != "," {
                fail!(
                    span,
                    format!("Malformed tag: '{}'", list.tokens.to_string())
                );
            }

            // get the combined parameter expression
            let mut expression = parts
                .iter()
                .skip(2)
                .map(|(s, _)| s.clone())
                .collect::<String>()
                .trim()
                .to_string();

            // if the expression is empty, then default to 'true'
            if expression.is_empty() {
                expression = "true".into();
            }

            // build the tag struct
            return Some(Tag {
                name: name.clone(),
                variant: String::new(),
                value: Value::None,
                expression: expression,
                span: attr_span,
            });
        }
    }
    None
}

fn get_tags(variant: &syn::Variant) -> Vec<Tag> {
    let mut context = HashMapContext::<DefaultNumericTypes>::new();

    // get the name of the variant
    let name = variant.ident.to_string();

    // collect all tags from variant
    let mut tags = variant
        .attrs
        .iter()
        .filter_map(get_tag)
        .collect::<Vec<Tag>>();

    let mut history: HashSet<String> = HashSet::new();

    // check for duplicate tags
    for tag in tags.iter() {
        if history.contains(&tag.name) {
            fail!(tag.span, format!("Duplicate tag: '{}'", tag.name));
        } else {
            history.insert(tag.name.clone());
        }
    }

    // for each tag, evaluate and set value
    for tag in tags.iter_mut() {
        tag.variant = name.clone();
        tag.value = eval(&tag, &mut context);
    }

    tags
}

/// Derive macro to associate data with enum variants
/// 
/// This macro provides the `tag` attribute that associates data with enum variants,
/// and generates functions to access the tag values. Each generated function returns
/// an `Option<T>` where `T` is `f64`, `i64`, `&'static str`, or `bool`. The expressions
/// included in each tag are evaluated at compile time.
/// 
/// ## Examples
/// 
/// ### Code
/// 
/// ```rust
/// use enumrs::Tagged;
/// 
/// #[derive(Tagged)]
/// pub enum Country {
/// 
///     #[tag(id, 1)]
///     #[tag(index, id - 1)]
///     #[tag(name, "Afghanistan")]
///     #[tag(description, "Description of Afghanistan")]
/// 	AFG = 1,
/// 
///     #[tag(id, 2)]
///     #[tag(index, id - 1)]
///     #[tag(name, "Albania")]
///     #[tag(description, "Description of Albania")]
/// 	ALB = 2,
/// 
///     // ...
/// }
/// ```
/// 
/// ### Expanded
/// 
/// ```rust
/// pub enum Country {
/// 	AFG = 1,
/// 	ALB = 2,
///     // ...
/// }
/// 
/// impl Country {
///     pub fn id(&self) -> Option<i32> {
///         match self {
///             Self::AFG => Some(1),
///             Self::ALB => Some(2),
///             _ => None
///         }
///     }
///     pub fn index(&self) -> Option<i32> {
///         match self {
///             Self::AFG => Some(0),
///             Self::ALB => Some(1),
///             _ => None
///         }
///     }
///     pub fn name(&self) -> Option<&'static str> {
///         match self {
///             Self::AFG => Some("Afghanistan"),
///             Self::ALB => Some("Albania"),
///             _ => None
///         }
///     }
///     pub fn description(&self) -> Option<&'static str> {
///         match self {
///             Self::AFG => Some("Description of Afghanistan"),
///             Self::ALB => Some("Description of Albania"),
///             _ => None
///         }
///     }
/// }
/// ```
#[proc_macro_derive(Tagged, attributes(tag))]
#[proc_macro_error::proc_macro_error]
pub fn tagged_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ename = input.ident.to_string();
    let enum_name = ename.parse::<TokenStream2>().unwrap();

    // get enum data or fail with message
    let body = match input.data {
        syn::Data::Enum(s) => s,
        _ => fail!(format!("{} is not an enum", ename)),
    };

    // get the number of variants for comparison against
    // generated match arms to decide if we need a trailing
    // catch-all arm.
    let count = body.variants.iter().count();

    // collect tags for all variants
    let mut result = body
        .variants
        .iter()
        .flat_map(get_tags)
        .collect::<Vec<Tag>>();

    let mut functions: Vec<TokenStream2> = vec![];

    // sort the results so that `chunk_by` captures all of the
    // tags for each variant.
    result.sort_by(|a, b| a.name.cmp(&b.name));

    // build the tokenstreams for the generated functions
    for (name, chunk) in &result.into_iter().chunk_by(|t| t.name.clone()) {
        let tags = chunk.collect::<Vec<_>>();

        // get a copy of the first tag as a reference
        let tag = match tags.first().map(Clone::clone) {
            Some(v) => v,
            None => continue,
        };

        let first_discriminant = mem::discriminant(&tag.value);

        // check all tags have the same return type
        for item in tags.iter().skip(1) {
            if mem::discriminant(&item.value) != first_discriminant {
                fail!(
                    item.span,
                    format!(
                        "Mismatched expression value: expected '{}', found '{}'",
                        tag.value.return_type_string(&tag),
                        item.value.return_type_string(&tag)
                    )
                );
            }
        }

        // convert the tag name to a token stream
        let name = parse!(tag.span, name);

        // get the return type of the function to generate
        let kind = tag.value.return_type(&tag);

        // get the default match arm for the generated function
        let default = if tags.len() == count {
            parse!("")
        } else {
            parse!("_ => None")
        };

        // get all variant names
        let variants = tags
            .iter()
            .map(|t| t.variant.clone())
            .map(|s| s.parse::<TokenStream2>().unwrap())
            .collect::<Vec<_>>();

        // convert values to value token streams
        let values = tags
            .iter()
            .map(|t| t.value.value_tokens(&t))
            .collect::<Vec<_>>();

        // generate a function and add to functions
        functions.push(quote!(
            pub fn #name (&self) -> Option<#kind> {
                match self {
                    #( Self::#variants => Some(#values), )*
                    #default
                }
            }
        ));
    }

    quote!(
        impl #enum_name {
            #( #functions )*
        }
    )
    .into()
}
