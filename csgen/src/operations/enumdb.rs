use libcsgen::{error::GenError, operations::db_enum};

use crate::{copy_to_clipboard, io::readlines};

pub fn operate(args: &Vec<String>, flags: &Vec<String>) -> Result<(), GenError>{
    let name = match args.get(2){
        Some(v) => v,
        None => return Err(GenError::new("Enum name not found."))
    };

    let nocopy = flags.iter().any(|v| v == "--no-copy");
    let skip_col = flags.iter().any(|v| v == "--skip-col");
    let extra_docs = flags.iter().any(|v| v == "--extra-docs");

    print!("Paste database enum table without headers:\n");

    let input = readlines();

    let code = db_enum::generate(name, &input, skip_col, extra_docs)?;

    if nocopy{
        print!("{}", code);
    }
    else{
        copy_to_clipboard(&code);
    }
    
    Ok(())
}
