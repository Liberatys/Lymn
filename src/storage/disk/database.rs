pub trait Database {
    fn load_config(&mut self) -> bool;
    fn write_config(&mut self) -> bool;
}
