use crate::prompt::env::VarError;
use clap::{Arg, ArgAction, ArgMatches, Command};
use nix::unistd;
use std::env;

const COMMAND_SYMBOL: &str = "⬢";
const COMMAND_KEYMAP: &str = "vicmd";
const NO_ERROR: &str = "0";
const SSH_SESSION_ENV: &str = "SSH_TTY";

fn get_username() -> Result<String, VarError> {
    env::var("USER")
}

#[derive(Debug)]
struct HostnameError {
    details: String
}

impl HostnameError {
    fn new(msg: &str) -> HostnameError {
        HostnameError{details: msg.to_string()}
    }
}

impl std::fmt::Display for HostnameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl std::error::Error for HostnameError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn get_hostname() -> Result<String, HostnameError> {
    let hostname = unistd::gethostname().map_err(|_| HostnameError::new("Failed getting hostname"))?;
    let hostname = hostname.into_string().map_err(|_| HostnameError::new("Hostname wasn't valid UTF-8"))?;
    Ok(hostname)
}


fn print_prompt(
    venv: &str,
    userinfo: &str,
    hostinfo: &str,
    shell_color: &i32,
    symbol: &str,
    show_userinfo_hostinfo: bool,
) {
    if show_userinfo_hostinfo {
        if userinfo == "root" {
            println!(
                "{venv}%F{{009}}{userinfo}%f@%F{{014}}{hostinfo}%f %F{{{shell_color}}}{symbol}%f "
            );
        } else {
            println!(
                "{venv}%F{{011}}{userinfo}%f@%F{{014}}{hostinfo}%f %F{{{shell_color}}}{symbol}%f "
            );
        }
    } else {
        println!("{venv} %F{{{shell_color}}}{symbol}%f ");
    }
}

pub fn display(sub_matches: &ArgMatches) {
    let last_return_code = sub_matches
        .get_one::<String>("last_return_code")
        .map(AsRef::as_ref)
        .unwrap_or("0");
    let keymap = sub_matches
        .get_one::<String>("keymap")
        .map(AsRef::as_ref)
        .unwrap_or("US");
    let venv_name = sub_matches
        .get_one::<String>("venv")
        .map(AsRef::as_ref)
        .unwrap_or("");
    let insert_symbol: &str = "❯";
    let insert_symbol = sub_matches
        .get_one::<String>("prompt_symbol")
        .map(AsRef::as_ref)
        .unwrap_or(insert_symbol);
    let _command_symbol: &str = sub_matches
        .get_one::<String>("command_symbol")
        .map(AsRef::as_ref)
        .unwrap_or(COMMAND_SYMBOL);

    let _showinfo = sub_matches.get_flag("userhost");
    let _sshinfo = sub_matches.get_flag("sshinfo");
    let userinfo = get_username().unwrap_or_else(|_| "".to_string());
    let hostinfo = get_hostname().unwrap_or_else(|_| "".to_string());

    let symbol = match keymap {
        COMMAND_KEYMAP => _command_symbol,
        _ => insert_symbol,
    };

    let shell_color = match (symbol, last_return_code) {
        (_command_symbol, _) if _command_symbol == COMMAND_SYMBOL => 3,
        (_, NO_ERROR) => 5,
        _ => 9,
    };

    let venv = match venv_name.len() {
        0 => String::from(""),
        _ => format!("%F{{11}}|{venv_name}|%f "),
    };

    if (_sshinfo && env::var(SSH_SESSION_ENV).is_ok()) || _showinfo {
        print_prompt(&venv, &userinfo, &hostinfo, &shell_color, symbol, true);
    } else {
        print_prompt(&venv, &userinfo, &hostinfo, &shell_color, symbol, false);
    }
}

pub fn cli_arguments() -> clap::Command {
    Command::new("prompt")
        .arg(Arg::new("last_return_code").short('r'))
        .arg(Arg::new("keymap").short('k'))
        .arg(Arg::new("venv").short('v').long("venv"))
        .arg(
            Arg::new("userhost")
                .short('u')
                .long("userhost")
                .help("Posts a $user@$host info prior prompt")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sshinfo")
                .short('s')
                .long("sshinfo")
                .help("Only print $user@$host when inside ssh session")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("prompt_symbol")
                .short('p')
                .long("prompt_symbol")
                .help("Changes the prompt symbol"),
        )
        .arg(
            Arg::new("command_symbol")
                .short('c')
                .long("command_symbol")
                .help("Changes the command symbol (vim mode)"),
        )
}
