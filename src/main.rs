mod repl;
mod tui;

use std::fs::{DirEntry, create_dir};

use anyhow::Context;
use clap::{self, Parser, ValueEnum};

use mk8d_random::SAVES_DIR;

use repl::Repl;
use tui::tui::Tui;

const MK8D_DEFAULT_SAVE_JSON: &str = include_str!("../data/mk8d_default_save.json");

#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    Repl,
    Tui,
}

#[derive(Debug, Parser)]
#[command(name = "mk8d-random", author = "UserOfNames", version, about)]
struct Args {
    #[arg(short, value_enum, default_value_t = Mode::Repl, help = "Mode for the UI")]
    mode: Mode,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if !SAVES_DIR.try_exists().context("Accessing save directory")? {
        create_dir(&*SAVES_DIR).context("Creating save directory")?;
    }

    let saves: Result<Vec<DirEntry>, _> = std::fs::read_dir(&*SAVES_DIR)
        .context(format!("Accessing save directory {:?}", SAVES_DIR))?
        .collect();
    let saves: Vec<DirEntry> = saves?;

    match args.mode {
        Mode::Tui => {
            todo!("Implement TUI");
            let mut terminal = ratatui::init();
            let app_result = Tui::new().run(&mut terminal);
            ratatui::restore();
            Ok(app_result?)
        }

        Mode::Repl => {
            let mut repl = Repl::new(saves)?;
            Ok(repl.run()?)
        }
    }
}
