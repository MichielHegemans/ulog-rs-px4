#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::{TokenStream};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, GenericParam, Generics, Ident};

#[proc_macro_derive(Data)]
pub fn derive_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let (message_name_code, message_name) = build_message_name(&name);
    let format_str = build_format_str(&input.data, &name);

    let expanded = quote! {
        #message_name_code

        impl #impl_generics ulog::Data for #name #ty_generics #where_clause {
            fn message_name() -> &'static str {
                #message_name
            }

            fn message_format() -> Result<ulog::message::Format, ulog::message::Error> {
                #format_str
            }
        }
    };

    TokenStream::from(expanded)
}

fn build_format_str(data: &Data, name: &Ident) -> proc_macro2::TokenStream {
    let name_str = name.to_string().to_ascii_lowercase();

    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let field_entries = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;

                        let name_str = quote! { #name }.to_string().to_ascii_lowercase();

                        quote_spanned! { f.span() =>
                            format!("{} {}", <#ty as ulog::DataType>::type_string(), #name_str)
                        }
                    });

                    quote! {
                        let mut s = String::new();
                        s.push_str(#name_str);
                        s.push(':');

                        let params = vec![
                            #(#field_entries ,)*
                        ];

                        s.push_str(&params.join(";"));

                        Format::new(s)
                    }
                },
                // Not supported as we need names
                Fields::Unnamed(_) => unimplemented!(),
                // Empty message is pointless.
                Fields::Unit => unimplemented!(),
            }
        },
        // I guess technically could be supported as bool for unit enum, and fields when the enum
        // is a single value enum like A(u8), but a timestamp field is always required.
        Data::Enum(_) => unimplemented!(),
        // Unsafe and not useful.
        Data::Union(_) => unimplemented!(),
    }
}

fn build_message_name(ident: &Ident) -> (proc_macro2::TokenStream, Ident) {
    let message_name = format_ident!("_MESSAGE_NAME_{}", ident.to_string().to_ascii_uppercase());
    let name_str = ident.to_string().to_ascii_lowercase();

    let code = quote! {
        static #message_name: &str = #name_str;
    };

    (code, message_name)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(ulog::Data));
        }
    }

    generics
}
