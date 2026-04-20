use super::model_command::ArgInfo;
use super::model_command::CommandInfo;
use super::model_parsed::EnumKind;
use super::model_parsed::FieldRole;
use super::model_parsed::FieldType;
use super::model_parsed::ParsedField;
use super::model_parsed::ParsedSource;
use super::model_parsed::ParsedStruct;
use super::model_parsed::ParsedVariant;
use super::utils::first_sentence;
use super::utils::to_kebab_case;

pub fn build_command_info(
    name: &str,
    parsed_struct: &ParsedStruct,
    source: &ParsedSource,
) -> CommandInfo {
    let about = parsed_struct
        .command_attr
        .about
        .clone()
        .unwrap_or_else(|| first_sentence(&parsed_struct.doc_comment));

    let mut args = Vec::new();
    let mut subcommands = Vec::new();

    collect_fields(&parsed_struct.fields, source, &mut args, &mut subcommands);

    CommandInfo {
        name: name.to_string(),
        about,
        args,
        subcommands,
    }
}

fn collect_fields(
    fields: &[ParsedField],
    source: &ParsedSource,
    args: &mut Vec<ArgInfo>,
    subcommands: &mut Vec<CommandInfo>,
) {
    for field in fields {
        if field.arg_attr.hide {
            continue;
        }

        match field.role {
            FieldRole::Subcommand => {
                if let Some(type_name) = &field.inner_type_name
                    && let Some(sub_enum) = source
                        .enums
                        .iter()
                        .find(|e| e.name == *type_name && e.kind == EnumKind::Subcommand)
                {
                    for variant in &sub_enum.variants {
                        subcommands.push(build_subcommand(variant, source));
                    }
                }
            }
            FieldRole::Flatten => {
                if let Some(type_name) = &field.inner_type_name
                    && let Some(flat_struct) =
                        source.arg_structs.iter().find(|s| s.name == *type_name)
                {
                    collect_fields(&flat_struct.fields, source, args, subcommands);
                }
            }
            FieldRole::Normal => {
                args.push(build_arg_info(field, source));
            }
        }
    }
}

fn build_subcommand(variant: &ParsedVariant, source: &ParsedSource) -> CommandInfo {
    let command_name = variant
        .rename
        .clone()
        .unwrap_or_else(|| to_kebab_case(&variant.name));

    let about = first_sentence(&variant.doc_comment);

    let mut args = Vec::new();
    let mut subcommands = Vec::new();

    if let Some(type_name) = &variant.inner_type_name
        && let Some(inner_struct) = source.arg_structs.iter().find(|s| s.name == *type_name)
    {
        collect_fields(&inner_struct.fields, source, &mut args, &mut subcommands);
    }

    if !variant.fields.is_empty() {
        collect_fields(&variant.fields, source, &mut args, &mut subcommands);
    }

    CommandInfo {
        name: command_name,
        about,
        args,
        subcommands,
    }
}

fn build_arg_info(field: &ParsedField, source: &ParsedSource) -> ArgInfo {
    let attr = &field.arg_attr;

    let help = attr
        .help
        .clone()
        .or_else(|| attr.long_help.clone())
        .unwrap_or_else(|| field.doc_comment.clone());

    let is_flag = matches!(field.type_info, FieldType::Bool);

    let value_name = attr
        .value_name
        .clone()
        .unwrap_or_else(|| field.name.to_uppercase().replace('_', "-"));

    let signature = build_signature(field, &value_name);

    let default = attr
        .default_value
        .clone()
        .or_else(|| attr.default_value_t.clone());

    let required = attr
        .required
        .unwrap_or(matches!(field.type_info, FieldType::Plain(_)) && !is_flag);

    let possible_values = if attr.value_enum {
        field
            .inner_type_name
            .as_ref()
            .and_then(|type_name| {
                source
                    .enums
                    .iter()
                    .find(|e| e.name == *type_name && e.kind == EnumKind::ValueEnum)
            })
            .map(|e| {
                e.variants
                    .iter()
                    .map(|v| v.rename.clone().unwrap_or_else(|| to_kebab_case(&v.name)))
                    .collect()
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    ArgInfo {
        signature,
        help,
        default,
        required,
        possible_values,
    }
}

fn build_signature(field: &ParsedField, value_name: &str) -> String {
    let attr = &field.arg_attr;
    let is_flag = matches!(field.type_info, FieldType::Bool);
    let is_positional = attr.short.is_none() && attr.long.is_none();

    if is_positional {
        return format!("<{value_name}>");
    }

    let mut parts = Vec::new();

    if let Some(short_char) = attr.short {
        if short_char == '\0' {
            if let Some(first_char) = field.name.chars().next() {
                parts.push(format!("-{first_char}"));
            }
        } else {
            parts.push(format!("-{short_char}"));
        }
    }

    if let Some(ref long_name) = attr.long {
        if long_name.is_empty() {
            parts.push(format!("--{}", field.name.replace('_', "-")));
        } else {
            parts.push(format!("--{long_name}"));
        }
    }

    let mut sig = parts.join(", ");

    if !is_flag {
        sig.push_str(&format!(" <{value_name}>"));
    }

    sig
}
