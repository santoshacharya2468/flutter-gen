use clap::{Parser, Subcommand};
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
   Module{
     name:String
   },
   Bloc{
    name:String
   },
   Repository{
    name:String
   }
}