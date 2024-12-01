use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use chrono::Datelike;
use clap::Parser;
use clap::Subcommand;
use color_eyre::eyre;
use color_eyre::eyre::bail;
use color_eyre::eyre::Context;
use color_eyre::eyre::OptionExt;

mod configuration;
use configuration::Configuration;
use configuration::DayConfiguration;
mod templates;
use templates::variant::RunVariant;
use templates::variant::Variant;
use templates::AoCTemplate;
use templates::Template;
#[derive(Parser, Debug)]
#[command(propagate_version = true)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Instantiate a template for the new day
    New {
        /// The day to create the project for
        #[arg(short, long, default_value = chrono::Local::now().day().to_string())]
        day: u8,
        /// The template to use
        #[arg(short, long, value_enum)]
        template: Template,
    },
    /// Run the existing day solution
    Run {
        /// The day of the project
        #[arg(short, long, default_value = default_day())]
        day: u8,
        /// which part of the day to run
        #[arg(short, long, default_value_t = RunVariant::Both, value_enum)]
        variant: RunVariant,
    },
    /// Test the solution on the samples
    RunSamples {
        /// The day of the project
        #[arg(short, long, default_value = default_day())]
        day: u8,
        /// which part of the day to run
        #[arg(short, long, default_value_t = RunVariant::Both, value_enum)]
        variant: RunVariant,
    },
    /// Add a sample to the current day.
    /// The input is taken from the clipboard, while the result is taken from stdin
    AddSample {
        /// The day of the project
        #[arg(short, long, default_value = default_day())]
        day: u8,
        /// Which part the sample belongs to
        #[arg(short, long, value_enum)]
        variant: Variant,
    },
    /// Initialize a new AoC project
    Init {
        /// The directory to use as root
        #[arg(default_value = ".")]
        root: PathBuf,
    },
}

fn default_day_logic() -> eyre::Result<u32> {
    let root = find_aoc_root()?.canonicalize()?;
    let cwd = std::env::current_dir()?.canonicalize()?;

    let days_directory = root.join("days");

    let day = cwd.strip_prefix(days_directory)?;
    let day = day.to_str().ok_or_eyre("not a valid utf8 directory name")?;
    let day: u32 = day.parse()?;

    Ok(day)
}

fn default_day() -> String {
    let today = chrono::Local::now().day();
    default_day_logic().unwrap_or(today).to_string()
}

fn find_aoc_root() -> eyre::Result<PathBuf> {
    let mut root = env::current_dir()?;
    while !root.join(".aoc").exists() {
        if !root.pop() {
            bail!("could not go to parent");
        }
    }

    Ok(root)
}

fn day_root(aoc_root: &Path, day: u8) -> PathBuf {
    aoc_root.join(format!("days/{day:02}"))
}

fn create_new_day(day: u8, template: Template) -> eyre::Result<()> {
    let root = find_aoc_root()?;
    let day_root = day_root(&root, day);
    fs::create_dir_all(&day_root)?;

    template.template().extract(&day_root)?;

    let input_contents = cli_clipboard::get_contents().unwrap();
    template.add_input(&day_root, &input_contents)?;

    let day_config = DayConfiguration { day, template };
    let mut config = Configuration::read(&root)?;
    config.days.push(day_config);
    config.write(&root)?;

    Ok(())
}

fn run_day(day: u8, variant: RunVariant) -> eyre::Result<()> {
    let root = find_aoc_root()?;
    let day_root = day_root(&root, day);
    std::env::set_current_dir(day_root)?;
    let day_config = Configuration::read(&root)?
        .for_day(day)
        .expect("day {?:day} not initialized");

    day_config.template.run(variant).spawn()?;
    Ok(())
}

fn run_samples(day: u8, variant: RunVariant) -> eyre::Result<()> {
    let root = find_aoc_root()?;
    let day_root = day_root(&root, day);
    std::env::set_current_dir(day_root)?;
    let day_config = Configuration::read(&root)?
        .for_day(day)
        .expect("day {?:day} not initialized");

    day_config.template.run_samples(variant).spawn()?;
    Ok(())
}

fn init_aoc(root: PathBuf) -> eyre::Result<()> {
    fs::create_dir_all(root.join(".aoc"))?;

    Configuration::default().write(&root)?;

    Ok(())
}

fn main() -> eyre::Result<()> {
    let args = Args::parse();
    color_eyre::install()?;
    // dbg!(args);
    match args.command {
        Commands::New { day, template } => {
            create_new_day(day, template).wrap_err("creating new day failed")
        }
        Commands::Run { day, variant } => run_day(day, variant).wrap_err("running day failed"),
        Commands::Init { root } => init_aoc(root).wrap_err("create dir failed"),
        Commands::RunSamples { day, variant } => {
            run_samples(day, variant).wrap_err("running samples failed")
        }
        Commands::AddSample { day, variant } => {
            add_sample(day, variant).wrap_err("adding sample failed")
        }
    }
}

fn add_sample(day: u8, variant: Variant) -> eyre::Result<()> {
    let root = find_aoc_root()?;
    let day_root = day_root(&root, day);
    std::env::set_current_dir(&day_root)?;
    let day_config = Configuration::read(&root)?
        .for_day(day)
        .expect("day {?:day} not initialized");

    let input = cli_clipboard::get_contents().unwrap();
    let mut result = String::new();
    std::io::stdin().read_to_string(&mut result)?;

    day_config
        .template
        .add_sample(&day_root, variant, &input, &result)?;

    Ok(())
}
