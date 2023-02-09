use libcsgen::{error::GenError, operations::service};

use crate::copy_to_clipboard;

pub fn operate(args: &Vec<String>, flags: &Vec<String>) -> Result<(), GenError>{
    let name = match args.get(2){
        Some(v) => v,
        None => return Err(GenError::new("Service name not found."))
    };

    print!("Remember to add it to the ServiceCollection!\n");
    let text = service::generate(name)?;
    
    if flags.iter().any(|v| v == "--no-copy"){ print!("{}\n", text); }
    else { copy_to_clipboard(&text); }
    Ok(())
}