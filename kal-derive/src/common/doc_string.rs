use syn::{Attribute, Lit, Meta, MetaNameValue};

pub fn join_doc_string(attrs: &[Attribute]) -> String {
    let mut result = String::new();
    for attr in attrs {
        if !attr.path.is_ident("doc") {
            continue;
        }
        if let Ok(Meta::NameValue(MetaNameValue {
            lit: Lit::Str(s), ..
        })) = attr.parse_meta()
        {
            result.push_str(&s.value());
        }
    }
    result
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
