#[macro_use]
extern crate lazy_static;
use std::io;
extern crate printpdf;
#[macro_use]
extern crate prettytable;
extern crate leptess;

mod executor;
mod ocarina;
mod storage;
use ocarina::token::def_determinator::DefDeterminator;

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
    let stdin = io::stdin();
    let input = &mut String::new();
    loop {
        input.clear();
        match stdin.read_line(input) {
            Ok(_) => {
                trim_newline(input);
                if input == "exit" {
                    break;
                }
                let mut ocarina =
                    ocarina::ocarina::OcarinaParser::new(DefDeterminator::new(input.to_owned()));
                ocarina.generate_token_list();
                let resulting_token_list = ocarina.compress_token_list();
                for x in 0..resulting_token_list.len() {
                    if resulting_token_list[x].len() == 0 {
                        continue;
                    }
                    let mut executor = executor::executor::Executor::new(
                        &resulting_token_list[x],
                        storage::disk::pdf_table::default_disk_constructor(),
                    );
                    executor.set_query_vec(input);
                    let return_tuple = executor.evaluate_query();
                    println!("{}", return_tuple.0);
                }
            }
            Err(_) => println!("{}", "help"),
        }
    }
}
