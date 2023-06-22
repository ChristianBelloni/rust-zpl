use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{fs::File, io::Read, path::PathBuf};
use syn::{
    punctuated::Punctuated, token::Comma, DataStruct, DeriveInput, Expr, Field, Fields,
    FieldsNamed, Generics, Lit, LitInt, LitStr, Type,
};

pub fn inner_include_label_config(input: TokenStream, source_path: &str) -> TokenStream {
    let manifest_dir = source_path;

    let raw_input = input.to_string();
    let mut struct_name: Option<String> = None;
    let mut file_path: Option<String> = None;
    for tkn in input {
        match tkn {
            proc_macro2::TokenTree::Literal(lit) => {
                if struct_name.is_none() {
                    _ = struct_name.insert(lit.to_string().replace('\"', ""));
                } else {
                    _ = file_path.insert(lit.to_string().replace('\"', ""));
                }
            }
            proc_macro2::TokenTree::Punct(_) => {
                if struct_name.is_none() {
                    panic!()
                }
            }
            _ => panic!(),
        }
    }
    println!("{raw_input}");
    let struct_name = struct_name.unwrap();
    let mut file_path = file_path.unwrap();
    println!("filename: {file_path}");
    println!("struct_name: {struct_name}");
    let mut cwd = PathBuf::from(manifest_dir);
    file_path = file_path.replace("./", "");
    cwd.push(file_path);
    let full_path = cwd.as_path();
    println!("current_dir {full_path:?}");
    let mut file_contents = File::open(full_path)
        .unwrap_or_else(|_| panic!("Failed to open config file at {full_path:?}"));
    let mut buf = String::new();
    file_contents
        .read_to_string(&mut buf)
        .unwrap_or_else(|_| panic!("Failed to read config file at {full_path:?}"));
    let lines = buf.lines();
    let mut fields = Punctuated::<Field, Comma>::new();
    let mut into_impl_fields = Vec::<Expr>::new();
    for (i, line) in lines.enumerate() {
        let mut splitted = line.split(':');
        let ident_str = splitted
            .next()
            .unwrap_or_else(|| panic!("Failed to read indent at line {i}"));
        let ty_str = splitted
            .next()
            .unwrap_or_else(|| panic!("Failed to read type at line {i}"));

        println!("indent: {ident_str}, type: {ty_str}");
        let ident = format_ident!("{}", ident_str);
        let ty: Type =
            syn::parse_str("String").unwrap_or_else(|_| panic!("Failed to read type at line {i}"));
        let field = Field {
            ident: Some(ident.clone()),
            ty,
            mutability: syn::FieldMutability::None,
            attrs: Default::default(),
            vis: syn::Visibility::Public(Default::default()),
            colon_token: Default::default(),
        };
        fields.push(field);
        let ty = get_field_type(ty_str);
        let expr = parse_field(ident_str, ty);
        into_impl_fields.push(Expr::Verbatim(expr));
        // let expr = match ty {
        //     FieldType::Text(data) => {
        //         quote! {FieldType::Text(value.#ident.clone())}
        //     }
        //     FieldType::CodeEAN13(data) => {
        //         quote! {FieldType::Code(Code::EAN13(value.#ident.clone()))}
        //     }
        //     FieldType::ImagRaw(data) => quote! {FieldType::Code(Code::EAN13(value.#ident.clone()))},
        // };
        // let expr = Expr::Verbatim(expr);
        // into_impl_fields.push((expr, ident_str.to_string()));
    }
    let data_struct = DataStruct {
        struct_token: Default::default(),
        fields: Fields::Named(FieldsNamed {
            brace_token: Default::default(),
            named: fields.clone(),
        }),
        semi_token: None,
    };
    let ident = format_ident!("{}", struct_name);
    let input = DeriveInput {
        attrs: Default::default(),
        vis: syn::Visibility::Inherited,
        ident: ident.clone(),
        generics: Generics::default(),
        data: syn::Data::Struct(data_struct),
    };

    let ident_lit = Lit::Str(LitStr::new(&ident.to_string(), ident.span()));
    quote! {
        #input
        impl From<&#ident> for Label {
            fn from(value: &#ident) -> Self {
                Label {
                    name: String::from(#ident_lit),
                    details: LabelDetails {},
                    contents: vec![#(#into_impl_fields,)*],
                }
            }
        }
    }
}

fn parse_field(ident: &str, ty: FieldType) -> TokenStream {
    let ident = format_ident!("{}", ident);
    let span = ident.span();
    let ident_lit = Lit::Str(LitStr::new(&ident.to_string(), span));
    let x;
    let y;
    let expr: Expr;
    match ty {
        FieldType::Text(data) => {
            x = data.x;
            y = data.y;
            let x_size = data.specific.x_size;
            let y_size = data.specific.y_size;
            let x_size = Lit::Int(LitInt::new(&x_size.to_string(), span));
            let y_size = Lit::Int(LitInt::new(&y_size.to_string(), span));
            expr = Expr::Verbatim(
                quote! {FieldType::Text(TextFieldProperties { x_size: #x_size, y_size: #y_size, contents: value.#ident.clone()})},
            );
        }
        FieldType::CodeEAN13(data) => {
            x = data.x;
            y = data.y;

            expr = Expr::Verbatim(quote! {FieldType::Code(Code::EAN13(value.#ident.clone()))});
        }
        FieldType::ImagRaw(data) => {
            x = data.x;
            y = data.y;
            expr = Expr::Verbatim(quote! {FieldType::Image(value.#ident.clone())});
        }
    };
    let x = Lit::Int(LitInt::new(&x.to_string(), span));
    let y = Lit::Int(LitInt::new(&y.to_string(), span));
    quote! {
        ItemField {
            placeholder: #ident_lit.into(),
            x_pos: #x,
            y_pos: #y,
            contents: #expr,
        }
    }
}

fn get_field_type(raw: &str) -> FieldType {
    let mut splitted = raw.split(',');

    let raw = splitted
        .next()
        .unwrap()
        .trim_start_matches(' ')
        .trim_end_matches(' ');
    let x = splitted
        .next()
        .unwrap_or_else(|| panic!("missing x property"))
        .trim_start_matches(' ')
        .trim_end_matches(' ');
    let y = splitted
        .next()
        .unwrap_or_else(|| panic!("missing y property"))
        .trim_start_matches(' ')
        .trim_end_matches(' ');

    match raw {
        "TEXT" => {
            let x_size = splitted
                .next()
                .unwrap_or_else(|| panic!("missing x property"))
                .trim_start_matches(' ')
                .trim_end_matches(' ');
            let y_size = splitted
                .next()
                .unwrap_or_else(|| panic!("missing y property"))
                .trim_start_matches(' ')
                .trim_end_matches(' ');
            FieldType::Text(FieldProperies {
                x: x.parse()
                    .unwrap_or_else(|_| panic!("failed to parse x property")),
                y: y.parse()
                    .unwrap_or_else(|_| panic!("failed to parse x property")),

                specific: TextProperties {
                    x_size: x_size
                        .parse()
                        .unwrap_or_else(|_| panic!("failed to parse x_size property")),
                    y_size: y_size
                        .parse()
                        .unwrap_or_else(|_| panic!("failed to parse y_size property")),
                },
            })
        }
        "EAN13" => FieldType::CodeEAN13(FieldProperies {
            x: x.parse()
                .unwrap_or_else(|_| panic!("failed to parse x property")),
            y: y.parse()
                .unwrap_or_else(|_| panic!("failed to parse x property")),
            specific: CodeProperties {},
        }),
        "IMAGE" => FieldType::ImagRaw(FieldProperies {
            x: x.parse()
                .unwrap_or_else(|_| panic!("failed to parse x property")),
            y: y.parse()
                .unwrap_or_else(|_| panic!("failed to parse x property")),
            specific: ImageProperties {},
        }),
        _ => panic!("unsupported type {raw}"),
    }
}

#[allow(dead_code)]
enum FieldType {
    Text(FieldProperies<TextProperties>),
    CodeEAN13(FieldProperies<CodeProperties>),
    ImagRaw(FieldProperies<ImageProperties>),
}

struct FieldProperies<T> {
    pub x: u64,
    pub y: u64,
    specific: T,
}

struct TextProperties {
    pub x_size: u64,
    pub y_size: u64,
}

struct CodeProperties {}
struct ImageProperties {}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;
    // cargo test --package config-reader-macro --lib -- include_label_config::test::test_include_label_config --exact --nocapture
    #[test]
    pub fn test_include_label_config() {
        let cwd = concat!(env!("CARGO_MANIFEST_DIR"));

        inner_include_label_config(quote! {"MyLabel", "./test_configs/basic_config.lbl"}, cwd);
    }
}
