use syn::Attribute;
use syn::Fields;
use syn::Item;
use syn::Meta;

use super::attr_parser::extract_arg_attr;
use super::attr_parser::extract_command_attr;
use super::attr_parser::extract_doc_comment;
use super::attr_parser::extract_field_role;
use super::attr_parser::extract_rename_from_attrs;
use super::model_parsed::EnumKind;
use super::model_parsed::ParsedEnum;
use super::model_parsed::ParsedField;
use super::model_parsed::ParsedSource;
use super::model_parsed::ParsedStruct;
use super::model_parsed::ParsedVariant;
use super::type_helper::classify_type;
use super::type_helper::extract_inner_type_name;
use super::type_helper::type_to_string;

pub fn extract_items(items: &[Item], parsed: &mut ParsedSource) {
    for item in items {
        match item {
            Item::Struct(item_struct) if has_derive(&item_struct.attrs, &["Parser", "Args"]) => {
                parsed.arg_structs.push(parse_struct(item_struct));
            }
            Item::Enum(item_enum) => {
                if has_derive(&item_enum.attrs, &["Subcommand"]) {
                    parsed
                        .enums
                        .push(parse_enum(item_enum, EnumKind::Subcommand));
                } else if has_derive(&item_enum.attrs, &["ValueEnum"]) {
                    parsed
                        .enums
                        .push(parse_enum(item_enum, EnumKind::ValueEnum));
                }
            }
            Item::Mod(item_mod) => {
                if let Some((_, ref mod_items)) = item_mod.content {
                    extract_items(mod_items, parsed);
                }
            }
            _ => {}
        }
    }
}

fn has_derive(attrs: &[Attribute], names: &[&str]) -> bool {
    attrs.iter().any(|attr| {
        if let Meta::List(meta_list) = &attr.meta
            && attr.path().is_ident("derive")
        {
            let tokens = meta_list.tokens.to_string();
            return names.iter().any(|name| tokens.contains(name));
        }
        false
    })
}

fn parse_struct(item: &syn::ItemStruct) -> ParsedStruct {
    let fields = match &item.fields {
        Fields::Named(named) => named.named.iter().map(parse_field).collect(),
        _ => Vec::new(),
    };

    ParsedStruct {
        name: item.ident.to_string(),
        command_attr: extract_command_attr(&item.attrs),
        doc_comment: extract_doc_comment(&item.attrs),
        fields,
    }
}

fn parse_enum(item: &syn::ItemEnum, kind: EnumKind) -> ParsedEnum {
    let variants = item
        .variants
        .iter()
        .map(|variant| {
            let (inner_type_name, fields) = match &variant.fields {
                Fields::Unnamed(unnamed) => {
                    let inner = unnamed.unnamed.first().map(|f| type_to_string(&f.ty));
                    (inner, Vec::new())
                }
                Fields::Named(named) => {
                    let fields: Vec<ParsedField> = named.named.iter().map(parse_field).collect();
                    (None, fields)
                }
                Fields::Unit => (None, Vec::new()),
            };

            ParsedVariant {
                name: variant.ident.to_string(),
                doc_comment: extract_doc_comment(&variant.attrs),
                inner_type_name,
                fields,
                rename: extract_rename_from_attrs(&variant.attrs),
            }
        })
        .collect();

    ParsedEnum {
        name: item.ident.to_string(),
        kind,
        variants,
    }
}

fn parse_field(field: &syn::Field) -> ParsedField {
    let field_name = field
        .ident
        .as_ref()
        .map(|i| i.to_string())
        .unwrap_or_default();

    ParsedField {
        name: field_name,
        doc_comment: extract_doc_comment(&field.attrs),
        arg_attr: extract_arg_attr(&field.attrs),
        role: extract_field_role(&field.attrs),
        type_info: classify_type(&field.ty),
        inner_type_name: extract_inner_type_name(&field.ty),
    }
}
