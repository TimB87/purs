use clap::*;
use failure::Error;
use nix::unistd;
use std::env;

const COMMAND_SYMBOL: &str = "⬢";
const COMMAND_KEYMAP: &str = "vicmd";
const NO_ERROR: &str = "0";
const SSH_SESSION_ENV: &str = "SSH_TTY";

fn get_username() -> Result<String, Error> {
    Ok(env::var("USER")?)
}

fn get_hostname() -> Result<String, Error> {
    let mut buf = [0u8; 64];
    let hostname_cstr = unistd::gethostname(&mut buf)?;
    let hostname = hostname_cstr.to_str()?;
    Ok(hostname.to_string())
}

pub fn display(sub_matches: &ArgMatches) {
    let binding = "0".to_owned();
    let last_return_code = sub_matches.get_one::<String>("last_return_code").unwrap_or(&binding);
    let binding = "US".to_owned();
    let keymap = sub_matches.get_one::<String>("keymap").unwrap_or(&binding);
    let binding = "".to_owned();
    let venv_name = sub_matches.get_one::<String>("venv").unwrap_or(&binding);
    let insert_symbol: &str = "❯";
    let binding = insert_symbol.to_owned();
    let insert_symbol = sub_matches
        .get_one::<String>("prompt_symbol")
        .unwrap_or(&binding);
    let binding = COMMAND_SYMBOL.to_owned();
    let _command_symbol: &str = sub_matches
        .get_one::<String>("command_symbol")
        .unwrap_or(&binding);
    //.value_of("command_symbol")

    let _showinfo = sub_matches.contains_id("userhost");
    let _sshinfo = sub_matches.contains_id("sshinfo");
    let userinfo = get_username().unwrap_or_else(|_| "".to_string());
    let hostinfo = get_hostname().unwrap_or_else(|_| "".to_string());

    let symbol = match keymap.as_str() {
        COMMAND_KEYMAP => _command_symbol,
        _ => insert_symbol,
    };

    let shell_color = match (symbol, last_return_code.as_str()) {
        (_command_symbol, _) if _command_symbol == COMMAND_SYMBOL => 3,
        (_, NO_ERROR) => 5,
        _ => 9,
    };

    let venv = match venv_name.len() {
        0 => String::from(""),
        _ => format!("%F{{11}}|{}|%f ", venv_name),
    };

    if _showinfo && _sshinfo {
        match env::var(SSH_SESSION_ENV) {
            Ok(_) => match userinfo.as_str() {
                "root" => print!(
                    "{}%F{{009}}{}%f@%F{{014}}{}%f %F{{{}}}{}%f ",
                    venv, userinfo, hostinfo, shell_color, symbol
                ),
                _ => print!(
                    "{}%F{{011}}{}%f@%F{{014}}{}%f %F{{{}}}{}%f ",
                    venv, userinfo, hostinfo, shell_color, symbol
                ),
            },
            Err(_) => {
                print!("{}%F{{{}}}{}%f ", venv, shell_color, symbol);
            }
        }
    } else if _showinfo {
        match userinfo.as_str() {
            "root" => print!(
                "{}%F{{009}}{}%f@%F{{014}}{}%f %F{{{}}}{}%f ",
                venv, userinfo, hostinfo, shell_color, symbol
            ),
            _ => print!(
                "{}%F{{011}}{}%f@%F{{014}}{}%f %F{{{}}}{}%f ",
                venv, userinfo, hostinfo, shell_color, symbol
            ),
        }
    } else {
        print!("{}%F{{{}}}{}%f ", venv, shell_color, symbol);
    }
}

pub fn cli_arguments<'a>() -> clap::Command {
    Command::new("prompt")
        .arg(Arg::new("last_return_code").short('r'))
        .arg(Arg::new("keymap").short('k'))
        .arg(Arg::new("venv").short('v').long("venv"))
        .arg(
            Arg::new("userhost")
                .short('u')
                .long("userhost")
                .help("Posts a $user@$host info prior prompt").action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sshinfo")
                .short('s')
                .long("sshinfo")
                .help("Only print $user@$host when inside ssh session").action(ArgAction::SetTrue),
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
