mod args;
mod cmds;
mod db;
mod tui;
mod utils;

use args::RbalCli;
use db::init_local;

fn main() {
    init_local().unwrap();

    RbalCli::run().unwrap();
}
