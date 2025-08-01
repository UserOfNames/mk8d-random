mod courses;
mod repl;
mod tui;

use std::fs::{DirEntry, create_dir};
use std::path::PathBuf;
use std::sync::LazyLock;

use anyhow;
use clap::{self, Parser, ValueEnum};
use dirs;

use repl::Repl;
use tui::tui::Tui;

const MK8D_DEFAULT_SAVE_JSON: &str = include_str!("../data/mk8d_default_save.json");

static SAVES_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut saves_dir = dirs::data_dir().expect("Could not find data directory");

    saves_dir.push("mk8d-random");
    if !saves_dir
        .try_exists()
        .expect("Could not identify whether the saves directoy exists")
    {
        create_dir(&saves_dir).expect("Saves directory not found and could not be created");
    }

    saves_dir
});

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

    let saves: Vec<DirEntry> = std::fs::read_dir(&*SAVES_DIR)?.collect::<Result<_, _>>()?;

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
