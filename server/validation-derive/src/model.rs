use syn::{Ident, Path};

pub struct FieldModel {
    pub ident: Ident,
    pub name_str: String,

    pub parse: Option<Path>,
    pub with: Option<Path>,
    pub parse_each: Option<Path>,
}

pub struct Model {
    pub struct_name: Ident,
    pub from_type: Path,
    pub fields: Vec<FieldModel>,
}
