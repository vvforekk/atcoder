use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "cargo")]
#[clap(bin_name = "cargo")]
pub enum Opt {
    #[clap(name = "acx")]
    #[clap(about, author, version)]
    Acx(AcxOpt),
}

#[derive(Debug, Args)]
pub struct AcxOpt {
    #[clap(subcommand)]
    subcommand: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Start,
    Clist,
}

fn main() -> anyhow::Result<()> {
    let Opt::Acx(opt) = Opt::parse();

    match opt.subcommand {
        Command::Start => cargo_acx::commands::start::start(),
        Command::Clist => cargo_acx::commands::clist::clist(),
    }
}
