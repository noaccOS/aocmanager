use color_eyre::eyre;
use gleam::Gleam;
use include_dir::Dir;
use std::{path::Path, process::Command};
use zig::Zig;

mod gleam;
pub mod variant;
mod zig;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[repr(u8)]
#[serde(rename_all = "kebab-case")]
pub enum Template {
    Gleam(Gleam) = 0,
    Zig(Zig) = 1,
}

impl From<&str> for Template {
    fn from(value: &str) -> Self {
        match value {
            "gleam" => Self::Gleam(Gleam::default()),
            "zig" => Self::Zig(Zig::default()),
            _ => panic!("Unknown template"),
        }
    }
}

impl AoCTemplate for Template {
    fn template(&self) -> Dir<'static> {
        match self {
            Template::Gleam(gleam) => gleam.template(),
            Template::Zig(zig) => zig.template(),
        }
    }

    fn add_input(&self, day_root: &Path, input_contents: &str) -> eyre::Result<()> {
        match self {
            Template::Gleam(gleam) => gleam.add_input(day_root, input_contents),
            Template::Zig(zig) => zig.add_input(day_root, input_contents),
        }
    }

    fn run(&self, variant: variant::RunVariant) -> Command {
        match self {
            Template::Gleam(gleam) => gleam.run(variant),
            Template::Zig(zig) => zig.run(variant),
        }
    }

    fn run_samples(&self, variant: variant::RunVariant) -> Command {
        match self {
            Template::Gleam(gleam) => gleam.run_samples(variant),
            Template::Zig(zig) => zig.run_samples(variant),
        }
    }

    fn add_sample(
        &self,
        day_root: &Path,
        variant: variant::Variant,
        input: &str,
        result: &str,
    ) -> eyre::Result<()> {
        match self {
            Template::Gleam(gleam) => gleam.add_sample(day_root, variant, input, result),
            Template::Zig(zig) => zig.add_sample(day_root, variant, input, result),
        }
    }
}

pub trait AoCTemplate {
    fn template(&self) -> Dir<'static>;

    fn add_input(&self, day_root: &Path, input_contents: &str) -> eyre::Result<()>;
    fn run(&self, variant: variant::RunVariant) -> Command;
    fn run_samples(&self, variant: variant::RunVariant) -> Command;

    fn add_sample(
        &self,
        day_root: &Path,
        variant: variant::Variant,
        input: &str,
        result: &str,
    ) -> eyre::Result<()>;
}
