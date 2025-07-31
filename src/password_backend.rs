use crate::{status::{Status,StatusErr,SecretItem}};
pub trait PasswordBackend {
    fn get(&self, query: &str) -> Result<SecretItem,StatusErr>;
    fn add(&mut self, item: SecretItem) -> Result<Status,StatusErr>;
    fn remove(&mut self, item: SecretItem) -> Result<Status,StatusErr>;
    fn search(&self, filter: &str) -> Vec<SecretItem>;
    fn sync(&mut self) -> Result<Status,StatusErr>;
}
