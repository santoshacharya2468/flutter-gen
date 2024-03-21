mod gen_utils;
mod gen_command;
use gen_command::{Command,CreateCommand,GenArgs};
use clap::Parser;
fn main() {
    let args=GenArgs::parse();
    match  args.command {
        Command::Create(create)=>{
            match  create {
                 CreateCommand::Module { name }=>{
                   gen_utils:: create_module(name);
                 },
                 CreateCommand::Bloc { name }=>{
                   gen_utils:: create_bloc(name)
                 },
                 CreateCommand::Repository { name }=>{
                   gen_utils:: create_repository(name)
                 }
            }
        },
        Command::Delete=>{},
        Command::Update=>{}
    }
}


