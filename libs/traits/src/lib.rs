use std::error::Error;

pub trait Building {
    fn prep(&self, pkg: &str) -> Result<(), Box<dyn Error>>;
    fn build(&self, pkg: &str) -> Result<(), Box<dyn Error>>;
    fn install(&self, pkg: &str) -> Result<(), Box<dyn Error>>;
    fn remove(&self, pkg: &str) -> Result<(), Box<dyn Error>>;
    fn query(&self, pkg: &str) -> Result<(), Box<dyn Error>>;
}
