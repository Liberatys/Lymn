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
        let mut table =
            storage::disk::disk_table::DiskTable::new(String::from("tab"), String::from("data"));
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
    fn test_insertion_and_retrieval() {
        let mut ocarina = ocarina::ocarina::OcarinaParser::new(
            "CREATE TABLE dat(col val, go ta);INSERT INTO dat (col) VALUES(t)(g)(t);SELECT col FROM dat",
        );
        ocarina.generate_token_list();
        let resulting_token_list = ocarina.compress_token_list();
        let mut table = storage::disk::disk_table::default_disk_constructor();
        for i in 0..2 {
            let mut executor =
                executor::executor::Executor::new(&resulting_token_list[i], table.clone());
            println!("{:?}", executor.evaluate_query());
        }
        let mut executor =
            executor::executor::Executor::new(&resulting_token_list[2], table.clone());
        assert_eq!(
            format!(
                "{:?}",
                vec![String::from("t"), String::from("g"), String::from("t"),]
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
        let mut table =
            storage::disk::disk_table::DiskTable::new(String::from("tab"), String::from("data"));
        table.insert_new_column("t".to_string());
        table.write();
        let mut executor = executor::executor::Executor::new(&resulting_token_list[0], table);
        assert_eq!("Index: 1", executor.evaluate_query());
    }
}
