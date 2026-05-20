use proc_macro2::TokenStream;
use quote::quote;

use crate::model::{FieldModel, Model};

pub fn expand_impl(model: Model) -> TokenStream {
    let Model {
        struct_name,
        from_type,
        fields,
    } = model;

    let field_defs = fields.iter().map(gen_field_def);
    let field_builds = fields.iter().map(gen_field_build);
    let field_unwraps = fields.iter().map(gen_field_unwrap);

    quote! {
        impl validation_core::ValidateObject for #from_type {
            type Output = #struct_name;
            type Error = validation_core::ValidationErrors;

            fn try_validate(self) -> Result<Self::Output, Self::Error> {
                let dto = self;
                let mut errors = validation_core::ValidationErrors::new();

                #(#field_defs)*

                #(#field_builds)*

                if !errors.is_empty() {
                    return Err(errors);
                }

                Ok(#struct_name {
                    #(#field_unwraps),*
                })
            }
        }
    }
}

fn gen_field_def(field: &FieldModel) -> TokenStream {
    let ident = &field.ident;

    quote! {
        let mut #ident = None;
    }
}

fn gen_field_build(field: &FieldModel) -> TokenStream {
    if field.parse_each.is_some() {
        gen_vec_field(field)
    } else {
        gen_single_field(field)
    }
}

fn gen_vec_field(field: &FieldModel) -> TokenStream {
    let ident = &field.ident;
    let name = &field.name_str;
    let parse_each = field.parse_each.as_ref().unwrap();

    quote! {
        {
            let mut out = Vec::new();

            for (i, item) in dto.#ident.into_iter().enumerate() {
                match #parse_each(item) {
                    Ok(v) => out.push(v),
                    Err(e) => errors.push(validation_core::FieldError {
                        field: #name,
                        index: Some(i),
                        message: e.to_string(),
                    }),
                }
            }

            if errors.is_empty() {
                #ident = Some(out);
            }
        }
    }
}

fn gen_single_field(field: &FieldModel) -> TokenStream {
    let ident = &field.ident;
    let name = &field.name_str;

    let parse_expr = if let Some(parse) = &field.parse {
        quote! { #parse(dto.#ident) }
    } else {
        quote! { dto.#ident.parse() }
    };

    let with_expr = if let Some(with) = &field.with {
        quote! {
            if let Err(e) = #with(&val) {
                errors.push(validation_core::FieldError {
                    field: #name,
                    index: None,
                    message: e.to_string(),
                });
            }
        }
    } else {
        quote! {}
    };

    quote! {
        {
            let parsed = #parse_expr;

            match parsed {
                Ok(mut val) => {
                    if let Ok(v) = validation_core::ValidateObject::try_validate(val) {
                        val = v;
                    }

                    #with_expr

                    #ident = Some(val);
                },
                Err(e) => {
                    errors.push(validation_core::FieldError {
                        field: #name,
                        index: None,
                        message: e.to_string(),
                    });
                }
            }
        }
    }
}

fn gen_field_unwrap(field: &FieldModel) -> TokenStream {
    let ident = &field.ident;

    quote! {
        #ident: #ident.unwrap()
    }
}
