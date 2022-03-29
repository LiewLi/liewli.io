mod cli;
mod gen;

use anyhow::Result;
use cli::{Command, Opt};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt.cmd {
        Command::Gen { url } => gen::gen(&url),
    }
}
