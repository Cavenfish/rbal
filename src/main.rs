mod db;
mod args;
mod cmds;
mod utils;

use utils::init_local;
use args::{RbalArgs, RbalSubcommands};
use cmds::{add_trans, export_file, balance, show};

use clap::Parser;

fn main() {

    init_local();

    let args = RbalArgs::parse();

    match args.command {
        RbalSubcommands::Add(tran) => add_trans(tran),
        RbalSubcommands::Export(file) => export_file(file),
        RbalSubcommands::Balance => balance(),
        RbalSubcommands::Show => show(),
    };
}
