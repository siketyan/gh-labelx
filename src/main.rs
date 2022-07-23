mod manifest;
mod terraform;
mod tf;

use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use gh_config::{Hosts, GITHUB_COM};
use tempdir::TempDir;
use uuid::Uuid;

use crate::manifest::Manifest;
use crate::terraform::Terraform;

#[derive(Debug, clap::Parser)]
struct ManifestArgs {
    /// Path to the manifest file.
    #[clap(short, long)]
    file: PathBuf,
}

#[derive(Debug, clap::Subcommand)]
enum Action {
    /// Dumps the generated Terraform file to stdout or the file.
    Dump {
        #[clap(flatten)]
        manifest: ManifestArgs,

        #[clap(long)]
        output: Option<PathBuf>,
    },
    /// Plans your changes to the labels.
    Plan {
        #[clap(flatten)]
        manifest: ManifestArgs,
    },
    /// Applies the manifest to labels on GitHub.
    Apply {
        #[clap(flatten)]
        manifest: ManifestArgs,
    },
}

#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    action: Action,

    /// Hostname of GitHub Enterprise Server.
    #[clap(short, long)]
    host: Option<String>,

    /// Repository owner or organisation.
    #[clap(short, long)]
    owner: String,

    /// Repository name.
    #[clap(short, long)]
    repo: String,
}

fn main() -> Result<()> {
    use clap::Parser;

    let cli: Cli = Cli::parse();
    let into_tf =
        |args: &ManifestArgs| Manifest::load(&args.file)?.generate_tf(&cli.owner, &cli.repo);

    let write_tf = |tf: &str| -> Result<Terraform> {
        let tf_dir = TempDir::new("gh-labelx")?
            .path()
            .join(Uuid::new_v4().to_string());

        fs::create_dir_all(&tf_dir)?;
        fs::write(tf_dir.join("main.tf"), tf)?;

        let hosts = Hosts::load()?;
        let host = hosts
            .get(cli.host.unwrap_or_else(|| GITHUB_COM.to_owned()).as_ref())
            .ok_or_else(|| anyhow!("GitHub host configuration not found"))?;

        Ok(Terraform::new(
            &tf_dir,
            [("GITHUB_TOKEN".into(), host.oauth_token.to_owned())],
        ))
    };

    match cli.action {
        Action::Dump { manifest, output } => {
            let tf = into_tf(&manifest)?;

            (|s: &str| -> Result<()> {
                match &output {
                    Some(path) => fs::write(path, s)?,
                    _ => print!("{}", s),
                };
                Ok(())
            })(&tf)?;
        }
        Action::Plan { manifest } => {
            let tf = into_tf(&manifest)?;
            let terraform = write_tf(&tf)?;

            terraform.init()?;
            terraform.plan()?;
        }
        Action::Apply { manifest } => {
            let tf = into_tf(&manifest)?;
            let terraform = write_tf(&tf)?;

            terraform.init()?;
            terraform.apply()?;
        }
    }

    Ok(())
}
