use syn::Attribute;
use syn::Expr;
use syn::Lit;
use syn::Meta;
use syn::meta::ParseNestedMeta;

use super::model_parsed::ArgAttr;
use super::model_parsed::CommandAttr;
use super::model_parsed::FieldRole;

pub fn extract_doc_comment(attrs: &[Attribute]) -> String {
    let lines: Vec<String> = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| {
            if let Meta::NameValue(nv) = &attr.meta
                && let Expr::Lit(expr_lit) = &nv.value
                && let Lit::Str(lit_str) = &expr_lit.lit
            {
                Some(lit_str.value().trim().to_string())
            } else {
                None
            }
        })
        .collect();

    lines.join(" ")
}

pub fn extract_command_attr(attrs: &[Attribute]) -> CommandAttr {
    let mut result = CommandAttr::default();

    for attr in attrs {
        if !attr.path().is_ident("command") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                result.name = meta_str(&meta);
            } else if meta.path.is_ident("about") {
                result.about = meta_str(&meta);
            }
            Ok(())
        });
    }

    result
}

pub fn extract_arg_attr(attrs: &[Attribute]) -> ArgAttr {
    let mut result = ArgAttr::default();

    for attr in attrs {
        if !attr.path().is_ident("arg") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("short") {
                result.short = Some(meta_char(&meta).unwrap_or('\0'));
            } else if meta.path.is_ident("long") {
                result.long = Some(meta_str(&meta).unwrap_or_default());
            } else if meta.path.is_ident("help") {
                result.help = meta_str(&meta);
            } else if meta.path.is_ident("long_help") {
                result.long_help = meta_str(&meta);
            } else if meta.path.is_ident("default_value") {
                result.default_value = meta_str(&meta);
            } else if meta.path.is_ident("default_value_t") {
                result.default_value_t = meta_lit_to_string(&meta);
            } else if meta.path.is_ident("value_name") {
                result.value_name = meta_str(&meta);
            } else if meta.path.is_ident("required") {
                result.required = Some(meta_bool(&meta).unwrap_or(true));
            } else if meta.path.is_ident("hide") {
                result.hide = meta_bool(&meta).unwrap_or(true);
            } else if meta.path.is_ident("value_enum") {
                result.value_enum = true;
            } else if meta.path.is_ident("env") {
                result.env = Some(meta_str(&meta).unwrap_or_default());
            }
            Ok(())
        });
    }

    result
}

pub fn extract_field_role(attrs: &[Attribute]) -> FieldRole {
    for attr in attrs {
        if !attr.path().is_ident("command") {
            continue;
        }
        let mut role = FieldRole::Normal;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("subcommand") {
                role = FieldRole::Subcommand;
            } else if meta.path.is_ident("flatten") {
                role = FieldRole::Flatten;
            }
            Ok(())
        });
        if role != FieldRole::Normal {
            return role;
        }
    }
    FieldRole::Normal
}

pub fn extract_rename_from_attrs(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("clap") && !attr.path().is_ident("command") {
            continue;
        }
        let mut rename = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                rename = meta_str(&meta);
            }
            Ok(())
        });
        if rename.is_some() {
            return rename;
        }
    }
    None
}

fn meta_str(meta: &ParseNestedMeta) -> Option<String> {
    let lit: Lit = meta.value().ok()?.parse().ok()?;
    if let Lit::Str(s) = lit {
        Some(s.value())
    } else {
        None
    }
}

fn meta_char(meta: &ParseNestedMeta) -> Option<char> {
    let lit: Lit = meta.value().ok()?.parse().ok()?;
    if let Lit::Char(c) = lit {
        Some(c.value())
    } else {
        None
    }
}

fn meta_bool(meta: &ParseNestedMeta) -> Option<bool> {
    let lit: Lit = meta.value().ok()?.parse().ok()?;
    if let Lit::Bool(b) = lit {
        Some(b.value)
    } else {
        None
    }
}

fn meta_lit_to_string(meta: &ParseNestedMeta) -> Option<String> {
    let lit: Lit = meta.value().ok()?.parse().ok()?;
    match lit {
        Lit::Str(s) => Some(s.value()),
        Lit::Int(i) => Some(i.to_string()),
        Lit::Float(f) => Some(f.to_string()),
        Lit::Bool(b) => Some(b.value.to_string()),
        _ => None,
    }
}
