#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Literal;
use proc_macro2::TokenStream as TokenStream2;
use syn::TypeGenerics;
use syn::{DeriveInput, DataStruct, DataEnum, DataUnion, FieldsNamed, FieldsUnnamed, punctuated::Punctuated};


#[proc_macro_derive(Layout, attributes())]
pub fn derive_layout(input: TokenStream) -> TokenStream {
    let derive = syn::parse_macro_input!(input as DeriveInput);

    let generics = &derive.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let ident = &derive.ident;
    let data = &derive.data;

    let token_stream = match data {
        syn::Data::Struct(data_struct) => {
            parse_struct(&data_struct, &ident, &ty_generics)
        },
        syn::Data::Enum(data_enum) => {
            parse_enum(data_enum.clone(), ident.clone())
        },
        syn::Data::Union(data_union) => {
            parse_union(data_union.clone(), ident.clone())
        },
    };


    quote::quote! {
        impl #impl_generics ::layout_lib::Layout for #ident #ty_generics #where_clause {
            fn get_layout() -> ::layout_lib::LayoutInfo {
                #token_stream
            }
        }
    }
    .into()
}


fn parse_struct(data_struct: &DataStruct, ident: &Ident, ty_generics: &TypeGenerics) -> TokenStream2 {
    let unit = Punctuated::new();
    let punctuated = match &data_struct.fields {
        syn::Fields::Named(FieldsNamed{named, ..}) => {
            named
        },
        syn::Fields::Unnamed(FieldsUnnamed{unnamed, ..}) => {
            unnamed
        },
        syn::Fields::Unit => { &unit },
    };

    let mut struct_tokens = quote::quote! {
        let mut struct_layout = ::layout_lib::LayoutInfo::default();
        struct_layout.name = std::any::type_name::<#ident #ty_generics>();
        struct_layout.size = std::mem::size_of::<#ident #ty_generics>();
        struct_layout.align = std::mem::align_of::<#ident #ty_generics>();
    };

    let mut i = 0;
    for field in punctuated.iter() {
        let field_ident = &field.ident;
        let (field_ident, field_name) = match field_ident {
            Some(ident) => (quote::quote!{ #ident }, Literal::string(&ident.to_string())),
            None => {
                let num = Literal::i32_unsuffixed(i);
                (quote::quote!{ #num }, Literal::string(&i.to_string()))
            },
        };

        i += 1;

        let field_ty = &field.ty;
        let field_tokens = quote::quote! {
            let mut field = ::layout_lib::Field::default();
            field.name = #field_name;
            field.offset = ::layout_lib::offset_of_struct!(#ident #ty_generics, #field_ident);

            let mut layout = ::layout_lib::LayoutInfo::default();
            layout.name = std::any::type_name::<#field_ty>();
            layout.size = std::mem::size_of::<#field_ty>();
            layout.align = std::mem::align_of::<#field_ty>();
            field.layout = layout;

            struct_layout.fields.push(field);
        };
        struct_tokens.extend(field_tokens.into_iter());
    }

    struct_tokens.extend(quote::quote! {
        struct_layout.fields.sort_by_key(|v| v.offset);
        struct_layout
    }.into_iter());
    struct_tokens
}

fn parse_enum(_data_enum: DataEnum, ident: Ident) -> TokenStream2 {
    let mut struct_tokens = quote::quote! {
        let mut struct_layout = ::layout_lib::LayoutInfo::default();
        struct_layout.name = std::any::type_name::<#ident>();
        struct_layout.size = std::mem::size_of::<#ident>();
        struct_layout.align = std::mem::align_of::<#ident>();
    };

    struct_tokens.extend(quote::quote! {
        struct_layout
    }.into_iter());
    struct_tokens
}

fn parse_union(data_union: DataUnion, ident: Ident) -> TokenStream2 {
    let mut struct_tokens = quote::quote! {
        let mut struct_layout = ::layout_lib::LayoutInfo::default();
        struct_layout.name = std::any::type_name::<#ident>();
        struct_layout.size = std::mem::size_of::<#ident>();
        struct_layout.align = std::mem::align_of::<#ident>();
    };

    let mut i = 0;
    for field in data_union.fields.named.into_iter() {
        let field_ident = &field.ident;
        let field_name = match field_ident{
            Some(ident) => Literal::string(&ident.to_string()),
            None => Literal::string(&i.to_string()),
        };
        i += 1;

        let field_ty = &field.ty;
        let field_tokens = quote::quote! {
            let mut field = Field::default();
            field.name = #field_name;
            field.offset = offset_of_struct!(#ident, #field_ident);

            let mut layout = ::layout_lib::LayoutInfo::default();
            layout.name = std::any::type_name::<#field_ty>();
            layout.size = std::mem::size_of::<#field_ty>();
            layout.align = std::mem::align_of::<#field_ty>();
            field.layout = layout;

            struct_layout.fields.push(field);
        };
        struct_tokens.extend(field_tokens.into_iter());
    }

    struct_tokens.extend(quote::quote! {
        struct_layout
    }.into_iter());
    struct_tokens
}