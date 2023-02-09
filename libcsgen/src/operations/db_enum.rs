use crate::{error::GenError, helpers};

pub fn generate(name: &str, table_csv: &str, skip_col: bool, extra_docs: bool) -> Result<String, GenError>{
    let dbenum = DatabaseEnum::from_csv(table_csv, skip_col)?;

    let code = dbenum.csharp(name, extra_docs);
    
    Ok(code)
}

pub struct DatabaseEnum(Vec<DatabaseEnumValue>);

impl DatabaseEnum{
    pub fn from_csv(input: &str, notification_parameters: bool) -> Result<Self, GenError>{
        let lines = input.lines();
        let mut vec = Vec::<DatabaseEnumValue>::new();
        for line in lines{
            let parts: Vec<&str> = line.split('\t').collect();
            let index = match parts.get(0){
                Some(p) => p.parse::<i32>()?,
                None => return Err(GenError::new("Invalid input. Missing enum index column."))
            };
            let name = match parts.get(if notification_parameters{2} else {1}) {
                Some(p) => p.to_owned().to_owned(),
                None => return Err(GenError::new("Invalid input. Missing enum name column."))
            };
            let description = match parts.get(if notification_parameters{3} else {2}) {
                Some(p) => p.to_owned().to_owned(),
                None => String::new()
            };
            vec.push(DatabaseEnumValue::new(index, name, description))
        }
        Ok(DatabaseEnum(vec))
    }

    pub fn csharp(&self, name: &str, model_docs: bool) -> String{
        let mut cscode = String::new();
        if model_docs{
            cscode += "/// <summary>\n";
            for value in &self.0{
                cscode += &value.summary();
            }
            cscode+= "/// </summary>\n"
        }

        cscode+= &format!("public enum {}\n{{\n", name);

        for value in &self.0{
            cscode += &value.csharp();
        }

        cscode+="}\n";
        cscode
    }
}

pub struct DatabaseEnumValue{
    index: i32,
    name: String,
    description: String
}

impl DatabaseEnumValue{
    pub fn new(index:i32, name: String, description: String) -> Self {
        let namevec = name.split(&['.', ' ']);
        let mut final_name = String::new();
        for frag in namevec{
            final_name += &helpers::uppercase_first_letter(frag);
        }
        DatabaseEnumValue { index: index, name: final_name, description: description }
    }

    pub fn csharp(&self) -> String{
        format!("/// <summary>\n/// {desc}\n/// </summary>\n{name} = {value},\n", desc = self.description, name = self.name, value = self.index)
    }
    
    pub fn summary(&self) -> String{
        format!("/// {value} = {name} ({desc})\n", desc = self.description, name = self.name, value = self.index)
    }
}
