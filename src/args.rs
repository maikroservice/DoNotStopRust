use std::fs::read_to_string;
use std::io::{BufRead, stdin};
use clap::Parser;
use anyhow::Result;


#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(
        short = "",
        long = "",
        help = ""
    )]
    pub(crate) argument_1: Option<String>,
}