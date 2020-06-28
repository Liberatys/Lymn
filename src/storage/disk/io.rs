/// interface that all tables/databases have to implement in order to be used by the system
pub trait StorageEntity {
    fn write(&self) -> bool;
    fn read(&mut self) -> bool;
    fn create(&self) -> bool;
    fn exists(&self) -> bool;
    fn delete(&self) -> bool;
    fn size(&self) -> usize;
    fn backup(&self) -> String;
}
