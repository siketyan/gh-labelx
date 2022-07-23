use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Label {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

pub type Labels = Vec<Label>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manifest {
    pub labels: Labels,
}

impl Manifest {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(serde_yaml::from_slice(fs::read(path)?.as_ref())?)
    }
}
