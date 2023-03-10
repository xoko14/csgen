use libcsgen::error::GenError;

pub fn operate() -> Result<(), GenError>{
    print!(r#"C# code generator.

Usage: csgen OPERATION [ARGS] [FLAGS]
            
OPERATION:
    service SERVICE_NAME [--no-copy]
        Generates a service class with its parent interface.
        ARGS:
            SERVICE_NAME: Name of the service class.
    
    repo REPO_NAME [DB_CONTEXT_NAME] [--no-copy]
        Generates a repository class with its parent interface and a EntityFramework DB context instanced by dependency injection.
        ARGS:
            REPO_NAME: Name of the repository class.
            DB_CONTEXT_NAME (optional): Name of the DB context class. Default is "WappingDbContext".
    
    enumdb ENUM_NAME [--no-copy] [--skip-col] [--extra-docs]
        Generates a C# enum from a copied table in csv format (without headers and tabs as column separators).
        ARGS:
            ENUM_NAME: Name of the enum to generate.
        FLAGS:
            --skip-col: Skips second column.
            --extra-docs: Adds all possible enum values in the summary of the enum.

    dbinsert OPERATION_TYPE [--no-copy] [--identity]
        Generates a SQL insert statement based on a csv formatted table (with headers and tabs as column separators).
        ARGS:
            OPERATION_TYPE can be either:
                empty: Doesn't include any value in the insert statement.
                copy: Copies all rows of the table to the values of the insert statement.
        FLAGS:
            --identity: For autogenerated identity tables, omits the first column in the resulting sql insert statement.

    help
        Prints help text.

FLAGS:
    --no-copy: Prints the result instead of copying it to the clipboard.

"#);
    Ok(())
}