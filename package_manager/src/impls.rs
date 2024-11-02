use fetch_data::hash_download;
use log::{error, info, trace};
use regex::Regex;
#[cfg(not(target_os = "windows"))]
use rustix::fs::symlink;
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::fs::DirBuilder;
#[cfg(target_os = "windows")]
use std::os::windows::fs::symlink_file as symlink;
use std::{
    env::temp_dir,
    error::Error,
    fs::read_to_string,
    path::{Path, PathBuf},
    process::{exit, Command},
};
use traits::{Building, DependencyResolution, Filling};

/// Structure representing dependencies with name, category, version, and SHA256 checksum.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Deps {
    name: String,
    pub category: String,
    version: Vec<i32>,
    sha256: String,
}

/// Structure representing a build step with a name and a command vector.
#[derive(Serialize, Deserialize, Clone)]
struct Step {
    name: String,
    cmd: Vec<String>,
}

/// Macro to execute build steps.
macro_rules! step {
    ($var:expr) => {
        for i in &$var.0 {
            let (command, args) = i.cmd.split_first().unwrap();
            info!("\tRunning step {}", i.name);
            let mut cmd = Command::new(command);
            cmd.args(args);
            let output = cmd.output();

            match output {
                Ok(output) => {
                    info!("\tRunning command {} {:?}", command, args);
                    if output.status.success() {
                        trace!("{:#?}", output.stdout);
                    } else {
                        error!("{:#?}", output.stderr);
                        exit(1);
                    }
                }
                Err(e) => {
                    error!("{e:#?}");
                    exit(1)
                }
            }
        }
    };
}

/// Structure representing the preparation steps.
#[derive(Serialize, Deserialize, Clone)]
struct Prepare(Vec<Step>);

/// Structure representing the build steps.
#[derive(Serialize, Deserialize, Clone)]
struct Build(Vec<Step>);

/// Structure representing the installation steps.
#[derive(Serialize, Deserialize, Clone)]
struct Install(Vec<Step>);

/// Structure representing a file to fetch with name, file type, source URL, and SHA256 checksum.
#[derive(Serialize, Deserialize, Clone)]
pub struct Fetch {
    pub name: String,
    pub ft: String,
    pub src: String,
    pub sha256: String,
}

/// Structure representing a builder with name, category, version, SHA256 checksum, dependencies, downloads, prepare, build, and install steps.
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
    #[cfg(not(target_os = "windows"))]
    prepare: Prepare,
    #[cfg(target_os = "windows")]
    build: Option<Build>,
    #[cfg(not(target_os = "windows"))]
    build: Build,
    install: Install,
}

impl Builder {}

#[cfg(target_os = "windows")]
impl Default for Builder {
    fn default() -> Self {
        // Default implementation for Windows
        Self {
            name: String::new(),
            category: String::new(),
            version: (0, 0, 0),
            sha256: String::new(),
            dependencies: Some(vec![Deps {
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
            prepare: Some(Prepare(vec![Step {
                name: String::new(),
                cmd: vec![String::new()],
            }])),
            build: Some(Build(vec![Step {
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

#[cfg(not(target_os = "windows"))]
impl Default for Builder {
    fn default() -> Self {
        // Default implementation for non-Windows
        Self {
            name: String::new(),
            category: String::new(),
            version: (0, 0, 0),
            sha256: String::new(),
            dependencies: Some(vec![Deps {
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
    /// Fills the builder from a YAML file.
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
    /// Resolves dependencies.
    fn resolve(&mut self) -> Result<(), Box<dyn Error>> {
        self.prep()?;
        self.build()?;
        self.install()?;
        Ok(())
    }
}

impl Building for Builder {
    /// Prepares the build by resolving dependencies and downloading files.
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
            let path = Path::new(temp_dir().clone().as_path())
                .join(format!("{}.{}", i.name, i.ft));
            info!(
                "Downloading {}.{} from {} to {}",
                i.name,
                i.ft,
                i.src,
                path.display()
            );
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
        info!("Running pre-build steps");
        #[cfg(target_os = "windows")]
        if let Some(prep) = self.prepare.clone() {
            step!(prep);
        } else if cfg!(target_os = "windows") {
            info!("We are on Windows, we don't need to run prepare tasks");
        } else {
            info!("Everything okay, prepare step not present");
        }
        #[cfg(not(target_os = "windows"))]
        step!(self.prepare);
        Ok(())
    }

    /// Builds the package.
    fn build(&mut self) -> Result<(), Box<dyn Error>> {
        #[cfg(target_os = "windows")]
        if let Some(b) = self.build.clone() {
            step!(b);
        }
        #[cfg(not(target_os = "windows"))]
        step!(self.build);
        Ok(())
    }
    /// Installs the package.
    fn install(&mut self) -> Result<(), Box<dyn Error>> {
        DirBuilder::new().recursive(true).create(format!(
            "/mtos/pkgs/{}+{}.{}.{}+{}_{}",
            self.name,
            self.version.0,
            self.version.1,
            self.version.2,
            self.sha256,
            chrono::NaiveDate::default()
        ))?;
        for i in &mut self.install.0 {
            let arge = i.cmd.iter().len();
            trace!(target: "install",  "\tRunning step {}", i.name);
            std::env::set_var(
                "PATH",
                format!(
                    "/mtos/pkgs/{}+{}.{}.{}+{}_{}",
                    self.name,
                    self.version.0,
                    self.version.1,
                    self.version.2,
                    self.sha256,
                    chrono::NaiveDate::default()
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
            DirBuilder::new().recursive(true).create(&val)?;
            Command::new(i.cmd[0].clone())
                .env(key, val.clone())
                .args(&mut i.cmd[1..=arge])
                .output()?;
            info!(target: "install",  "\tSymlinking final directory to the one with the package name");
            symlink(val, Path::new("/mtos/pkgs").join(&self.name))?;
        }
        info!(target: "install", "DONE!");
        Ok(())
    }

    /// Removes the package.
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

    /// Queries package information.
    fn query(&self) -> Result<(), Box<dyn Error>> {
        info!("Name: {}/{}", self.category, self.name);
        info!("Version: {:#?}", self.version);
        info!("Dependencies: {:#?}", self.dependencies);
        info!("SHA256: {}", self.sha256);
        Ok(())
    }
}
