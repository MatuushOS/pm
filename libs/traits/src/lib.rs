use std::{error::Error, path::PathBuf};

/// Defines a trait for resolving dependencies.
pub trait DependencyResolution {
    /// Resolves dependencies.
    fn resolve(&mut self) -> Result<(), Box<dyn Error>>;
}

/// Defines a trait for filling something.
pub trait Filling {
    /// Fills something with the contents of the file at the given path.
    fn fill(&mut self, f: PathBuf) -> Result<(), Box<dyn Error>>;
}

/// Defines a trait for building the package
pub trait Building: DependencyResolution + Filling {
    /// Prepares for building.
    fn prep(&self) -> Result<(), Box<dyn Error>>;
    /// Builds the package
    fn build(&mut self) -> Result<(), Box<dyn Error>>;
    /// Installs the package
    fn install(&mut self) -> Result<(), Box<dyn Error>>;
    /// Removes the package
    fn remove(&self) -> Result<(), Box<dyn Error>>;
    /// Queries the package
    fn query(&self) -> Result<(), Box<dyn Error>>;
}
