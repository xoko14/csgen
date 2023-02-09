use crate::error::GenError;

pub fn generate(name: &str, db_object: &str) -> Result<String, GenError>{
    print!("Remember to add it to the ServiceCollection!\n");
    let text = format!("public interface I{cname}\n{{\n\n}}\n\npublic class {cname}: I{cname}\n{{\nprivate readonly {dbo} _db;\n\npublic {cname}({dbo} db){{\n_db = db;\n}}\n}}", cname = name, dbo = db_object);
    Ok(text)
}