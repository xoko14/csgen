use crate::error::GenError;

pub fn generate_empty(table_csv: &str, is_identity: bool) -> Result<String, GenError> {
    let table = DbTable::parse(table_csv)?;
    let result = table.to_insert(is_identity)?;
    Ok(result)
}

pub fn generate_copy(table_csv: &str, is_identity: bool) -> Result<String, GenError> {
    let mut lines = table_csv.lines();
    let header = match lines.nth(0) {
        Some(h) => h.to_owned(),
        None => return Err(GenError::new("Invalid table input.")),
    };
    let columns = header.split('\t');
    let name = match header.split('\t').next() {
        Some(t) => t.replace("Id", "").replace("ID", ""),
        None => return Err(GenError::new("Error while parsing table.")),
    };
    let column_str = if is_identity {
        columns.skip(1).collect::<Vec<&str>>().join(", ")
    } else {
        columns.collect::<Vec<&str>>().join(", ")
    };
    let sql = format!(
        "insert into {tablename} ({columns}) values ({template})",
        tablename = name,
        columns = column_str,
        template = vec![", "; column_str.chars().filter(|c| c.to_owned() == ',').count()].join("")
    );
    Ok(sql)
}

struct DbTable {
    pub header: DbRow,
    pub body: Vec<DbRow>,
}

struct DbRow {
    pub values: Vec<String>,
}

impl DbTable {
    pub fn parse(csv: &str) -> Result<Self, GenError> {
        let binding = csv.to_owned();
        let mut lines = binding.lines();
        let header = DbRow::parse(match lines.next() {
            Some(h) => h.to_owned(),
            None => return Err(GenError::new("Could not parse pasted table.")),
        });
        let mut body = Vec::<DbRow>::new();
        for line in lines {
            body.push(DbRow::parse(line.to_owned()));
        }
        Ok(DbTable {
            header: header,
            body: body,
        })
    }

    pub fn to_insert(self, is_identity: bool) -> Result<String, GenError> {
        let tablename = match self.header.values.get(0) {
            Some(h) => h.replace("Id", "").replace("ID", ""),
            None => return Err(GenError::new("Can't convert to SQL.")),
        };
        let column_str = if is_identity {
            let borrow_values = &self.header.values;
            borrow_values
                .into_iter()
                .skip(1)
                .map(|i| i.to_owned())
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            self.header.values.join(", ")
        };
        let mut sql = format!(
            "insert into {tablename} ({values}) values\n",
            tablename = tablename,
            values = column_str
        );
        let mut values = Vec::<String>::new();
        for row in self.body {
            let row_values_typed = row
                .values
                .into_iter()
                .map(|i| match i.trim().replace(',', ".").parse::<f32>() {
                    Ok(v) => v.to_string(),
                    Err(_) => {
                        if i == "NULL" {
                            i
                        } else {
                            format!("'{}'", i)
                        }
                    }
                })
                .collect::<Vec<String>>();
            let row_value = if is_identity {
                row_values_typed
                    .into_iter()
                    .skip(1)
                    .collect::<Vec<String>>()
                    .join(", ")
            } else {
                row_values_typed.join(", ")
            };
            values.push(format!("({})", row_value));
        }
        sql = format!("{}{}", sql, values.join(",\n"));
        Ok(sql)
    }
}

impl DbRow {
    pub fn parse(line: String) -> Self {
        let values = line.split('\t').map(|i| i.to_owned()).collect();
        DbRow { values: values }
    }
}
