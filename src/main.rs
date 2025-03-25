mod db;
mod args;
mod cmds;
mod utils;

use db::init_local;
use args::{RbalArgs, RbalSubcommands};
use cmds::{add_trans, export_file, import_file, balance, show};

use clap::Parser;

fn main() {

    init_local();

    let args = RbalArgs::parse();

    match args.command {
        RbalSubcommands::Add(cmds) => add_trans(cmds),
        RbalSubcommands::Export(cmds) => export_file(cmds),
        RbalSubcommands::Import(cmds) => import_file(cmds),
        RbalSubcommands::Balance => balance(),
        RbalSubcommands::Show(cmd) => show(cmd),
    };
}
