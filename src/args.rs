// External
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct RbalArgs {

  #[clap(subcommand)]
  pub command: RbalSubcommands,

}

#[derive(Debug, Subcommand)]
pub enum RbalSubcommands {

  /// Add transaction
  Add(TransInfo),

  // Remove transaction
  // TODO

  // Edit Transaction
  // TODO

  /// Import transactions list
  Import(ImportArgs),

  /// Export transactions list
  Export(ExportArgs),

  /// Get net spent
  Balance,

  /// Show all transactions
  Show(ShowArgs),

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
  #[arg(long, default_value_t=0)]
  pub id: u32,

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
  #[arg(long, default_value="today")]
  pub date: String,
}