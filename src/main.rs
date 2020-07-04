#[macro_use]
extern crate lazy_static;
use std::io;
mod executor;
mod ocarina;
mod storage;
use storage::disk::io::StorageEntity;
use storage::disk::table::Table;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn main() {
    println!("{}", "Lymn Interface - Version 0.0.1");
    let mut stdin = io::stdin();
    let input = &mut String::new();
    let mut table = storage::disk::in_memory_table::InMemoryTabel::new(
        String::from("tab"),
        String::from("data"),
    );
    table.insert_new_column("t".to_owned());
    table.insert_new_column("d".to_owned());
    table.write();
    loop {
        input.clear();
        match stdin.read_line(input) {
            Ok(_) => {
                trim_newline(input);
                let mut ocarina = ocarina::ocarina::OcarinaParser::new(input);
                ocarina.generate_token_list();
                let resulting_token_list = ocarina.compress_token_list();
                for x in 0..resulting_token_list.len() {
                    if resulting_token_list[x].len() == 0 {
                        continue;
                    }
                    let mut executor =
                        executor::executor::Executor::new(&resulting_token_list[x], table.clone());
                    println!("{}", executor.evaluate_query());
                }
            }
            Err(_) => println!("{}", "help"),
        }
    }
}
