mod gen_utils;
mod gen_command;
mod req_res;
use gen_command::{Command,CreateCommand,GenArgs};
use clap::Parser;
fn main() {
    let args=GenArgs::parse();
    match  args.command {
        Command::Create(create)=>{
            match  create {
                 CreateCommand::Module(args)=>{
                   gen_utils:: create_module(args);
                 },
                 CreateCommand::Bloc(args)=>{
                   gen_utils:: create_bloc(&args)
                 },
                 CreateCommand::Repository(args)=>{
                   gen_utils:: create_repository(&args)
                 }
            }
        },
        Command::Delete=>{},
        Command::Update=>{}
    }
}


