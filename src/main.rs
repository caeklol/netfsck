use clap::Parser;
use netfsck::runner;
use std::{fs, path::Path};

#[derive(Parser)]
struct Cli {
    path: String,

    #[clap(default_value_t = 30000)]
    tape_size: usize
}

fn main() -> Result<(), color_eyre::Report> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let file_path = fs::canonicalize(Path::new(&cli.path))?;
    let bf_code = fs::read_to_string(file_path)?;

    let mut env = runner::Environment::new(cli.tape_size);
    env.evaluate(&bf_code)?;

    Ok(())
}
