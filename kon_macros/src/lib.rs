use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::Field;
use syn::Fields;
use syn::GenericParam;
use syn::ItemStruct;
use syn::LifetimeParam;
use syn::Token;
use syn::TypeParam;

type IdentGenerics<'s> = Punctuated<TokenStream, Token![,]>;

fn impl_build(input: &ItemStruct, ident_generics: &IdentGenerics) -> impl ToTokens {
    let ItemStruct {
        ident,
        generics,
        fields,
        ..
    } = input;

    let impl_generics = generics
        .params
        .iter()
        .map(|generic| match generic {
            GenericParam::Lifetime(_) => unimplemented!(),

            GenericParam::Type(generic) => {
                let ident = &generic.ident;
                let bounds = &generic.bounds;
                quote! {
                    #ident: Build + #bounds
                }
            }

            GenericParam::Const(_) => unimplemented!(),
        })
        .collect::<Punctuated<_, Token![,]>>();

    let output = match fields {
        Fields::Named(_) => todo!(),

        Fields::Unnamed(fields) => {
            let items = fields
                .unnamed
                .iter()
                .map(|Field { ty, .. }| quote! { #ty::Output })
                .collect::<Punctuated<_, Token![,]>>();

            quote! { #ident<#items> }
        }

        Fields::Unit => unimplemented!(),
    };

    let build = quote! { todo!() };

    quote! {
        impl<#impl_generics> Build for #ident<#ident_generics> {
            type Output = #output;

            fn build(self) -> Self::Output {
                #build
            }
        }
    }
}

fn impl_resolve_anchors(input: &ItemStruct, ident_generics: &IdentGenerics) -> impl ToTokens {
    let ItemStruct {
        ident,
        generics,
        fields,
        ..
    } = input;

    let impl_generics = generics
        .params
        .iter()
        .map(|generic| match generic {
            GenericParam::Lifetime(_) => unimplemented!(),

            GenericParam::Type(generic) => {
                let ident = &generic.ident;
                let bounds = &generic.bounds;
                quote! {
                    #ident: Build + #bounds
                }
            }

            GenericParam::Const(_) => unimplemented!(),
        })
        .collect::<Punctuated<_, Token![,]>>();

    let anchors_set = match fields {
        Fields::Named(_) => todo!(),

        Fields::Unnamed(fields) => {
            let items = fields
                .unnamed
                .iter()
                .map(|Field { ty, .. }| quote! { #ty::AnchorsSet })
                .collect::<Punctuated<_, Token![,]>>();

            quote! { (#items) }
        }

        Fields::Unit => unimplemented!(),
    };

    let get_anchor = quote! { todo!() };
    let resolve_anchor = quote! { todo!() };

    quote! {
        impl<#impl_generics> ResolveAnchors for #ident<#ident_generics> {
            type AnchorsSet = #anchors_set;

            fn get_anchor<_A: Anchor>(&self) -> Option<Shared<_A::Value>> {
                #get_anchor
            }

            fn resolve_anchor<_A: Anchor>(&mut self, anchor: &Shared<_A::Value>) {
                #resolve_anchor
            }
        }
    }
}

#[proc_macro_derive(Build)]
pub fn derive_build(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let ident_generics = input
        .generics
        .params
        .iter()
        .map(|generic| match generic {
            GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => quote! { #lifetime },
            GenericParam::Type(TypeParam { ident, .. }) => quote! { #ident },
            GenericParam::Const(_) => unimplemented!(),
        })
        .collect::<Punctuated<_, Token![,]>>();

    let impl_build = impl_build(&input, &ident_generics);
    let impl_resolve_anchors = impl_resolve_anchors(&input, &ident_generics);

    quote! {
        #impl_build
        #impl_resolve_anchors
    }
    .into()
}
