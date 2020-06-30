extern crate Lymn;
use storage::disk::io::StorageEntity;
use storage::disk::table::Table;
use Lymn::ocarina;
use Lymn::storage;
use Lymn::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn selection_query_execution_integration() {
        let mut ocarina = ocarina::ocarina::OcarinaParser::new("SELECT t FROM tab");
        ocarina.generate_token_list();
        let resulting_token_list = ocarina.compress_token_list();
        let mut table = storage::disk::in_memory_table::InMemoryTabel::new(
            String::from("tab"),
            String::from("data"),
        );
        table.insert_new_column("t".to_string());
        table.insert_row(vec!["value"]);
        table.insert_row(vec!["value"]);
        table.insert_row(vec!["value"]);
        table.insert_row(vec!["value"]);
        table.insert_row(vec!["value"]);
        table.insert_row(vec!["value"]);
        table.write();
        let mut executor = executor::executor::Executor::new(&resulting_token_list[0], table);
        assert_eq!(
            format!(
                "{:?}",
                vec![
                    String::from("value"),
                    String::from("value"),
                    String::from("value"),
                    String::from("value"),
                    String::from("value"),
                    String::from("value")
                ]
            ),
            executor.evaluate_query()
        );
    }

    #[test]
    #[ignore]
    fn insert_query_execution_integration() {
        //TODO: if tests fails its probably due to a change in the way value insertion is managed /
        //handled
        let mut ocarina = ocarina::ocarina::OcarinaParser::new("INSERT INTO tab t:value");
        ocarina.generate_token_list();
        let resulting_token_list = ocarina.compress_token_list();
        let mut table = storage::disk::in_memory_table::InMemoryTabel::new(
            String::from("tab"),
            String::from("data"),
        );
        table.insert_new_column("t".to_string());
        table.write();
        let mut executor = executor::executor::Executor::new(&resulting_token_list[0], table);
        assert_eq!("Index: 1", executor.evaluate_query());
    }
}
