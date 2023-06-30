use rand::Rng;

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
            .add_column_int(0, 100)
            .add_column_float(1.0, 2.0)
            .add_column_string_gibberish(1, 10)
            .generate(10);
        println!("{cmd}")
    }
}
