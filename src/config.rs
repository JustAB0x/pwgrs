use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(version, about = "A secure password generator", long_about = None)]
pub struct Config {
    #[arg(short, long, default_value_t = 12, help = "Length of the password")]
    pub length: u32,

    #[arg(long, action = ArgAction::SetFalse, help = "Exclude lowercase letters")]
    pub no_lowercase: bool,

    #[arg(long, action = ArgAction::SetFalse, help = "Exclude uppercase letters")]
    pub no_uppercase: bool,

    #[arg(long, action = ArgAction::SetFalse, help = "Exclude numbers")]
    pub no_numbers: bool,

    #[arg(long, action = ArgAction::SetFalse, help = "Exclude symbols")]
    pub no_symbols: bool,

    #[arg(long, help = "Exclude ambiguous characters")]
    pub no_ambiguous: bool,
}