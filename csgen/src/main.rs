use std::env;

use libcsgen::error::GenError;

use clipboard_win::{formats, set_clipboard};
use operations::{service, repository, help, enumdb, dbinsert};


mod operations;
pub mod io;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut flags = Vec::<String>::new();

    let mut i = 0;
    while i < args.len() {
        if args[i].starts_with("--"){
            let flag = args.remove(i);
            flags.push(flag);
        }
        else {i+=1;}
    }

    match start(&args, &flags){
        Ok(_) => (),
        Err(e) => print!("{}\n", e)
    }
}

fn start(args: &Vec<String>, flags: &Vec<String>) -> Result<(), GenError>{
    let op = match args.get(1){
        Some(v) => match Operation::parse(v) {
            Some(o) => o,
            None => return Err(GenError::new("Operation not supported. Use \"csgen help\" to see a list of supported commands."))
        },
        None => return Err(GenError::new("Option parameter not found. Use \"csgen help\" to see a list of supported commands."))

    };

    match op {
        Operation::Service => service::operate(args, flags)?,
        Operation::Repository => repository::operate(args, flags)?,
        Operation::Help => help::operate()?,
        Operation::EnumDb => enumdb::operate(args, flags)?,
        Operation::DbInsert => dbinsert::operate(args, flags)?,
    };

    Ok(())
}

pub fn copy_to_clipboard(text: &str){
    match set_clipboard(formats::Unicode, text){
        Ok(_) => print!("Copied to clipboard!\n"),
        Err(_) => print!("{}\n", text)
    };
}

enum Operation {
    Service,
    Repository,
    Help,
    EnumDb,
    DbInsert
}

impl Operation{
    fn parse(op: &str) -> Option<Self>{
        match op {
            "service" => Some(Operation::Service),
            "repo" => Some(Operation::Repository),
            "help" => Some(Operation::Help),
            "enumdb" => Some(Operation::EnumDb),
            "dbinsert" => Some(Operation::DbInsert),
            _ => None
        }
    }
}
