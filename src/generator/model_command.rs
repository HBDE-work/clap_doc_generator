#[derive(Debug)]
pub struct ArgInfo {
    pub signature: String,
    pub help: String,
    pub default: Option<String>,
    pub required: bool,
    pub possible_values: Vec<String>,
}

#[derive(Debug)]
pub struct CommandInfo {
    pub name: String,
    pub about: String,
    pub args: Vec<ArgInfo>,
    pub subcommands: Vec<CommandInfo>,
}
