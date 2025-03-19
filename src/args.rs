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

  /// Export transactions list
  Export(ExportArgs),

  /// Get net spent
  Balance,

  /// Show all transactions
  Show,

}

#[derive(Debug, Serialize, Deserialize, Args)]
pub struct TransInfo {

  /// Type of transaction
  #[arg(short)]
  #[serde(rename = "type")]
  pub t: String,

  /// Where
  #[arg(short)]
  #[serde(rename = "where")]
  pub w: String,

  /// For
  #[arg(short)]
  #[serde(rename = "for")]
  pub f: String,

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

#[derive(Debug, Args)]
pub struct ExportArgs {

  /// Filename 
  #[arg(short)]
  pub filename: String,
   
}