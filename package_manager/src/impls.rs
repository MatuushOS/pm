use fetch_data::hash_download;
use log::{error, info, trace};
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::fs::read_to_string;
use std::{
    env::temp_dir,
    error::Error,
    path::{Path, PathBuf},
    process::{exit, Command},
};
use traits::{Building, DependencyResolution, Filling};
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Deps {
    name: String,
    category: String,
    version: Vec<i32>,
    sha256: String,
}
#[derive(Serialize, Deserialize, Clone)]
struct Step {
    name: String,
    cmd: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone)]
struct Prepare(Vec<Step>);
#[derive(Serialize, Deserialize, Clone)]
struct Build(Vec<Step>);
#[derive(Serialize, Deserialize, Clone)]
struct Install(Vec<Step>);
#[derive(Serialize, Deserialize, Clone)]
pub struct Fetch {
    name: String,
    ft: String,
    pub src: String,
    pub sha256: String,
}
#[derive(Serialize, Deserialize, Clone)]
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
}
impl Default for Builder {
    fn default() -> Self {
        Self {
            name: String::new(),
            category: String::new(),
            version: (0, 0, 0),
            sha256: String::new(),
            dependencies: vec![Deps {
                name: String::new(),
                category: String::new(),
                version: vec![0],
                sha256: String::new(),
            }],
            dl: vec![Fetch {
                name: String::new(),
                ft: String::new(),
                src: String::new(),
                sha256: String::new(),
            }],
            prepare: Prepare(vec![Step {
                name: String::new(),
                cmd: vec![String::new()],
            }]),
            build: Build(vec![Step {
                name: String::new(),
                cmd: vec![String::new()],
            }]),
            install: Install(vec![Step {
                name: String::new(),
                cmd: vec![String::new()],
            }]),
        }
    }
}
impl Filling for Builder {
    fn fill(&mut self, f: PathBuf) -> Result<(), Box<dyn Error>> {
        let f = read_to_string(f.as_path()).unwrap();
        let cfg: Self = from_str(&f).unwrap();
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
        self.prep()?;
        self.build()?;
        self.install()?;
        Ok(())
    }
}
impl Building for Builder {
    /// Mainly dependency resolution and downloads
    fn prep(&self) -> Result<(), Box<dyn std::error::Error>> {
        trace!(target: "prepare", "Making package {}", self.name);
        if self.dependencies.is_empty() {
            info!("Nothing to resolve");
        } else {
            for i in &self.dependencies {
                self.clone()
                    .fill(Path::new(&i.category).join(&i.name).with_extension("yml"))?;
                self.clone().resolve()?;
            }
        }
        for i in &self.dl.clone() {
            info!("Downloading {}.{} to {}", i.name, i.ft, i.src);
            let path = Path::new(temp_dir().clone().as_path()).join(format!("{}{}", i.name, i.ft));
            if hash_download(i.clone().src, &path)? != i.sha256 {
                std::fs::remove_file(path)?;
                error!("FILE IS UNSAFE TO USE! STOPPING THE OPENRATION NOW!!!");
                exit(1);
            } else {
                unarchive::extract(path);
                std::env::set_current_dir("src")?;
            }
        }
        println!("Running pre-build steps");
        self.prepare.0.iter().for_each(|i| {
            let arge = i.cmd.len();
            println!("\tRunning step {}", i.name);
            match Command::new(i.cmd[0].clone())
                .args(&i.cmd[1..arge])
                .output()
            {
                Ok(ok) => println!("{:#?}", ok.stdout.iter()),
                Err(e) => {
                    eprintln!("{e:#?}");
                    exit(1)
                }
            }
        });
        Ok(())
    }

    fn build(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for i in &mut self.build.0 {
            let arge = i.cmd.iter().len();
            trace!("\tRunning step {}", i.name);
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
            trace!(target: "install",  "\tRunning step {}", i.name);
            let key = "INSTDIR";
            let val = Path::new("/mtos/pkgs").join(format!(
                "{}+{}.{}.{}+{}",
                self.name, self.version.0, self.version.1, self.version.2, self.sha256
            ));
            std::fs::DirBuilder::new().recursive(true).create(&val)?;
            Command::new(i.cmd[0].clone())
                .env(key, val)
                .args(&mut i.cmd[1..arge])
                .output()?;
        }
        std::env::set_var(
            "PATH",
            format!(
                "/mtos/pkgs/{}+{}.{}.{}+{}",
                self.name, self.version.0, self.version.1, self.version.2, self.sha256
            ),
        );
        info!(target: "install", "DONE!");
        Ok(())
    }

    fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::remove_dir_all(format!(
            "/mtos/pkgs/{}+{}.{}.{}+{}",
            self.name, self.version.0, self.version.1, self.version.2, self.sha256
        ))?;
        Ok(())
    }

    fn query(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Name: {}/{}", self.category, self.name);
        info!("Version: {:#?}", self.version);
        info!("Dependencies: {:#?}", self.dependencies);
        info!("SHA256: {}", self.sha256);
        Ok(())
    }
}
