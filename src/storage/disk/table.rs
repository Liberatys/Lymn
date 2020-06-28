pub trait Table {
    fn load_column_definition(&mut self) -> bool;
    fn read_column_definition(&self) -> bool;
    fn reoder_data(&mut self) -> bool;
    //TODO: create a row object
    fn get_row(&self, index: usize) -> Vec<String>;
    fn get_colum(&self, index: usize) -> Vec<String>;
    fn insert_row(&mut self, row: Vec<String>) -> bool;
    fn insert_new_column(&mut self, column: String) -> bool;
}
