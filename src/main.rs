use clap::Command;

mod precmd;
mod prompt;

fn main() {
    let matches = Command::new("Purs")
        .subcommand(precmd::cli_arguments())
        .subcommand(prompt::cli_arguments())
        .get_matches();

    match matches.subcommand() {
        Some(("precmd", argmatches)) => precmd::display(argmatches),
        Some(("prompt", argmatches)) => prompt::display(argmatches),
        _ => (),
    }
}
