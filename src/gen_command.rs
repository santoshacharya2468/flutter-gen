use clap::{Args, Parser, Subcommand};
#[derive(Debug,Parser)]
pub struct  GenArgs {
    #[clap(subcommand)]
   pub  command: Command
}
#[derive(Debug,Subcommand)]
pub enum Command{
    #[clap(subcommand)]
    Create(CreateCommand),
    Update,
    Delete,
}
#[derive(Debug,Subcommand)]
pub enum CreateCommand{
   Module(CreateArgs),
   Bloc(CreateArgs),
   Repository(CreateArgs)
}
#[derive(Debug,Args)]
pub struct  CreateArgs{
  pub name:String,
  #[clap(long)]
  pub req:Option<String>,
  #[clap(long)]
  pub res:Option<String>
}