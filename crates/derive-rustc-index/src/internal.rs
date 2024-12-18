use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsUnnamed, Path, Type, TypePath};

pub fn derive_idx(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let Data::Struct(DataStruct {
        fields: Fields::Unnamed(FieldsUnnamed {
            unnamed: fields, ..
        }),
        ..
    }) = input.data
    else {
        return quote! {
            compile_error!("`#[derive(Idx)` only supports `struct` with unnamed fields");
        };
    };
    if fields.len() != 1 {
        return quote! {
            compile_error!("`#[derive(Idx)` requires exactly one field");
        };
    }
    let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = &fields.first().unwrap().ty
    else {
        return quote! {
            compile_error!("`#[derive(Idx)` requires the field type to be a primitive");
        };
    };
    if segments.len() != 1 {
        return quote! {
            compile_error!("`#[derive(Idx)` requires the field type have just one path segment");
        };
    }
    match segments.first().unwrap().ident.to_string().as_str() {
        "u16" => quote! {
            impl ::ra_ap_rustc_index::Idx for #name {
                fn new(idx: usize) -> Self {
                    Self(u16::try_from(idx).unwrap())
                }

                fn index(self) -> usize {
                    self.0.into()
                }
            }
        },
        "u32" => quote! {
            impl ::ra_ap_rustc_index::Idx for #name {
                fn new(idx: usize) -> Self {
                    Self(u32::try_from(idx).unwrap())
                }

                fn index(self) -> usize {
                    self.0.try_into().unwrap()
                }
            }
        },
        _ => quote! {
            compile_error!("`#[derive(Idx)` requires the field type to be `u16` or `u32`");
        },
    }
}
