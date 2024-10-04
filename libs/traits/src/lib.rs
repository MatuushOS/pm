use std::{error::Error, path::PathBuf};
pub trait DependencyResolution {
    fn resolve(&mut self) -> Result<(), Box<dyn Error>>;
}

pub trait Filling {
    fn fill(&mut self, f: PathBuf) -> Result<(), Box<dyn Error>>;
}

pub trait Building: DependencyResolution + Filling {
    fn prep(&self) -> Result<(), Box<dyn Error>>;
    fn build(&mut self) -> Result<(), Box<dyn Error>>;
    fn install(&mut self) -> Result<(), Box<dyn Error>>;
    fn remove(&self) -> Result<(), Box<dyn Error>>;
    fn query(&self) -> Result<(), Box<dyn Error>>;
}
