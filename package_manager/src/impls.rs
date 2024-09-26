// use compress_tools::Ownership;
use fetch_data::hash_download;
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::{
    env::temp_dir,
    error::Error,
    fs::read_to_string,
    path::Path,
    process::{exit, Command},
};
use traits::{Building, DependencyResolution, Filling};
#[derive(Serialize, Deserialize, Default, Clone)]
struct Deps {
    name: String,
    category: String,
    version: Vec<i32>,
    sha256: String,
}
#[derive(Serialize, Deserialize, Default, Clone)]
struct Step {
    name: String,
    cmd: Vec<String>,
}
#[derive(Serialize, Deserialize, Default, Clone)]
struct Prepare(Vec<Step>);
#[derive(Serialize, Deserialize, Default, Clone)]
struct Build(Vec<Step>);
#[derive(Serialize, Deserialize, Default, Clone)]
struct Install(Vec<Step>);
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Fetch {
    name: String,
    ft: String,
    pub src: String,
    pub sha256: String,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Builder {
    name: String,
    pub category: String,
    version: (i32, i32, i32),
    sha256: String,
    dependencies: Vec<Deps>,
    pub dl: Vec<Fetch>,
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
        Ok(sha256::try_digest(Path::new(pkg)).unwrap())
    }
}
impl Filling for Builder {
    fn fill(&mut self, f: &str) -> Result<(), Box<dyn Error>> {
        let f = read_to_string(f)?;
        let cfg: Self = from_str(&f)?;
        self.name = cfg.name;
        self.category = cfg.category;
        self.version = cfg.version;
        self.dependencies = cfg.dependencies;
        self.prepare = cfg.prepare;
        self.build = cfg.build;
        self.install = cfg.install;
        self.dl = cfg.dl;
        Ok(())
    }
}
impl DependencyResolution for Builder {
    fn resolve(&mut self) -> Result<(), Box<dyn Error>> {
        &self.prep()?;
        &self.build()?;
        &self.install()?;
        Ok(())
    }
}
impl Building for Builder {
    /// Mainly dependency resolution and downloads
    fn prep(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.dependencies.is_empty() {
            println!("Nothing to resolve");
        } else {
            for i in self.dependencies.iter() {
                self.clone()
                    .fill(format!("{}/{}.yml", i.category, i.name).as_str())?;
                self.clone().resolve()?;
            }
        }
        println!("Making package {}", self.name);
        for i in self.dl.clone().iter() {
            println!("Downloading {}.{} to {}", i.name, i.ft, i.src);
            let path = Path::new(temp_dir().clone().as_path()).join(format!("{}{}", i.name, i.ft));
            if hash_download(i.clone().src, &path)? != i.sha256 {
                std::fs::remove_file(path)?;
                eprintln!("FILE IS UNSAFE TO USE! STOPPING THE OPENRATION NOW!!!");
                exit(1);
            } else {
                // compress_tools::uncompress_archive(&path, "src", Ownership::Preserve)?;
            }
        }
        println!("Running pre-build steps");
        (&self.prepare.0).into_iter().for_each(|i| {
            let arge = i.cmd.len();
            println!("\tRunning step {}", i.name);
            match Command::new(i.cmd[0].clone())
                .args(&i.cmd[1..arge])
                .output()
            {
                Ok(ok) => println!("{:#?}", ok.stdout.iter()),
                Err(e) => {
                    eprintln!("{:#?}", e);
                    exit(1)
                }
            }
        });
        Ok(())
    }

    fn build(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for i in &mut self.build.0 {
            let arge = i.cmd.iter().len();
            println!("\tRunning step {}", i.name);
            Command::new(i.cmd[0].clone())
                .args(&mut i.cmd[1..arge])
                .output()?;
        }
        Ok(())
    }

    fn install(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::DirBuilder::new().recursive(true).create(format!(
            "/mtos/pkgs/{}+{}{}{}+{}",
            self.name, self.version.0, self.version.1, self.version.2, self.sha256
        ))?;
        for i in &mut self.install.0 {
            let arge = i.cmd.iter().len();
            println!("\tRunning step {}", i.name);
            Command::new(i.cmd[0].clone())
                .env(
                    "INSTDIR",
                    Path::new("/mtos/pkgs").join(format!(
                        "{}+{}{}{}+{}",
                        self.name, self.version.0, self.version.1, self.version.2, self.sha256
                    )),
                )
                .args(&mut i.cmd[1..arge])
                .output()?;
        }
        Ok(())
    }

    fn remove(&self, pkg: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn query(&self, pkg: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
