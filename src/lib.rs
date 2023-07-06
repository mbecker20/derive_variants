use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(EnumVariants, attributes(variant_derive, variant_attr))]
pub fn derive_partial(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident,
        data,
        ..
    } = syn::parse(input).unwrap();

    let variant_ident = Ident::new(&format!("{}Variants", ident), Span::call_site());

    let variant_derives = attrs
        .iter()
        .find(|attr| attr.path().is_ident("variant_derive"));

    let variant_derives = if let Some(variant_derives) = variant_derives {
        variant_derives
            .parse_args()
            .expect("failed to parse partial_derive")
    } else {
        proc_macro2::TokenStream::new()
    };

    let variant_attrs = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("variant_attr"))
        .map(|attr| {
            attr
                .parse_args::<proc_macro2::TokenStream>()
                .expect("failed to parse variant_attr args")
        });

    let variants = match data {
        Data::Enum(e) => e.variants.into_iter().map(|v| v.ident),
        _ => panic!(""),
    };

    quote! {
        #[derive(#variant_derives)]
		#(#variant_attrs)*
        #vis enum #variant_ident {
            #(#variants),*
        }
    }
    .into()
}
