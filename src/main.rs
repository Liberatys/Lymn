#[macro_use]
extern crate lazy_static;
use std::io;
mod ocarina;
mod storage;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn main() {
    let mut stdin = io::stdin();
    let input = &mut String::new();
    loop {
        input.clear();
        match stdin.read_line(input) {
            Ok(_) => {
                trim_newline(input);
                let mut ocarina = ocarina::ocarina::OcarinaParser::new(input);
                ocarina.generate_token_list();
                let resulting_token_list = ocarina.compress_token_list();
                println!("{:?}", resulting_token_list);
            }
            Err(_) => println!("{}", "help"),
        }
    }
}
