use crate::{error::GenError};

pub fn generate(service_name: &str) -> Result<String, GenError>{
    let text = format!("public interface I{cname}\n{{\n\n}}\n\npublic class {cname}: I{cname}\n{{\npublic {cname}(){{\n\n}}\n}}", cname = service_name);
    Ok(text)
}