use std::error::Error;
pub trait DependencyResolution {
    fn resolve(self, pkg: &str) -> Result<(), Box<dyn Error>>;
}

pub trait Filling {
    fn fill(&mut self, f: &str) -> Result<(), Box<dyn Error>>;
}

pub trait Building: DependencyResolution + Filling {
    fn prep(&self, pkg: &str) -> Result<(), Box<dyn Error>>;
    fn build(&self, pkg: &str) -> Result<(), Box<dyn Error>>;
    fn install(&self, pkg: &str) -> Result<(), Box<dyn Error>>;
    fn remove(&self, pkg: &str) -> Result<(), Box<dyn Error>>;
    fn query(&self, pkg: &str) -> Result<(), Box<dyn Error>>;
}
