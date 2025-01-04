use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(value_enum, short, long)]
    pub action: Option<Action>
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Action {
    Seed,
    Drop,
    All,
}
