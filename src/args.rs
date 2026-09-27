use std::fmt;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use crate::cmds::{
    add_trans, balance, coins, edit_trans, export_file, import_file, remove_trans, show,
};
use crate::db::load_db;
use crate::tui::app::App;

#[derive(Debug, Parser)]
pub struct RbalCli {
    #[clap(subcommand)]
    pub command: Rbal,
}

impl RbalCli {
    pub fn run() -> Result<()> {
        let db = load_db()?;
        let args = Self::parse();

        match args.command {
            Rbal::Add(cmds) => add_trans(cmds, db),
            Rbal::Remove(cmds) => remove_trans(cmds, db),
            Rbal::Edit(cmds) => edit_trans(cmds, db),
            Rbal::Export(cmds) => export_file(cmds, db),
            Rbal::Import(cmds) => import_file(cmds, db)?,
            Rbal::Balance => balance(db),
            Rbal::Coins => coins(db),
            Rbal::Show(cmds) => show(cmds, db),
            Rbal::Tui => {
                let terminal = ratatui::init();
                let _ = App::default().run(terminal);
                ratatui::restore();
            }
        };

        Ok(())
    }
}

#[derive(Debug, Subcommand)]
pub enum Rbal {
    /// Add transaction
    Add(TransInfo),

    /// Remove transaction
    Remove(RemoveArgs),

    /// Edit Transaction
    Edit(EditArgs),

    /// Import transactions list
    Import(ImportArgs),

    /// Export transactions list
    Export(ExportArgs),

    /// Get net spent
    Balance,

    /// Show summary of coins used
    Coins,

    /// Show all transactions
    Show(ShowArgs),

    /// Launch TUI
    Tui,
}

#[derive(Debug, Args)]
pub struct ExportArgs {
    /// Filename
    #[arg(short)]
    pub filename: String,
}

#[derive(Debug, Args)]
pub struct ImportArgs {
    /// Filename
    #[arg(short)]
    pub filename: String,
}

#[derive(Debug, Args)]
pub struct ShowArgs {
    /// Transaction ID (Defaults to show all)
    #[arg(long, default_value_t = 0)]
    pub id: u32,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Transaction ID
    #[arg(short, long)]
    pub id: u32,
}

#[derive(Debug, Args)]
pub struct EditArgs {
    /// ID of row to modify
    pub id: u32,

    /// Vendor
    #[arg(short)]
    pub vendor: Option<String>,

    /// Give a description
    #[arg(short)]
    pub message: Option<String>,

    /// Coin used
    #[arg(short, long)]
    pub coin: Option<String>,

    /// Network used
    #[arg(short, long)]
    pub network: Option<String>,

    /// Amount in dollars
    #[arg(short, long)]
    pub amount: Option<f64>,

    /// Date of transaction (Defaults to today)
    #[arg(long, default_value = "today")]
    pub date: Option<String>,
}

#[derive(Debug, Args)]
pub struct TransInfo {
    #[clap(skip)]
    pub id: u32,

    /// Vendor
    #[arg(short)]
    pub vendor: String,

    /// Give a description
    #[arg(short)]
    pub message: String,

    /// Coin used
    #[arg(short, long)]
    pub coin: String,

    /// Network used
    #[arg(short, long)]
    pub network: String,

    /// Amount in dollars
    #[arg(short, long)]
    pub amount: f64,

    /// Date of transaction (Defaults to today)
    #[arg(long, default_value = "today")]
    pub date: String,
}

impl fmt::Display for TransInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
            ID     : {}
            Date   : {}
            Vendor : {}
            Amount : {}
            Coin   : {}
            Network: {}
            Note   : {}
            ",
            self.id, &self.date, &self.vendor, self.amount, &self.coin, &self.network, self.message
        )
    }
}
