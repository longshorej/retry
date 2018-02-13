extern crate clap;

use clap::{ Arg, App, AppSettings };
use std::process::Command;
 
const DEFAULT_ERROR_CODE: i32 = 1;
const DEFAULT_RETRIES: i32 = 3;
const DEFAULT_SUCCESS_CODE: i32 = 0;

fn validate_i32(arg: String) -> Result<(), String> {
    if arg.parse::<i32>().is_ok() {
        Ok(())
    } else {
        Err(format!("cannot parse {} to i32", arg))
    }
}        

fn main() {
    let default_retries = DEFAULT_RETRIES.to_string();
    let args =
        App::new("retry")
        .setting(AppSettings::TrailingVarArg)
        .version("0.0.1")
        .about("Runs a command until it succeeds or has failed a specified number of times")
        .author("Jason Longshore <longshorej@gmail.com>")
        .arg(
            Arg::with_name("retries")
                .long("retries")
                .short("r")
                .takes_value(true)
                .default_value(&default_retries)
                .validator(validate_i32)
                .help("Number of times to retry the command."))
        .arg(
            Arg::with_name("command")
                .multiple(true)
                .required(true))
        .get_matches();

    let command: Vec<&str> = args.values_of("command").unwrap_or_default().collect();

    let (cmd_str, cmd_args) = command.split_at(1);

    let retries =
        args
        .value_of("retries")
        .unwrap_or_default()
        .parse()
        .unwrap_or(DEFAULT_RETRIES);
    
    let mut retries_left = retries;

    fn exit(code: i32) -> () {
        ::std::process::exit(code);
    }

    fn retry_or_exit(retries_left: &mut i32, code: i32) {
        if *retries_left > 1 {
            *retries_left -= 1;
        } else {
            exit(code);
        }
    }

    loop {
        let status =
            Command::new(cmd_str[0])
            .args(cmd_args)
            .status();

        match status {
            Result::Ok(result) => {
                let code = result.code().unwrap_or(DEFAULT_ERROR_CODE);

                if code == DEFAULT_SUCCESS_CODE {
                    exit(DEFAULT_SUCCESS_CODE);
                } else {
                    retry_or_exit(&mut retries_left, code);
                }
            },
            Result::Err(_) => {
                retry_or_exit(&mut retries_left, DEFAULT_ERROR_CODE);
            }
        }
    }
}
