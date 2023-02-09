use libcsgen::{error::GenError, operations::sql_insert};

use crate::{io::readlines, copy_to_clipboard};

pub fn operate(args: &Vec<String>, flags: &Vec<String>) -> Result<(), GenError>{
    let option = match args.get(2) {
        Some(o) => match DbInsertOption::parse(o){
            Some(o) => o,
            None => return Err(GenError::new("Option not supported."))
        },
        None => return Err(GenError::new("Option not specified."))
    };

    let nocopy = flags.iter().any(|v| v == "--no-copy");
    let identity = flags.iter().any(|v| v == "--identity");

    println!("Paste database table with headers");
    let lines_raw = readlines();
    

    match option{
        DbInsertOption::Copy => {
            let result = sql_insert::generate_copy(&lines_raw, identity)?;
            if nocopy{
                println!("{}", result);
            }
            else{
                copy_to_clipboard(&result)
            }
        },
        DbInsertOption::Insert => {
            let result = sql_insert::generate_empty(&lines_raw, identity)?;
            if nocopy{
                println!("{}", result);
            }
            else{
                copy_to_clipboard(&result)
            }
        },
    }

    Ok(())
}

enum DbInsertOption{
    Copy,
    Insert
}
impl DbInsertOption {
    fn parse(op: &str) -> Option<Self>{
        match op {
            "copy" => Some(DbInsertOption::Copy),
            "empty" => Some(DbInsertOption::Insert),
            _ => None
        }
    }
}