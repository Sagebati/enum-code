use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Attribute, Generics, GenericParam, parse_quote};


fn parse_attr_code(attr: &Attribute) -> Option<i32> {
    if attr.path().is_ident("code") {
        let code_in_attr = attr
            .parse_args::<syn::LitInt>()
            .expect("#[code()] value must be integer")
            .base10_parse::<i32>()
            .expect("#[code()] value is not a integer");
        Some(code_in_attr)
    } else {
        None
    }
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(enum_code::Code));
        }
    }
    generics
}

pub fn parse_code_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let enum_code = input.attrs.iter().find_map(parse_attr_code);

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, _ty_generics, _where_clause) = generics.split_for_impl();

    let getters: Vec<_> = match input.data {
        Data::Enum(e) => e
            .variants
            .iter()
            .map(|variant| {
                let attrs = variant.attrs.clone();
                let variant_ident = variant.ident.clone();
                let variant_fields = variant.fields.clone();

                let code_value = attrs.iter().find_map(parse_attr_code);
                let code_value_supplied = code_value.is_some();
                let code_value = enum_code.unwrap_or(0)  + code_value.unwrap_or(0);
                let enum_code = enum_code.unwrap_or(0);
                match variant_fields {
                    Fields::Named(..) => quote! {
                        #name::#variant_ident { .. } => #code_value
                    },
                    Fields::Unnamed(..) if code_value_supplied => quote! {
                        #name::#variant_ident ( .. ) => #code_value
                    },
                    Fields::Unnamed(..) if !code_value_supplied => quote! {
                        #name::#variant_ident ( x ) => #enum_code + x.get_code()
                    },
                    Fields::Unit => quote! {
                        #name::#variant_ident => #code_value
                    },
                    _ => unreachable!()
                }
            }).collect(),
        _ => panic!("Code attribute is only applicable to enums!"),
    };

    let output = quote! {
        impl #impl_generics enum_code::Code for #name {
            fn get_code(&self) -> i32 {
                match self {
                    #(#getters),*
                }
            }
        }
    };
    proc_macro::TokenStream::from(output)
}
