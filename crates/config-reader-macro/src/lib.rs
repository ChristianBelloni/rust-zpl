#![feature(proc_macro_span)]

use proc_macro::Span;
mod include_label_config;

#[proc_macro]
pub fn include_label_config(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut cwd = Span::call_site().source_file().path();
    while cwd.file_name().unwrap() != "src" {
        cwd.pop();
    }
    cwd.pop();
    include_label_config::inner_include_label_config(input.into(), cwd.to_str().unwrap()).into()
}
