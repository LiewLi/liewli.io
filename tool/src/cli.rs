use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Gen {
        #[structopt(short, long)]
        url: String,
    },
}
