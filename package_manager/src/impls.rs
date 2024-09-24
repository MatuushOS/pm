use std::error::Error;

use clap::Command;
use semver::Version;
use serde::{Deserialize, Serialize};
use sha256::Sha256Digest;
use traits::Building;

#[derive(Serialize, Deserialize, Default)]
struct Deps {
    name: String,
    category: String,
    version: Vec<i32>,
    sha256: String,
}
#[derive(Serialize, Deserialize, Default)]
struct Step {
    name: String,
    cmd: Vec<String>,
}
#[derive(Serialize, Deserialize, Default)]
struct Prepare(Vec<Step>);
#[derive(Serialize, Deserialize, Default)]
struct Build(Vec<Step>);
#[derive(Serialize, Deserialize, Default)]
struct Install(Vec<Step>);
#[derive(Serialize, Deserialize, Default)]
struct Fetch {
    name: String,
    ft: String,
    src: String,
}
#[derive(Serialize, Deserialize, Default)]
pub struct Builder {
    name: String,
    category: String,
    version: (i32, i32, i32),
    sha256: String,
    dependencies: Vec<Deps>,
    dl: Vec<Fetch>,
    prepare: Prepare,
    build: Build,
    install: Install,
}

impl Builder {
    pub fn write(self, path: &str) -> Result<(), Box<dyn Error>> {
        std::fs::write(path, serde_yaml::to_string::<Self>(&Self::default())?)?;
        Ok(())
    }
    fn check(self, pkg: &str) -> Result<String, Box<dyn Error>> {
        Ok(sha256::digest(pkg))
    }
    fn open_and_populate(&mut self, fs: &str) -> Result<(), Box<dyn Error>> {
        
        Ok(Self {
            name: todo!(),
            category: todo!(),
            version: todo!(),
            sha256: todo!(),
            dependencies: todo!(),
            dl: todo!(),
            prepare: todo!(),
            build: todo!(),
            install: todo!(),
        })
    }
}
impl Building for Builder {
    fn prep(&self, pkg: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn build(&self, pkg: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn install(&self, pkg: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn remove(&self, pkg: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn query(&self, pkg: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
