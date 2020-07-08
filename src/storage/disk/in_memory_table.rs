use super::io::StorageEntity;
use super::table::Table;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, InMemoryTabel>> = Mutex::new({
        let mut m = HashMap::new();
        m
    });
}

#[repr(C)]
#[derive(Clone)]
pub struct InMemoryTabel {
    pub name: String,
    pub database_name: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<String>>,
}

impl InMemoryTabel {
    pub fn new(table_name: String, database_name: String) -> InMemoryTabel {
        let table = InMemoryTabel {
            name: table_name,
            database_name: database_name,
            columns: Vec::new(),
            values: Vec::new(),
        };
        return table;
    }
}

impl StorageEntity for InMemoryTabel {
    fn write(&self) -> bool {
        let _return_val = HASHMAP
            .lock()
            .unwrap()
            .insert(self.name.clone(), self.clone());
        true
    }

    fn read(&mut self) -> bool {
        let new_table = &HASHMAP.lock().unwrap()[&self.name];
        self.database_name = new_table.database_name.clone();
        self.values = new_table.values.clone();
        self.columns = new_table.columns.clone();
        true
    }

    fn create(&self) -> bool {
        todo!()
    }

    fn exists(&self) -> bool {
        todo!()
    }

    fn delete(&self) -> bool {
        HASHMAP.lock().unwrap().remove(&self.name);
        true
    }
    fn size(&self) -> usize {
        std::mem::size_of::<InMemoryTabel>()
    }
    fn backup(&self) -> std::string::String {
        todo!()
    }
}

impl Table for InMemoryTabel {
    fn load_column_definition(&mut self) -> bool {
        todo!()
    }

    fn read_column_definition(&self) -> bool {
        todo!()
    }

    fn reoder_data(&mut self) -> bool {
        todo!()
    }

    fn get_row(&self, index: usize) -> Vec<std::string::String> {
        let mut value_vec = Vec::new();
        for i in 0..self.values.len() {
            value_vec.push(self.values[i][index].clone());
        }
        return value_vec;
    }

    fn get_colum(&self, index: usize) -> Vec<std::string::String> {
        return self.values[index].clone();
    }

    fn insert_row(&mut self, row: std::vec::Vec<&str>) -> bool {
        for x in 0..row.len() {
            self.values[x].push(row[x].to_string());
        }
        true
    }

    fn insert_new_column(&mut self, column: std::string::String) -> bool {
        self.columns.push(column);
        self.values.push(Vec::new());
        true
    }
    fn get_index_of_column(&self, name: &str) -> usize {
        let index = self.columns.iter().position(|r| r == name).unwrap();
        return index;
    }
    fn has_column(&self, column_name: &str) -> bool {
        let index: i32 = match self.columns.iter().position(|r| r == column_name) {
            Some(v) => v as i32,
            None => -1,
        };
        if index == -1 {
            return false;
        }
        return true;
    }

    fn insert_row_by_column(&mut self, value_map: std::collections::HashMap<&str, String>) -> bool {
        let mut value_vec: Vec<String> = Vec::new();
        for key in self.columns.iter() {
            match value_map.get(&key.as_ref()) {
                Some(v) => value_vec.push(v.to_string()),
                None => {
                    value_vec.push(String::from(" "));
                }
            }
        }
        for x in 0..value_vec.len() {
            self.values[x].push(value_vec[x].clone());
        }
        true
    }
    fn get_columns(&self) -> Vec<String> {
        return self.columns.clone();
    }

    fn table_exist(&self, table_name: &str) -> bool {
        match HASHMAP.lock().unwrap().get(table_name) {
            Some(_) => return true,
            None => return false,
        }
    }
    fn set_table_name(&mut self, _: std::string::String) {
        todo!()
    }
    fn reset_table(&mut self, _: std::string::String, _: std::string::String) -> bool {
        todo!()
    }
}

pub fn default_in_memory_constructor() -> InMemoryTabel {
    return InMemoryTabel::new(String::from(" "), String::from(" "));
}

#[cfg(test)]
mod tests {
    use super::super::io::StorageEntity;
    use super::super::table::Table;
    use super::*;
    #[test]
    fn test_storage_in_memory() {
        let mut table = InMemoryTabel::new(String::from("TABLE"), String::from("DATABASE"));
        table.insert_new_column("test".to_owned());
        table.insert_row(vec![("value")]);
        table.write();
        table = InMemoryTabel::new(String::from("TABLE"), String::from("DATABASE"));
        table.read();
        assert_eq!(table.get_row(0), vec![String::from("value")]);
    }
}
