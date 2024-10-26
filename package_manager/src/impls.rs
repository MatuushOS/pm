use fetch_data::hash_download;
use log::{error, info, trace};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::{
    fs::read_to_string,
    env::temp_dir,
    error::Error,
    path::{Path, PathBuf},
    process::{exit, Command}
};
use traits::{Building, DependencyResolution, Filling};
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Deps {
    name: String,
    pub category: String,
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
    pub name: String,
    pub ft: String,
    pub src: String,
    pub sha256: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Builder {
    name: String,
    pub category: String,
    version: (i32, i32, i32),
    sha256: String,
    dependencies: Option<Vec<Deps>>,
    pub dl: Vec<Fetch>,
    #[cfg(target_os = "windows")]
    prepare: Option<Prepare>,
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    prepare: Prepare,
    #[cfg(target_os = "windows")]
    build: Option<Build>,
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    build: Build,
    install: Install,
}

impl Builder {
    pub(crate) fn write(self, path: &str) -> Result<(), Box<dyn Error>> {
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
            dependencies: Option::from(vec![Deps {
                name: String::new(),
                category: String::new(),
                version: vec![0],
                sha256: String::new(),
            }]),
            dl: vec![Fetch {
                name: String::new(),
                ft: String::new(),
                src: String::new(),
                sha256: String::new(),
            }],
            prepare: Option::from(Prepare(vec![Step {
                name: String::new(),
                cmd: vec![String::new()],
            }])),
            build: Option::from(Build(vec![Step {
                name: String::new(),
                cmd: vec![String::new()],
            }])),
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
    fn prep(&self) -> Result<(), Box<dyn Error>> {
        trace!(target: "prepare", "Making package {}", self.name);
        if self.dependencies.is_none() {
            info!("Nothing to resolve");
        } else {
            for i in &self.dependencies.clone().unwrap() {
                self.clone().fill(
                    Path::new(&i.category).join(&i.name).with_extension("yml"),
                )?;
                self.clone().resolve()?;
            }
        }
        for i in &self.dl.clone() {
            info!("Downloading {}.{} from {}", i.name, i.ft, i.src);
            let path = Path::new(temp_dir().clone().as_path())
                .join(format!("{}{}", i.name, i.ft));
            if hash_download(i.clone().src, &path)? != i.sha256 {
                std::fs::remove_file(path)?;
                error!("FILE IS UNSAFE TO USE! STOPPING THE OPENRATION NOW!!!");
                exit(1);
            } else if path.extension()
                == Some(Regex::new(r".tar.*").unwrap().as_str().as_ref())
            {
                unarchive::extract(path);
                std::env::set_current_dir("src")?;
            } else {
                Command::new(format!("./{}", path.to_str().unwrap()));
            }
        }
        println!("Running pre-build steps");
        if let Some(prep) = self.prepare.clone() {
            prep.clone().0.iter().for_each(|i| {
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
        } else if cfg!(target_os = "windows") {
            info!("We are on Windows, we don't need to run prepare tasks");
        } else {
            info!("Everything okay, prepare step not present");
        }
        Ok(())
    }

    fn build(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(b) = self.build.clone() {
            for i in &mut b.clone().0 {
                let arge = i.cmd.iter().len();
                trace!("\tRunning step {}", i.name);
                Command::new(i.cmd[0].clone())
                    .args(&mut i.cmd[1..arge])
                    .output()?;
            }
        } else if cfg!(target_os = "windows") {
            info!("We are on Windows, we don't need to run build tasks");
        } else {
            info!("Everything okay, build step not present");
        }
        Ok(())
    }

    fn install(&mut self) -> Result<(), Box<dyn Error>> {
        std::fs::DirBuilder::new().recursive(true).create(format!(
            "/mtos/pkgs/{}+{}{}{}+{}",
            self.name,
            self.version.0,
            self.version.1,
            self.version.2,
            self.sha256
        ))?;
        for i in &mut self.install.0 {
            let arge = i.cmd.iter().len();
            trace!(target: "install",  "\tRunning step {}", i.name);
            std::env::set_var(
                "PATH",
                format!(
                    "/mtos/pkgs/{}+{}.{}.{}+{}",
                    self.name,
                    self.version.0,
                    self.version.1,
                    self.version.2,
                    self.sha256
                ),
            );
            let key = "INSTDIR";
            let val = Path::new("/mtos/pkgs").join(format!(
                "{}+{}.{}.{}+{}",
                self.name,
                self.version.0,
                self.version.1,
                self.version.2,
                self.sha256
            ));
            std::fs::DirBuilder::new().recursive(true).create(&val)?;
            Command::new(i.cmd[0].clone())
                .env(key, val)
                .args(&mut i.cmd[1..arge])
                .output()?;
        }
        info!(target: "install", "DONE!");
        Ok(())
    }

    fn remove(&self) -> Result<(), Box<dyn Error>> {
        std::fs::remove_dir_all(format!(
            "/mtos/pkgs/{}+{}.{}.{}+{}",
            self.name,
            self.version.0,
            self.version.1,
            self.version.2,
            self.sha256
        ))?;
        Ok(())
    }

    fn query(&self) -> Result<(), Box<dyn Error>> {
        info!("Name: {}/{}", self.category, self.name);
        info!("Version: {:#?}", self.version);
        info!("Dependencies: {:#?}", self.dependencies);
        info!("SHA256: {}", self.sha256);
        Ok(())
    }
}
