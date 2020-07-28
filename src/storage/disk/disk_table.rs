use super::io::StorageEntity;
use super::table::Table;

use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[repr(C)]
#[derive(Clone)]
pub struct DiskTable {
    pub name: String,
    pub database_name: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<String>>,
    pub default_path: String,
    pub change_map: Vec<String>,
}

impl DiskTable {
    pub fn new(table_name: String, database_name: String) -> DiskTable {
        let mut table = DiskTable {
            name: table_name,
            database_name: database_name,
            columns: Vec::new(),
            values: Vec::new(),
            default_path: String::from(""),
            change_map: Vec::new(),
        };
        table.default_path = format!("./{}", table.name);
        return table;
    }

    fn has_changed(&self) -> bool {
        return self.change_map.len() != 0;
    }

    fn only_appended(&self) -> bool {
        let mut counted_inserts = 0;
        self.change_map.iter().map(|v| {
            if v == "INSERT" {
                counted_inserts += 1
            }
        });
        if counted_inserts == self.change_map.len() {
            return true;
        }
        return false;
    }

    pub fn compile_column_definition(configuration_content: &mut String, col: &String) {
        configuration_content.push_str("<column>");
        configuration_content.push_str("\n");
        configuration_content.push_str(format!("<name>{}</name>", col).as_ref());
        configuration_content.push_str("\n");
        configuration_content.push_str("</column>");
        configuration_content.push_str("\n");
    }

    pub fn convert_path_to_absolute(path: &String) -> String {
        let srcdir = PathBuf::from(path);
        return match fs::canonicalize(&srcdir) {
            Ok(v) => v.as_path().to_str().unwrap().trim().to_owned(),
            Err(_) => String::from("Does not exist"),
        };
    }

    pub fn extract_name_and_columns_from_config(
        doc: &roxmltree::Document,
    ) -> (Vec<String>, String) {
        let columns = doc
            .descendants()
            .find(|n| n.has_tag_name("columns"))
            .unwrap();
        let mut columns_vec: Vec<String> = Vec::new();
        for x in columns.descendants() {
            for t in x.children() {
                if t.has_tag_name("name") {
                    columns_vec.push(t.text().unwrap().to_string());
                    break;
                }
            }
        }
        (columns_vec, String::from(" "))
    }
}

impl StorageEntity for DiskTable {
    fn write(&self) -> bool {
        for x in 0..self.columns.len() {
            let mut configuration_file = DiskTable::convert_path_to_absolute(&self.default_path);
            configuration_file.push_str(format!("/{}", self.columns[x]).as_ref());
            //temp: overwrite file content
            let mut file = match OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&configuration_file)
            {
                Ok(v) => v,
                Err(e) => {
                    println!("{}", e);
                    return false;
                }
            };
            if self.only_appended() {
                file = match OpenOptions::new().write(true).open(&configuration_file) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
                        return false;
                    }
                };
            }
            let mut vec_string: String;
            if self.values[x].len() == 1 {
                vec_string = self.values[x][0].clone();
            } else {
                vec_string = self.values[x].join(",");
            }
            if self.only_appended() {
                vec_string.insert(0, ',');
            }
            file.write_all(vec_string.as_bytes());
        }
        true
    }

    fn read(&mut self) -> bool {
        if !self.exists() {
            panic!("Was not able to read from table");
        }
        let mut configuration_file = DiskTable::convert_path_to_absolute(&self.default_path);
        configuration_file.push_str("/config.toml");
        let file = match fs::File::open(&configuration_file) {
            Ok(v) => v,
            Err(_e) => {
                return false;
            }
        };
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents);
        let doc = match roxmltree::Document::parse(&contents) {
            Ok(doc) => doc,
            Err(_e) => {
                return false;
            }
        };
        self.columns = DiskTable::extract_name_and_columns_from_config(&doc).0;
        let mut data_vec: Vec<Vec<String>> = Vec::new();
        for x in 0..self.columns.len() {
            let mut configuration_file = DiskTable::convert_path_to_absolute(&self.default_path);
            configuration_file.push_str(format!("/{}", self.columns[x]).as_ref());
            let mut file = match fs::File::open(&configuration_file) {
                Ok(v) => v,
                Err(e) => {
                    println!("{}", e);
                    return false;
                }
            };
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents);
            if contents.trim() == "" {
                data_vec.push(Vec::new());
                continue;
            }
            //convert string into a vec of Vec<String>
            data_vec.push(
                contents
                    .trim()
                    .split(",")
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect(),
            );
        }
        self.values = data_vec;
        true
    }

    fn create(&self) -> bool {
        if self.exists() {
            return false;
        }
        let res = fs::create_dir_all(&self.default_path);
        match res {
            Ok(_) => {}
            Err(_) => return false,
        }
        //TODO: implement a method to set the root path
        let mut configuration_file = DiskTable::convert_path_to_absolute(&self.default_path);
        configuration_file.push_str("/config.toml");
        let mut new_file = match fs::File::create(&configuration_file) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let mut configuration_content = String::new();
        configuration_content.push_str("<?xml version='1.0' encoding='utf-8'?>");
        configuration_content.push_str("\n");
        configuration_content.push_str("<table>");
        configuration_content.push_str("\n");
        configuration_content.push_str(format!("<name>{}</name>", self.name).as_ref());
        configuration_content.push_str("\n");
        configuration_content.push_str("<columns>");
        configuration_content.push_str("\n");
        for x in &self.columns {
            let mut current_table_path =
                DiskTable::convert_path_to_absolute(&self.default_path.clone());
            current_table_path.push_str(format!("/{}", x.trim()).as_ref());
            let new_file = match fs::File::create(&current_table_path) {
                Ok(v) => v,
                Err(_) => return false,
            };
            DiskTable::compile_column_definition(&mut configuration_content, x);
        }
        configuration_content.push_str("</columns>");
        configuration_content.push_str("</table>");
        new_file.write_all(configuration_content.as_bytes());
        true
    }

    fn exists(&self) -> bool {
        let abs_path = DiskTable::convert_path_to_absolute(&self.default_path);
        let current_path = Path::new(&abs_path);
        if current_path.exists() {
            if current_path.is_dir() {
                return true;
            }
        }
        return false;
    }

    fn delete(&self) -> bool {
        true
    }

    fn size(&self) -> usize {
        std::mem::size_of::<DiskTable>()
    }

    fn backup(&self) -> std::string::String {
        todo!()
    }
}

impl Table for DiskTable {
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
        self.change_map.push("INSERT".to_owned());
        true
    }

    fn set_table_name(&mut self, name: String) {
        self.name = name;
        self.default_path = format!("./{}", self.name);
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
    //TODO: write a function that checks if any table exists not just the current
    fn table_exist(&self, table_name: &str) -> bool {
        if self.name == table_name {
            return self.exists();
        } else {
            return false;
        }
    }

    fn reset_table(&mut self, name: std::string::String, database: std::string::String) -> bool {
        self.columns.clear();
        self.values.clear();
        self.set_table_name(name);
        self.database_name = database;
        true
    }
}

pub fn default_disk_constructor() -> DiskTable {
    return DiskTable::new(String::from(" "), String::from(" "));
}
