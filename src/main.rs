#![deny(warnings)]

mod args;

use crate::args::Args;
use clap::Parser;
use std::io::{stdout, BufWriter, Write};

#[tokio::main]
async fn main() {
    println!("{*:}");
}
