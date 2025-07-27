mod courses;
mod lists;
mod repl;
mod tui;

use std::fs::{DirEntry, create_dir};

use anyhow::{self, bail};
use clap::ValueEnum;
use clap::{self, Parser};
use dirs;

use repl::run_repl;

use tui::app::TUI;

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

    let Some(mut saves_dir) = dirs::data_dir() else {
        bail!("Could not find data directory");
    };

    saves_dir.push("mk8d-random");
    if !saves_dir.try_exists()? {
        create_dir(&saves_dir)?;
    }

    let saves: Vec<DirEntry> = std::fs::read_dir(&saves_dir)?.collect::<Result<_, _>>()?;

    match args.mode {
        Mode::Tui => {
            let mut terminal = ratatui::init();
            let app_result = TUI::new().run(&mut terminal);
            ratatui::restore();
            Ok(app_result?)
        }

        Mode::Repl => Ok(run_repl(saves, saves_dir)?),
    }
}
