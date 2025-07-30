mod courses;
mod repl;
mod templates;
mod tui;

use std::fs::{DirEntry, create_dir};

use anyhow::{self, bail};
use clap::ValueEnum;
use clap::{self, Parser};
use dirs;

use repl::Repl;
use tui::tui::Tui;

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
            todo!("Implement TUI");
            let mut terminal = ratatui::init();
            let app_result = Tui::new().run(&mut terminal);
            ratatui::restore();
            Ok(app_result?)
        }

        Mode::Repl => {
            let mut repl = Repl::new(saves, saves_dir)?;
            Ok(repl.run()?)
        }
    }
}
