pub trait PasswordBackend {
    fn get(&self, query: &str) -> Result<SecretItem>;
    fn add(&mut self, item: SecretItem) -> Result<Status>;
    fn remove(&mut self, item: SecretItem) -> Result<Status>;
    fn search(&self, filter: &str) -> Vec<SecretItem>;
    fn sync(&mut self) -> Result<Status>;
}
