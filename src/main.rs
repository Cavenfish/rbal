mod db;
mod args;
mod cmds;
mod utils;

use db::init_local;
use args::{RbalArgs, Rbal};
use cmds::{
    add_trans, remove_trans, export_file, 
    import_file, balance, show
};

use clap::Parser;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    init_local();

    let args = RbalArgs::parse();

    match args.command {
        Rbal::Add(cmds) => add_trans(cmds),
        Rbal::Remove(cmds) => remove_trans(cmds),
        Rbal::Export(cmds) => export_file(cmds),
        Rbal::Import(cmds) => import_file(cmds),
        Rbal::Balance => balance(),
        Rbal::Show(cmds) => show(cmds),
    };
}
