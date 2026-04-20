use std::fmt::Write;

use super::model_command::ArgInfo;
use super::model_command::CommandInfo;

pub fn render_markdown(command: &CommandInfo) -> String {
    let mut output = String::new();

    let usage = build_usage_string(command);
    let _ = writeln!(output, "**Usage:** `{usage}`\n");

    if !command.about.is_empty() {
        let _ = writeln!(output, "{}\n", command.about);
    }

    let (positional, options): (Vec<&ArgInfo>, Vec<&ArgInfo>) = command
        .args
        .iter()
        .partition(|a| !a.signature.starts_with('-'));

    render_args_table(&mut output, "Arguments", &positional);
    render_args_table(&mut output, "Options", &options);

    if !command.subcommands.is_empty() {
        render_command_table(&mut output, &command.subcommands);

        for sub in &command.subcommands {
            render_subcommand_section(&mut output, sub, &command.name);
        }
    }

    output
}

fn build_usage_string(command: &CommandInfo) -> String {
    let mut parts = vec![command.name.clone()];

    if command.args.iter().any(|a| a.signature.starts_with('-')) {
        parts.push("[OPTIONS]".to_string());
    }

    for arg in &command.args {
        if !arg.signature.starts_with('-') {
            if arg.required {
                parts.push(arg.signature.clone());
            } else {
                parts.push(format!(
                    "[{}]",
                    arg.signature.trim_matches(|c| c == '<' || c == '>')
                ));
            }
        }
    }

    if !command.subcommands.is_empty() {
        parts.push("<COMMAND>".to_string());
    }

    parts.join(" ")
}

fn render_command_table(output: &mut String, subcommands: &[CommandInfo]) {
    let _ = writeln!(output, "#### Commands\n");
    let _ = writeln!(output, "| Command | Description |");
    let _ = writeln!(output, "|---------|-------------|");

    for sub in subcommands {
        let _ = writeln!(output, "| `{}` | {} |", sub.name, escape_pipes(&sub.about));
    }

    output.push('\n');
}

fn render_subcommand_section(output: &mut String, sub: &CommandInfo, parent_name: &str) {
    let full_name = format!("{parent_name} {}", sub.name);
    let _ = writeln!(output, "### `{full_name}`\n");

    if !sub.about.is_empty() {
        let _ = writeln!(output, "{}\n", sub.about);
    }

    let (positional, options): (Vec<&ArgInfo>, Vec<&ArgInfo>) =
        sub.args.iter().partition(|a| !a.signature.starts_with('-'));

    render_args_table(output, "Arguments", &positional);
    render_args_table(output, "Options", &options);

    if !sub.subcommands.is_empty() {
        render_command_table(output, &sub.subcommands);
        for child in &sub.subcommands {
            render_subcommand_section(output, child, &full_name);
        }
    }
}

fn render_args_table(output: &mut String, heading: &str, args: &[&ArgInfo]) {
    if args.is_empty() {
        return;
    }

    let has_defaults = args.iter().any(|a| a.default.is_some());
    let has_possible_values = args.iter().any(|a| !a.possible_values.is_empty());

    let _ = writeln!(output, "#### {heading}\n");

    let mut header = format!("| {heading} | Description |");
    let mut separator = "|------|------|".to_string();

    if has_defaults {
        header.push_str(" Default |");
        separator.push_str("------|");
    }
    if has_possible_values {
        header.push_str(" Values |");
        separator.push_str("------|");
    }

    let _ = writeln!(output, "{header}");
    let _ = writeln!(output, "{separator}");

    for arg in args {
        let mut row = format!(
            "| `{}` | {} |",
            escape_pipes(&arg.signature),
            escape_pipes(&arg.help),
        );

        if has_defaults {
            let default_cell = arg
                .default
                .as_deref()
                .map(|d| format!("`{}`", escape_pipes(d)))
                .unwrap_or_default();
            row.push_str(&format!(" {default_cell} |"));
        }

        if has_possible_values {
            let values_cell = if arg.possible_values.is_empty() {
                String::new()
            } else {
                arg.possible_values
                    .iter()
                    .map(|v| format!("`{v}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            row.push_str(&format!(" {values_cell} |"));
        }

        let _ = writeln!(output, "{row}");
    }

    output.push('\n');
}

fn escape_pipes(text: &str) -> String {
    text.replace('|', "\\|")
}
