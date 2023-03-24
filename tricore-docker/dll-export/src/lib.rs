#![feature(proc_macro_span)]
#![feature(proc_macro_def_site)]

use macro_impl::{ExportConfiguration, Input};
use proc_macro::{Span, TokenStream};
use syn::parse_macro_input;

/// This function takes a list of C functions as input and provides exports for
/// these. By default these functions will panic, but individual handlers can be
/// configured to implement desired behavior
///
/// # Example
/// File: **app/header.h**
/// ```text
/// FTD2XX_API FT_STATUS WINAPI FT_OpenEx(PVOID pArg1, DWORD Flags, FT_HANDLE *pHandle);
/// FTD2XX_API FT_STATUS WINAPI FT_Open(int deviceNumber, FT_HANDLE *pHandle);
/// ```
/// File: **app/src/lib.rs**
/// ```no_run
/// fn custom_open(device_number: i32, handle: *mut FT_HANDLE) -> FT_STATUS {
///     /* ... */
///     # unimplemented!()
/// }
/// patch! {
///     "../header.h"
///     FT_Open handler(custom_open),
/// }
/// ```
#[proc_macro]
pub fn patch(attr: TokenStream) -> TokenStream {
    let config = parse_macro_input!(attr as ExportConfiguration);
    let call_location = Span::call_site();
    let mut path = call_location.source_file().path();
    assert!(path.pop());
    let path = path.join(config.literal.value());

    let file = std::fs::read_to_string(path).unwrap();
    let tokens: proc_macro::TokenStream = file.as_str().parse().unwrap();
    let valid_input = parse_macro_input!(tokens as Input);

    macro_impl::re_export(valid_input, config).into()
}
