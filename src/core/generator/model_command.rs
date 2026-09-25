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

/// Selects how generated Jenkins library code invokes the wrapped binary
#[cfg_attr(not(feature = "jenkins"), allow(dead_code))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum ExecutionModel {
    /// Linux shell, via the Jenkins `sh` pipeline step
    #[default]
    Sh,
    /// Windows PowerShell, via the Jenkins `powershell` pipeline step
    Ps,
    /// Windows batch, via the Jenkins `bat` pipeline step
    Bat,
    /// Trusted library code running on the controller itself, via `ProcessBuilder`
    Jvm,
}
