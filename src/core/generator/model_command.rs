#[derive(Debug)]
pub struct ArgInfo {
    #[cfg_attr(not(feature = "jenkins"), allow(dead_code))]
    pub field_name: String,

    #[cfg_attr(not(feature = "markdown"), allow(dead_code))]
    pub signature: String,

    #[cfg_attr(not(feature = "jenkins"), allow(dead_code))]
    pub long_name: Option<String>,

    #[allow(dead_code)]
    pub short_name: Option<char>,

    #[cfg_attr(not(feature = "markdown"), allow(dead_code))]
    pub help: String,

    pub default: Option<String>,

    pub required: bool,

    #[cfg_attr(not(feature = "jenkins"), allow(dead_code))]
    pub is_flag: bool,

    #[cfg_attr(not(feature = "jenkins"), allow(dead_code))]
    pub is_positional: bool,

    #[cfg_attr(not(feature = "jenkins"), allow(dead_code))]
    pub is_repeatable: bool,

    #[allow(dead_code)]
    pub env: Option<String>,

    #[cfg_attr(not(feature = "markdown"), allow(dead_code))]
    pub possible_values: Vec<String>,
}

#[derive(Debug)]
pub struct CommandInfo {
    pub name: String,
    pub about: String,
    pub args: Vec<ArgInfo>,
    pub subcommands: Vec<CommandInfo>,
}
