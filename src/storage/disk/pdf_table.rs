use super::io::StorageEntity;
use super::table::Table;
use leptess::{leptonica, tesseract};
use printpdf::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Command;

#[repr(C)]
#[derive(Clone)]
pub struct PDFTable {
    pub name: String,
    pub database_name: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<String>>,
    pub default_path: String,
    pub change_map: Vec<String>,
}

impl PDFTable {
    pub fn new(table_name: String, database_name: String) -> PDFTable {
        let mut table = PDFTable {
            name: table_name,
            database_name,
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

impl StorageEntity for PDFTable {
    fn write(&self) -> bool {
        for x in 0..self.columns.len() {
            let mut configuration_file = PDFTable::convert_path_to_absolute(&self.default_path);
            configuration_file.push_str(format!("/{}.pdf", self.columns[x]).as_ref());
            let (doc, page1, layer1) = PdfDocument::new(
                self.columns[x].clone(),
                Mm(600.0),
                Mm(700.0),
                String::from("Layer 1"),
            );
            let current_layer = doc.get_page(page1).get_layer(layer1);
            current_layer.begin_text_section();
            let font = doc
                .add_external_font(std::fs::File::open("assets/fonts/RobotoMedium.ttf").unwrap())
                .unwrap();
            let mut vec_string: String;
            if self.values[x].len() == 1 {
                vec_string = self.values[x][0].clone();
            } else {
                vec_string = self.values[x].join(",");
            }
            current_layer.set_font(&font, 30);
            current_layer.set_text_cursor(Mm(50.0), Mm(500.0));
            current_layer.set_line_height(33);
            current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
            current_layer.set_character_spacing(10);
            current_layer.write_text(vec_string, &font);
            current_layer.end_text_section();
            doc.save(&mut std::io::BufWriter::new(
                std::fs::File::create(configuration_file).unwrap(),
            ))
                .unwrap();
        }
        true
    }

    fn read(&mut self) -> bool {
        if !self.exists() {
            panic!("Was not able to read from table");
        }
        let mut configuration_file = PDFTable::convert_path_to_absolute(&self.default_path);
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
        self.columns = PDFTable::extract_name_and_columns_from_config(&doc).0;
        let mut data_vec: Vec<Vec<String>> = Vec::new();
        for x in 0..self.columns.len() {
            let mut pdf_file_base = String::new();
            let mut image_file_base = String::new();
            pdf_file_base.push_str(format!("{}.pdf", self.columns[x]).as_ref());
            image_file_base.push_str(format!("{}.jpg", self.columns[x]).as_ref());
            // find a better way to convert a pdf to an image .... only a quick and dirty fix
            Command::new("convert").arg(pdf_file_base).arg(image_file_base.clone()).current_dir(PDFTable::convert_path_to_absolute(&self.default_path)).status().expect("Failed to execute command");
            let mut lt = leptess::LepTess::new(None, "eng").unwrap();
            image_file_base = PDFTable::convert_path_to_absolute(&self.default_path);
            image_file_base.push_str(format!("/{}.jpg", self.columns[x]).as_ref());
            lt.set_image(&image_file_base);
            let mut contents = lt.get_utf8_text().unwrap();
            println!("{}",contents);
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
        let mut configuration_file = PDFTable::convert_path_to_absolute(&self.default_path);
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
                PDFTable::convert_path_to_absolute(&self.default_path.clone());
            current_table_path.push_str(format!("/{}", x.trim()).as_ref());
            let new_file = match fs::File::create(&current_table_path) {
                Ok(v) => v,
                Err(_) => return false,
            };
            PDFTable::compile_column_definition(&mut configuration_content, x);
        }
        configuration_content.push_str("</columns>");
        configuration_content.push_str("</table>");
        new_file.write_all(configuration_content.as_bytes());
        true
    }

    fn exists(&self) -> bool {
        let abs_path = PDFTable::convert_path_to_absolute(&self.default_path);
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
        std::mem::size_of::<PDFTable>()
    }

    fn backup(&self) -> std::string::String {
        todo!()
    }
}

impl Table for PDFTable {
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

    fn insert_new_column(&mut self, column: std::string::String) -> bool {
        self.columns.push(column);
        self.values.push(Vec::new());
        true
    }

    fn get_index_of_column(&self, name: &str) -> usize {
        let index = self.columns.iter().position(|r| r == name).unwrap();
        return index;
    }
    //TODO: write a function that checks if any table exists not just the current
    fn table_exist(&self, table_name: &str) -> bool {
        return if self.name == table_name {
            self.exists()
        } else {
            false
        };
    }
    fn get_columns(&self) -> Vec<String> {
        return self.columns.clone();
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
    fn set_table_name(&mut self, name: String) {
        self.name = name;
        self.default_path = format!("./{}", self.name);
    }

    fn reset_table(&mut self, name: std::string::String, database: std::string::String) -> bool {
        self.columns.clear();
        self.values.clear();
        self.set_table_name(name);
        self.database_name = database;
        true
    }
}

pub fn default_disk_constructor() -> PDFTable {
    return PDFTable::new(String::from(" "), String::from(" "));
}
