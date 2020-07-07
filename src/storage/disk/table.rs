use std::collections::HashMap;

pub trait Table {
    fn load_column_definition(&mut self) -> bool;
    fn read_column_definition(&self) -> bool;
    fn reoder_data(&mut self) -> bool;
    //TODO: create a row object
    fn get_row(&self, index: usize) -> Vec<String>;
    fn get_colum(&self, index: usize) -> Vec<String>;
    fn insert_row(&mut self, row: Vec<&str>) -> bool;
    fn insert_new_column(&mut self, column: String) -> bool;
    fn get_index_of_column(&self, name: &str) -> usize;
    fn table_exist(&self, table_name: &str) -> bool;
    fn get_columns(&self) -> Vec<String>;
    fn has_column(&self, column_name: &str) -> bool;
    fn insert_row_by_column(&mut self, value_map: HashMap<&str, String>) -> bool;
}
