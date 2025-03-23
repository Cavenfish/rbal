// External
use clap::{Args, Parser, Subcommand};
use serde::{Serialize, Deserialize};

#[derive(Debug, Parser)]
pub struct RbalArgs {

  #[clap(subcommand)]
  pub command: RbalSubcommands,

}

#[derive(Debug, Subcommand)]
pub enum RbalSubcommands {

  /// Add transaction
  Add(TransInfo),

  /// Import transactions list
  Import(ImportArgs),

  /// Export transactions list
  Export(ExportArgs),

  /// Get net spent
  Balance,

  /// Show all transactions
  Show,

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

#[derive(Debug, Serialize, Deserialize, Args)]
pub struct TransInfo {

  #[clap(skip)]
  pub id: u32,

  /// Vendor
  #[arg(short)]
  #[serde(rename = "where")]
  pub vendor: String,

  /// Give a description
  #[arg(short)]
  #[serde(rename = "for")]
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

  /// Date of transaction
  #[arg(short, long)]
  pub date: String,
}