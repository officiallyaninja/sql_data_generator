use std::fmt::format;

use rand::{distributions::Alphanumeric, rngs::ThreadRng, Rng};

pub struct Query {
    rng: ThreadRng,
    table_name: String,
    columns: Vec<Column>,
}
impl Query {
    pub(crate) fn build(name: impl Into<String>) -> Self {
        Self {
            rng: rand::thread_rng(),
            table_name: name.into(),
            columns: Vec::new(),
        }
    }

    fn add_column(&mut self, ty: Type) -> &mut Self {
        self.columns.push(Column {
            name: None,
            data_type: ty,
        });
        self
    }

    pub fn add_column_int(&mut self, min: i128, max: i128) -> &mut Self {
        self.add_column(Type::Integer { min, max })
    }

    pub fn add_column_float(&mut self, min: f64, max: f64) -> &mut Self {
        self.add_column(Type::Float { min, max })
    }

    pub fn add_column_string_gibberish(&mut self, min_len: usize, max_len: usize) -> &mut Self {
        self.add_column(Type::String(StringData::Gibberish {
            min: min_len,
            max: max_len,
        }))
    }

    fn generate_row(&mut self) -> String {
        let mut values = "".to_owned();
        for column in &self.columns {
            let value: String = match &column.data_type {
                Type::Integer { min, max } => format!("{}", self.rng.gen_range(*min..*max)),
                Type::Float { min, max } => format!("{}", self.rng.gen_range(*min..*max)),
                Type::String(data) => match data {
                    StringData::Gibberish { min, max } => rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(self.rng.gen_range(*min..*max))
                        .map(char::from)
                        .collect(),
                },
            };
            values += (format!("{value},")).as_ref();
        }
        values.pop();
        format!("({})", values)
    }

    pub fn generate(&mut self, count: u128) -> String {
        let mut cmd = format!("INSERT INTO {} VALUES ", self.table_name);
        for _ in 0..count {
            let row = self.generate_row();
            cmd += format!("{row},").as_ref();
        }
        cmd.pop();
        cmd.push(';');
        cmd
    }
}
pub(crate) struct Column {
    pub name: Option<String>,
    pub data_type: Type,
}

pub enum Type {
    Integer { min: i128, max: i128 },
    Float { min: f64, max: f64 },
    //DateTime
    String(StringData),
}
pub enum StringData {
    Gibberish { min: usize, max: usize },
}
