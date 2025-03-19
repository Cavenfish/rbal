mod args;
mod cmds;
mod utils;

use args::{RbalArgs, RbalSubcommands};
use cmds::{add_trans, balance, show};

use clap::Parser;

fn main() {

    let args = RbalArgs::parse();

    match args.command {
        RbalSubcommands::Add(tran) => add_trans(tran),
        RbalSubcommands::Balance => balance(),
        RbalSubcommands::Show => show(),
    };
}
