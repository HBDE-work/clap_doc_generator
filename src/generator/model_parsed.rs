#[derive(Debug, Clone, Default)]
pub struct ArgAttr {
    pub short: Option<char>,
    pub long: Option<String>,
    pub help: Option<String>,
    pub long_help: Option<String>,
    pub default_value: Option<String>,
    pub default_value_t: Option<String>,
    pub value_name: Option<String>,
    pub required: Option<bool>,
    pub hide: bool,
    pub value_enum: bool,
    pub env: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CommandAttr {
    pub name: Option<String>,
    pub about: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldRole {
    Normal,
    Subcommand,
    Flatten,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FieldType {
    Bool,
    Optional(String),
    Vec(String),
    Plain(String),
}

#[derive(Debug, Default)]
pub struct ParsedSource {
    pub arg_structs: Vec<ParsedStruct>,
    pub enums: Vec<ParsedEnum>,
}

#[derive(Debug, Clone)]
pub struct ParsedStruct {
    pub name: String,
    pub command_attr: CommandAttr,
    pub doc_comment: String,
    pub fields: Vec<ParsedField>,
}

#[derive(Debug, Clone)]
pub struct ParsedField {
    pub name: String,
    pub doc_comment: String,
    pub arg_attr: ArgAttr,
    pub role: FieldRole,
    pub type_info: FieldType,
    pub inner_type_name: Option<String>,
}

#[derive(Debug)]
pub struct ParsedEnum {
    pub name: String,
    pub kind: EnumKind,
    pub variants: Vec<ParsedVariant>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnumKind {
    Subcommand,
    ValueEnum,
}

#[derive(Debug)]
pub struct ParsedVariant {
    pub name: String,
    pub doc_comment: String,
    pub inner_type_name: Option<String>,
    pub fields: Vec<ParsedField>,
    pub rename: Option<String>,
}
