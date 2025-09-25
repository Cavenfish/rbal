mod args;
mod cmds;
mod db;
mod utils;

use args::{Rbal, RbalArgs};
use cmds::{add_trans, balance, coins, export_file, import_file, remove_trans, show};
use db::init_local;

use clap::Parser;

fn main() {
    init_local();

    let args = RbalArgs::parse();

    match args.command {
        Rbal::Add(cmds) => add_trans(cmds),
        Rbal::Remove(cmds) => remove_trans(cmds),
        Rbal::Export(cmds) => export_file(cmds),
        Rbal::Import(cmds) => import_file(cmds),
        Rbal::Balance => balance(),
        Rbal::Coins => coins(),
        Rbal::Show(cmds) => show(cmds),
    };
}
