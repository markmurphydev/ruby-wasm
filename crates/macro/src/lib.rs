//! attribution: Substantially copied from `walrus-macro` crate from Walrus library.
//!     I cut out the visitor implementation because we don't use it.
//! https://github.com/wasm-bindgen/walrus
//! MIT licensed.

#![recursion_limit = "256"]

extern crate proc_macro;

use self::proc_macro::TokenStream;
use heck::ToSnakeCase;
use proc_macro2::Span;
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::DeriveInput;
use syn::Error;
use syn::{parse_macro_input, Result, Token};

#[proc_macro_attribute]
pub fn wasm_instr(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let variants = match get_enum_variants(&input) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    assert_eq!(input.ident.to_string(), "Instr");

    let types = create_types(&input.attrs, &variants);
    let builder = create_builder(&variants);

    let expanded = quote! {
        #types
        #builder
    };

    TokenStream::from(expanded)
}

struct WasmVariant {
    syn: syn::Variant,
    opts: WasmVariantOpts,
}

#[derive(Default)]
struct WasmVariantOpts {
    display_name: Option<syn::Ident>,
    display_extra: Option<syn::Ident>,
    skip_builder: bool,
}

fn get_enum_variants(input: &DeriveInput) -> Result<Vec<WasmVariant>> {
    let en = match &input.data {
        syn::Data::Enum(en) => en,
        syn::Data::Struct(_) => {
            panic!("can only put #[wasm_instr] on an enum; found it on a struct")
        }
        syn::Data::Union(_) => {
            panic!("can only put #[wasm_instr] on an enum; found it on a union")
        }
    };
    en.variants
        .iter()
        .cloned()
        .map(|mut variant| {
            Ok(WasmVariant {
                opts: syn::parse(wasm_attrs(&mut variant.attrs))?,
                syn: variant,
            })
        })
        .collect()
}

impl Parse for WasmVariantOpts {
    fn parse(input: ParseStream) -> Result<Self> {
        enum Attr {
            DisplayName(syn::Ident),
            DisplayExtra(syn::Ident),
            SkipBuilder,
        }

        let attrs = Punctuated::<_, syn::token::Comma>::parse_terminated(input)?;
        let mut ret = WasmVariantOpts::default();
        for attr in attrs {
            match attr {
                Attr::DisplayName(ident) => ret.display_name = Some(ident),
                Attr::DisplayExtra(ident) => ret.display_extra = Some(ident),
                Attr::SkipBuilder => ret.skip_builder = true,
            }
        }
        return Ok(ret);

        impl Parse for Attr {
            fn parse(input: ParseStream) -> Result<Self> {
                let attr: syn::Ident = input.parse()?;
                if attr == "display_name" {
                    input.parse::<Token![=]>()?;
                    let name = input.call(syn::Ident::parse_any)?;
                    return Ok(Attr::DisplayName(name));
                }
                if attr == "display_extra" {
                    input.parse::<Token![=]>()?;
                    let name = input.call(syn::Ident::parse_any)?;
                    return Ok(Attr::DisplayExtra(name));
                }
                if attr == "skip_builder" {
                    return Ok(Attr::SkipBuilder);
                }
                Err(Error::new(attr.span(), "unexpected attribute"))
            }
        }
    }
}

fn wasm_attrs(attrs: &mut Vec<syn::Attribute>) -> TokenStream {
    let mut ret = proc_macro2::TokenStream::new();
    let ident = syn::Path::from(syn::Ident::new("wasm", Span::call_site()));
    for i in (0..attrs.len()).rev() {
        if attrs[i].path() != &ident {
            continue;
        }
        let attr = attrs.remove(i);
        let group = if let syn::Meta::List(syn::MetaList { tokens, .. }) = attr.meta {
            tokens
        } else {
            panic!("#[wasm(...)] expected")
        };
        ret.extend(group);
        ret.extend(quote! { , });
    }
    ret.into()
}

fn create_types(attrs: &[syn::Attribute], variants: &[WasmVariant]) -> impl quote::ToTokens {
    let types: Vec<_> = variants
        .iter()
        .map(|v| {
            let name = &v.syn.ident;
            let attrs = &v.syn.attrs;
            let fields = v.syn.fields.iter().map(|f| {
                let name = &f.ident;
                let attrs = &f.attrs;
                let ty = &f.ty;
                quote! {
                    #( #attrs )*
                    pub #name : #ty,
                }
            });
            quote! {
                #( #attrs )*
                #[derive(Clone, Debug)]
                pub struct #name {
                    #( #fields )*
                }

                impl From<#name> for Instr {
                    #[inline]
                    fn from(x: #name) -> Instr {
                        Instr::#name(x)
                    }
                }
            }
        })
        .collect();

    let methods: Vec<_> = variants
        .iter()
        .map(|v| {
            let name = &v.syn.ident;
            let snake_name = name.to_string().to_snake_case();

            let is_name = format!("is_{}", snake_name);
            let is_name = syn::Ident::new(&is_name, Span::call_site());

            let ref_name = format!("{}_ref", snake_name);
            let ref_name = syn::Ident::new(&ref_name, Span::call_site());

            let mut_name = format!("{}_mut", snake_name);
            let mut_name = syn::Ident::new(&mut_name, Span::call_site());

            let unwrap_name = format!("unwrap_{}", snake_name);
            let unwrap_name = syn::Ident::new(&unwrap_name, Span::call_site());

            let unwrap_mut_name = format!("unwrap_{}_mut", snake_name);
            let unwrap_mut_name = syn::Ident::new(&unwrap_mut_name, Span::call_site());

            let ref_name_doc = format!(
                "
                If this instruction is a `{}`, get a shared reference to it.

                Returns `None` otherwise.
            ",
                name
            );

            let mut_name_doc = format!(
                "
                If this instruction is a `{}`, get an exclusive reference to it.

                Returns `None` otherwise.
            ",
                name
            );

            let is_name_doc = format!("Is this instruction a `{}`?", name);

            let unwrap_name_doc = format!(
                "
                Get a shared reference to the underlying `{}`.

                Panics if this instruction is not a `{}`.
            ",
                name, name
            );

            let unwrap_mut_name_doc = format!(
                "
                Get an exclusive reference to the underlying `{}`.

                Panics if this instruction is not a `{}`.
            ",
                name, name
            );

            quote! {
                #[doc=#ref_name_doc]
                #[inline]
                fn #ref_name(&self) -> Option<&#name> {
                    if let Instr::#name(ref x) = *self {
                        Some(x)
                    } else {
                        None
                    }
                }

                #[doc=#mut_name_doc]
                #[inline]
                pub fn #mut_name(&mut self) -> Option<&mut #name> {
                    if let Instr::#name(ref mut x) = *self {
                        Some(x)
                    } else {
                        None
                    }
                }

                #[doc=#is_name_doc]
                #[inline]
                pub fn #is_name(&self) -> bool {
                    self.#ref_name().is_some()
                }

                #[doc=#unwrap_name_doc]
                #[inline]
                pub fn #unwrap_name(&self) -> &#name {
                    self.#ref_name().unwrap()
                }

                #[doc=#unwrap_mut_name_doc]
                #[inline]
                pub fn #unwrap_mut_name(&mut self) -> &mut #name {
                    self.#mut_name().unwrap()
                }
            }
        })
        .collect();

    let variants: Vec<_> = variants
        .iter()
        .map(|v| {
            let name = &v.syn.ident;
            let attrs = &v.syn.attrs;
            quote! {
                #( #attrs )*
                #name(#name)
            }
        })
        .collect();

    quote! {
        #( #types )*

        #( #attrs )*
        pub enum Instr {
            #(#variants),*
        }

        impl Instr {
            #( #methods )*
        }
    }
}

fn create_builder(variants: &[WasmVariant]) -> impl quote::ToTokens {
    let mut builder_methods = Vec::new();
    for variant in variants {
        if variant.opts.skip_builder {
            continue;
        }

        let name = &variant.syn.ident;

        let mut method_name = name.to_string().to_snake_case();

        if method_name == "return" || method_name == "const" {
            method_name.push('_');
        } else if method_name == "block" {
            continue;
        }
        let method_name = syn::Ident::new(&method_name, Span::call_site());

        let mut args = Vec::new();
        let mut arg_names = Vec::new();

        for field in variant.syn.fields.iter() {
            let name = field.ident.as_ref().expect("can't have unnamed fields");
            arg_names.push(name);
            let ty = &field.ty;
            args.push(quote! { #name: #ty });
        }

        let doc = format!(
            "Push a new `{}` instruction onto this builder's block.",
            name
        );

        let arg_names = &arg_names;
        let args = &args;

        builder_methods.push(quote! {
            #[inline]
            #[doc=#doc]
            pub fn #method_name(&self, ctx: &mut crate::CompileCtx<'_>, #(#args),*) -> &Self {
                self.instr(ctx, #name { #(#arg_names),* })
            }
        });
    }
    quote! {
        #[allow(missing_docs)]
        impl crate::InstrSeqBuilder {
            #(#builder_methods)*
        }
    }
}
