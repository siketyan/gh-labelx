use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Result;

pub struct Terraform {
    path: PathBuf,
    envs: HashMap<String, String>,
}

impl Terraform {
    pub fn new<P, E>(path: P, envs: E) -> Self
    where
        P: AsRef<Path>,
        E: Into<HashMap<String, String>>,
    {
        Self {
            path: path.as_ref().to_path_buf(),
            envs: envs.into(),
        }
    }

    pub fn init(&self) -> Result<()> {
        self.run("init")
    }

    pub fn plan(&self) -> Result<()> {
        self.run("plan")
    }

    pub fn apply(&self) -> Result<()> {
        self.run("apply")
    }

    fn run(&self, action: &str) -> Result<()> {
        self.command().arg(action).spawn()?.wait()?;

        Ok(())
    }

    fn command(&self) -> Command {
        let mut command = Command::new("terraform");
        command.current_dir(&self.path).envs(&self.envs);
        command
    }
}
