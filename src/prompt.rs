use clap::{App, Arg, ArgMatches, SubCommand};
use std::env;
use nix::unistd;
use failure::Error;

const INSERT_SYMBOL: &str = "❯";
const COMMAND_SYMBOL: &str = "⬢";
const COMMAND_KEYMAP: &str = "vicmd";
const NO_ERROR: &str = "0";

fn get_username() -> Result<String, Error> {
    Ok(env::var("USER")?)
}

fn get_hostname() -> Result<String, Error> {
    let mut buf = [0u8; 64];
    let hostname_cstr = unistd::gethostname(&mut buf)?;
    let hostname = hostname_cstr.to_str()?;
    Ok(hostname.to_string())
}

pub fn display(sub_matches: &ArgMatches<'_>) {
    let last_return_code = sub_matches.value_of("last_return_code").unwrap_or("0");
    let keymap = sub_matches.value_of("keymap").unwrap_or("US");
    let venv_name = sub_matches.value_of("venv").unwrap_or("");

    let userinfo = get_username().unwrap_or_else(|_|"".to_string());
    let hostinfo = get_hostname().unwrap_or_else(|_|"".to_string());

    let symbol = match keymap {
        COMMAND_KEYMAP => COMMAND_SYMBOL,
        _ => INSERT_SYMBOL,
    };

    let shell_color = match (symbol, last_return_code) {
        (COMMAND_SYMBOL, _) => 3,
        (_, NO_ERROR) => 5,
        _ => 9,
    };

    let venv = match venv_name.len() {
        0 => String::from(""),
        _ => format!("%F{{11}}|{}|%f ", venv_name),
    };

    if sub_matches.is_present("userhost") {
        match userinfo.as_str() {
            "root" => print!("{}%F{{009}}{}%f@%F{{014}}{}%f %F{{{}}}{}%f ", venv, userinfo, hostinfo, shell_color, symbol),
            _ => print!("{}%F{{011}}{}%f@%F{{014}}{}%f %F{{{}}}{}%f ", venv, userinfo, hostinfo, shell_color, symbol),
        }
    } else {
        print!("{}%F{{{}}}{}%f ", venv, shell_color, symbol);
    }
}

pub fn cli_arguments<'a>() -> App<'a, 'a> {
    SubCommand::with_name("prompt")
        .arg(
            Arg::with_name("last_return_code")
                .short("r")
                .takes_value(true),
        )
        .arg(Arg::with_name("keymap").short("k").takes_value(true))
        .arg(Arg::with_name("venv").short("v").long("venv").takes_value(true))
        .arg(Arg::with_name("userhost").short("u").long("uh"))
}
