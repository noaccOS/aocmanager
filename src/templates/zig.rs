use color_eyre::eyre;
use std::{fs, path::Path, process::Command};

use include_dir::include_dir;

use crate::templates::variant::RunVariant;

use super::AoCTemplate;

const TEMPLATE_DIR: include_dir::Dir<'static> = include_dir!("./templates/zig");

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Zig {}

impl AoCTemplate for Zig {
    fn template(&self) -> include_dir::Dir<'static> {
        TEMPLATE_DIR
    }

    fn add_input(&self, day_root: &Path, input_contents: &str) -> eyre::Result<()> {
        let assets_dir = day_root.join("assets");
        fs::create_dir_all(&assets_dir)?;
        fs::write(assets_dir.join("input"), input_contents)?;
        Ok(())
    }

    fn run(&self, variant: super::variant::RunVariant) -> Command {
        match variant {
            RunVariant::A => {
                let mut command = Command::new("zig");
                command.args(["build", "run", "a"]);
                command
            }
            RunVariant::B => {
                let mut command = Command::new("zig");
                command.args(["build", "run", "b"]);
                command
            }
            RunVariant::Both => {
                let mut command = Command::new("zig");
                command.args(["build", "run"]);
                command
            }
        }
    }

    fn run_samples(&self, variant: super::variant::RunVariant) -> Command {
        match variant {
            RunVariant::A => {
                let mut command = Command::new("zig");
                command.args(["build", "run", "samples-a"]);
                command
            }
            RunVariant::B => {
                let mut command = Command::new("zig");
                command.args(["build", "run", "samples-b"]);
                command
            }
            RunVariant::Both => {
                let mut command = Command::new("zig");
                command.args(["build", "run", "samples"]);
                command
            }
        }
    }

    fn add_sample(
        &self,
        day_root: &Path,
        variant: super::variant::Variant,
        input: &str,
        result: &str,
    ) -> eyre::Result<()> {
        let variant_str = match variant {
            crate::templates::variant::Variant::A => "a",
            crate::templates::variant::Variant::B => "b",
        };
        let sample_variant_subpath = format!("assets/samples/{variant_str}/");
        let sample_variant_subpath = day_root.join(sample_variant_subpath);
        fs::create_dir_all(&sample_variant_subpath)?;
        let biggest_existing_sample = fs::read_dir(&sample_variant_subpath)?
            .filter_map(|read_dir| read_dir.ok()?.file_name().into_string().ok())
            .filter_map(|file_name| file_name.parse::<u32>().ok())
            .max()
            .unwrap_or(0);
        let new_sample = format!("{:02}", biggest_existing_sample + 1);
        let sample_dir = sample_variant_subpath.join(new_sample);
        fs::create_dir(&sample_dir)?;
        fs::write(sample_dir.join("input"), input)?;
        fs::write(sample_dir.join("result"), result)?;
        Ok(())
    }
}
