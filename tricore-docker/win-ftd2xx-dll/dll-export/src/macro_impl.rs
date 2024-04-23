use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parenthesized, parse::Parse, punctuated::Punctuated, token::Paren, Error, Ident, LitStr, Token,
};

pub fn re_export(argument: Input, configuration: ExportConfiguration) -> TokenStream {
    let mut all_function_names: Vec<Statement> = argument.args.iter().map(Clone::clone).collect();
    let mut start = TokenStream::new();
    for modified_function in configuration.modified_functions.iter() {
        let Some(statement) = argument.get_properties(&modified_function.name.to_string()) else {
            return Error::new(
                modified_function.name.span(),
                "Function not found in definition",
            )
            .to_compile_error();
        };
        all_function_names.retain_mut(|f| f.name != modified_function.name);

        start.extend(statement.export(&modified_function.patch_type));
    }

    for not_touched_function in all_function_names.iter() {
        let patch_type = PatchType::Stub(Default::default());
        start.extend(not_touched_function.export(&patch_type));
    }
    start
}

#[derive(Clone)]
pub struct Argument {
    arg_type: Ident,
    is_pointer: Option<Token![*]>,
    name: Ident,
}

impl Argument {
    fn rust_type(&self) -> TokenStream {
        let arg_type = &self.arg_type;
        let arg_type = match arg_type.to_string().as_str() {
            "int" => Ident::new("i32", arg_type.span()),
            _ => arg_type.clone(),
        };
        if self.is_pointer.is_some() {
            quote! { *mut #arg_type }
        } else {
            quote! { #arg_type }
        }
    }
}

impl Parse for Argument {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Argument {
            arg_type: input.parse()?,
            is_pointer: input.parse().ok(),
            name: input.parse()?,
        })
    }
}

#[derive(Clone)]
struct FT2XXIdent {
    _ftd2xx_ident: Ident,
}

impl Parse for FT2XXIdent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ftd2xx_ident: Ident = input.parse()?;
        if _ftd2xx_ident != "FTD2XX_API" {
            let error = Error::new(
                input.span(),
                format!(
                    "Expected {:?}, got {:?}",
                    "FTD2XX_API",
                    _ftd2xx_ident.to_string()
                ),
            );
            Err(error)
        } else {
            Ok(FT2XXIdent { _ftd2xx_ident })
        }
    }
}

#[derive(Clone)]
struct WINAPIIdent {
    _winapi: Ident,
}

impl Parse for WINAPIIdent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _winapi: Ident = input.parse()?;
        if _winapi != "WINAPI" {
            let error = Error::new(
                input.span(),
                format!("Expected {:?}, got {:?}", "WINAPI", _winapi.to_string()),
            );
            Err(error)
        } else {
            Ok(WINAPIIdent { _winapi })
        }
    }
}
pub mod patch {
    syn::custom_keyword!(trace);
    syn::custom_keyword!(stub);
    syn::custom_keyword!(defer);
    syn::custom_keyword!(custom);
}

#[derive(Clone)]
pub enum PatchType {
    Stub(patch::stub),
    Defer(patch::defer),
    TraceDefer(patch::trace),
    Custom {
        keyword: patch::custom,
        paren: Paren,
        target: Ident,
    },
}

impl Parse for PatchType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(kw) = input.parse::<patch::stub>() {
            Ok(Self::Stub(kw))
        } else if let Ok(kw) = input.parse::<patch::defer>() {
            Ok(Self::Defer(kw))
        } else if let Ok(kw) = input.parse::<patch::trace>() {
            Ok(Self::TraceDefer(kw))
        } else if let Ok(kw) = input.parse::<patch::custom>() {
            let content;
            Ok(Self::Custom {
                keyword: kw,
                paren: parenthesized!(content in input),
                target: content.parse()?,
            })
        } else {
            Err(Error::new(input.span(), "Unexpected keyword"))
        }
    }
}
#[derive(Clone)]
pub struct Statement {
    _ftd2xx_ident: FT2XXIdent,
    return_type: Ident,
    _winapi: WINAPIIdent,
    name: Ident,
    _paren: Paren,
    arguments: Punctuated<Argument, Token![,]>,
}

impl Statement {
    fn export(&self, patch_type: &PatchType) -> TokenStream {
        let import_name = &self.name;
        let export_name = Ident::new(&format!("_{}", self.name), self.name.span());
        let return_type = &self.return_type;
        let return_type_clone = &return_type;
        let arguments = self.arguments.iter().map(|a| {
            let name = &a.name;
            let arg_type = &a.rust_type();
            quote! { #name: #arg_type }
        });
        let arguments_clone = arguments.clone();
        let call_arguments = self.arguments.iter().map(|a| &a.name);
        let import_name_str = import_name.to_string();
        match patch_type {
            PatchType::Defer(_) => {
                quote! {
                    #[no_mangle]
                    pub extern "stdcall" fn #export_name(#(#arguments),*) -> #return_type {
                        #[link(name = "ftd2xx", kind = "static")]
                        extern "stdcall" {
                            fn #import_name(#(#arguments_clone),*) -> #return_type_clone;
                        }
                        unsafe { #import_name(#(#call_arguments),*) }
                    }
                }
            }
            PatchType::Stub(_) => {
                quote! {
                    #[no_mangle]
                    pub extern "stdcall" fn #export_name(#(#arguments),*) -> #return_type {
                        unimplemented!("Function {} is a stub function", #import_name_str);
                    }
                }
            }
            PatchType::TraceDefer(_) => {
                quote! {
                    #[no_mangle]
                    pub extern "stdcall" fn #export_name(#(#arguments),*) -> #return_type {
                        #[link(name = "ftd2xx", kind = "static")]
                        extern "stdcall" {
                            fn #import_name(#(#arguments_clone),*) -> #return_type_clone;
                        }
                        println!("Tracing {}", #import_name_str);
                        let ret = unsafe { #import_name(#(#call_arguments),*) };
                        println!("Tracing result {:?}", ret);
                        ret
                    }
                }
            }
            PatchType::Custom { target, .. } => {
                quote! {
                    #[no_mangle]
                    pub extern "stdcall" fn #export_name(#(#arguments),*) -> #return_type {
                        match #target(#(#call_arguments),*) {
                            Ok(v) => v,
                            Err(e) => panic!("Patched DLL function {:?} aborted: {:?}", stringify!(#export_name), e)
                        }
                    }
                }
            }
        }
    }
}

impl Parse for Statement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Statement {
            _ftd2xx_ident: input.parse()?,
            return_type: input.parse()?,
            _winapi: input.parse()?,
            name: input.parse()?,
            _paren: parenthesized!(content in input),
            arguments: content.parse_terminated(Argument::parse)?,
        })
    }
}

pub struct Input {
    args: Punctuated<Statement, Token![;]>,
}

impl Input {
    fn get_properties(&self, name: &str) -> Option<&Statement> {
        self.args.iter().find(|s| s.name == name)
    }
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Input {
            args: input.parse_terminated(Statement::parse)?,
        })
    }
}

pub struct FunctionConfiguration {
    name: Ident,
    patch_type: PatchType,
}

impl Parse for FunctionConfiguration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(FunctionConfiguration {
            name: input.parse()?,
            patch_type: input.parse()?,
        })
    }
}

pub struct ExportConfiguration {
    pub literal: LitStr,
    modified_functions: Punctuated<FunctionConfiguration, Token![,]>,
}

impl Parse for ExportConfiguration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(ExportConfiguration {
            literal: input.parse()?,
            modified_functions: input.parse_terminated(FunctionConfiguration::parse)?,
        })
    }
}
