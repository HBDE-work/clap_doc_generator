use syn::Type;

use super::model_parsed::FieldType;

pub fn classify_type(ty: &Type) -> FieldType {
    let type_str = type_to_string(ty);

    if type_str == "bool" {
        return FieldType::Bool;
    }
    if let Some(inner) = strip_wrapper(&type_str, "Option") {
        return FieldType::Optional(inner);
    }
    if let Some(inner) = strip_wrapper(&type_str, "Vec") {
        return FieldType::Vec(inner);
    }
    FieldType::Plain(type_str)
}

pub fn type_to_string(ty: &Type) -> String {
    use quote::ToTokens;
    ty.to_token_stream().to_string().replace(' ', "")
}

pub fn extract_inner_type_name(ty: &Type) -> Option<String> {
    let type_str = type_to_string(ty);

    let inner = strip_wrapper(&type_str, "Option")
        .or_else(|| strip_wrapper(&type_str, "Vec"))
        .unwrap_or_else(|| type_str.clone());

    let segment = inner.rsplit("::").next().unwrap_or(&inner);

    let is_user_type = segment.chars().next().is_some_and(|c| c.is_uppercase())
        && !segment.contains('<')
        && !matches!(segment, "String" | "PathBuf" | "OsString");

    is_user_type.then(|| segment.to_string())
}

fn strip_wrapper(type_str: &str, wrapper: &str) -> Option<String> {
    let trimmed = type_str.trim();
    if trimmed.starts_with(wrapper)
        && trimmed[wrapper.len()..].trim_start().starts_with('<')
        && trimmed.ends_with('>')
    {
        let start = trimmed.find('<')? + 1;
        let end = trimmed.len() - 1;
        Some(trimmed[start..end].trim().to_string())
    } else {
        None
    }
}
