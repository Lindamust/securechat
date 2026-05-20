use syn::{Data, DeriveInput, LitStr, Path};

use crate::model::{FieldModel, Model};

pub fn parse_input(input: DeriveInput) -> Model {
    let struct_name = input.ident;
    let from_type = extract_from(&input.attrs);

    let fields = match input.data {
        Data::Struct(data) => data.fields.iter().map(parse_field).collect(),
        _ => panic!("ValidateDto only supports structs"),
    };

    Model {
        struct_name,
        from_type,
        fields,
    }
}

fn extract_from(attrs: &[syn::Attribute]) -> Path {
    for attr in attrs {
        if attr.path().is_ident("from") {
            let mut out = None;

            attr.parse_nested_meta(|meta| {
                out = Some(meta.path.clone());
                Ok(())
            })
            .unwrap();

            return out.expect("invalid #[from]");
        }
    }

    panic!("missing #[from(Type)]");
}

fn parse_field(field: &syn::Field) -> FieldModel {
    let ident = field.ident.clone().unwrap();
    let name_str = ident.to_string();

    let mut parse = None;
    let mut with = None;
    let mut parse_each = None;

    for attr in &field.attrs {
        if attr.path().is_ident("validate") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("parse") {
                    parse = Some(parse_meta_value_path(meta.value()?)?);
                }

                if meta.path.is_ident("with") {
                    with = Some(parse_meta_value_path(meta.value()?)?);
                }

                if meta.path.is_ident("parse_each") {
                    parse_each = Some(parse_meta_value_path(meta.value()?)?);
                }

                Ok(())
            })
            .unwrap();
        }
    }

    FieldModel {
        ident,
        name_str,
        parse,
        with,
        parse_each,
    }
}

fn parse_meta_value_path(value: syn::parse::ParseStream) -> syn::Result<Path> {
    let lit: LitStr = value.parse()?;
    lit.parse()
}
