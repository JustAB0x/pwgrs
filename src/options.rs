use crate::charset::Charset;
use crate::consts::*;
use crate::{count_chars_in_charsets, get_length_for_entropy};
use clap::{Arg, ArgAction, ArgMatches, Command};

pub struct Options {
    pub generator_options: GeneratorOptions,
    pub subcommand: SubCommand,
}

pub struct GeneratorOptions {
    pub length: u32,
    pub charsets: Vec<Charset>,
}

pub enum SubCommand {
    None(u32),
    Secret(u32),
    WiFi(u32),
}

impl Options {
    pub fn from_args() -> Options {
        let matches = Command::new("pwgrs")
            .version("2.0.0")
            .author("Box <box@sysn.co.uk>")
            .about("Secure password generator")
            .arg(
                Arg::new("LENGTH")
                    .short('l')
                    .long("length")
                    .value_name("LENGTH")
                    .num_args(1)
                    .help("Length of passwords"),
            )
            .arg(
                Arg::new("COUNT")
                    .short('c')
                    .long("count")
                    .value_name("COUNT")
                    .num_args(1)
                    .help("Amount of passwords"),
            )
            .arg(
                Arg::new("ALPHABET")
                    .short('a')
                    .long("alphabet")
                    .action(ArgAction::SetTrue)
                    .help("Use ALPHABET (a-z) charset"),
            )
            .arg(
                Arg::new("ALPHABET_UPPERCASE")
                    .short('A')
                    .long("alphabet-uppercase")
                    .action(ArgAction::SetTrue)
                    .help("Use ALPHABET_UPPERCASE (A-Z) charset"),
            )
            .arg(
                Arg::new("NUMBERS")
                    .short('n')
                    .long("numbers")
                    .action(ArgAction::SetTrue)
                    .help("Use NUMBERS (0-9) charset"),
            )
            .arg(
                Arg::new("SPECIAL")
                    .short('s')
                    .long("special")
                    .action(ArgAction::SetTrue)
                    .help("Use SPECIAL (*, %, -, ...) charset"),
            )
            .subcommand(
                Command::new("secret")
                    .about("Creates secret with at least 256 bits of entropy")
                    .arg(
                        Arg::new("COUNT")
                            .short('c')
                            .long("count")
                            .value_name("COUNT")
                            .num_args(1)
                            .help("Amount of secrets"),
                    ),
            )
            .subcommand(
                Command::new("wifi")
                    .about("Creates a wifi friendly password")
                    .arg(
                        Arg::new("COUNT")
                            .short('c')
                            .long("count")
                            .value_name("COUNT")
                            .num_args(1)
                            .help("Amount of wifi friendly passwords"),
                    ),
            )
            .get_matches();

        let mut charsets: Vec<Charset> = Vec::new();

        if matches.get_flag("ALPHABET") {
            charsets.push(Charset::new(&CHARSET_ALPHABET));
        }
        if matches.get_flag("ALPHABET_UPPERCASE") {
            charsets.push(Charset::new(&CHARSET_ALPHABET_UPPERCASE));
        }
        if matches.get_flag("NUMBERS") {
            charsets.push(Charset::new(&CHARSET_NUMBERS));
        }
        if matches.get_flag("SPECIAL") {
            charsets.push(Charset::new(&CHARSET_SPECIAL));
        }

        if charsets.is_empty() {
            charsets.push(Charset::new(&CHARSET_ALPHABET));
            charsets.push(Charset::new(&CHARSET_ALPHABET_UPPERCASE));
            charsets.push(Charset::new(&CHARSET_NUMBERS));
            charsets.push(Charset::new(&CHARSET_SPECIAL));
        }

        let subcommand = match matches.subcommand() {
            Some(("secret", sub_matches)) => {
                let count = get_count_from_matches(sub_matches);
                SubCommand::Secret(count)
            }
            Some(("wifi", sub_matches)) => {
                let count = get_count_from_matches(sub_matches);
                SubCommand::WiFi(count)
            }
            _ => {
                let count = get_count_from_matches(&matches);
                SubCommand::None(count)
            }
        };

        let length = match matches.get_one::<String>("LENGTH") {
            Some(len) => {
                let parsed: u32 = len.parse().expect("Length must be a positive number");
                if parsed == 0 {
                    panic!("Length must be greater than 0");
                }
                parsed
            }
            None => {
                get_length_for_entropy(MINIMUM_ENTROPY_IN_BITS, count_chars_in_charsets(&charsets))
            }
        };

        Options {
            subcommand,
            generator_options: GeneratorOptions { length, charsets },
        }
    }
}

fn get_count_from_matches(matches: &ArgMatches) -> u32 {
    let count: u32 = matches
        .get_one::<String>("COUNT")
        .unwrap_or(&"1".to_string())
        .parse()
        .expect("Count must be a positive number");
    if count == 0 {
        panic!("Count must be greater than 0");
    }
    count
}
