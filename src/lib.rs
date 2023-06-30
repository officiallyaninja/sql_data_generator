use rand::Rng;

mod db;
mod query_builder;

// fn generate_query(table: query_builder) -> String {
//     let mut rng = rand::thread_rng();
//     let mut values = "(".to_owned();
//     for column in table.columns {
//         let value: String = match column.data_type {};
//         values += (format!("{value},")).as_ref();
//     }

//     format!("INSERT INTO {} VALUES {})", table.name, values)
// }
#[cfg(test)]
mod tests {
    use crate::query_builder::{Query, Type};

    #[test]
    fn build_simple() {
        let cmd = Query::build("test")
            .add_column(Type::Integer { min: 0, max: 100 })
            .add_column(Type::Float { min: 1.0, max: 2.0 })
            .generate(3);
        println!("{cmd}")
    }
}
