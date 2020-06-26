#[macro_use]
extern crate lazy_static;

mod ocarina;

fn main() {
    let mut ocarina = ocarina::ocarina::OcarinaParser::new("SELECT * FROM test WHERE t = '?'");
    ocarina.generate_token_list();
    let resulting_token_list = ocarina.compress_token_list();
    println!("{:?}", resulting_token_list);
}
