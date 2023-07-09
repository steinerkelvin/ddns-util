use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    // #[clap(subcommand)]
    // command: Command,

    #[clap(short = '4', long, value_enum, default_value = "auto")]
    pub detect_ipv4: DetectIpv4Option,
    #[clap(short = '6', long, value_enum, default_value = "auto")]
    pub detect_ipv6: DetectIpv6Option,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum DetectIpv4Option {
    Auto,
    Nope,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum DetectIpv6Option {
    Auto,
    Nope,
}

#[derive(Subcommand)]
enum Command {}
