use libcsgen::{error::GenError, operations::repository};

use crate::copy_to_clipboard;

pub fn operate(args: &Vec<String>, flags: &Vec<String>) -> Result<(), GenError>{
    let name = match args.get(2){
        Some(v) => v,
        None => return Err(GenError::new("Repository name not found."))
    };

    let db_object = match args.get(3){
        Some(v) => v,
        None => "DbContext"
    };

    print!("Remember to add it to the ServiceCollection!\n");
    let text = repository::generate(name, db_object)?;

    if flags.iter().any(|v| v == "--no-copy"){ print!("{}\n", text); }
    else { copy_to_clipboard(&text); }
    Ok(())
}