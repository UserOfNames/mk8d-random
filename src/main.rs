mod courses;
mod lists;
mod repl;
mod tui;

use std::fs::{DirEntry, create_dir};
use std::io;

use clap::ValueEnum;
use clap::{self, Parser};
use dirs;

use tui::app::TUI;
use lists::mk8d;
use repl::repl;

#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    REPL,
    TUI,
}

#[derive(Debug, Parser)]
#[command(name = "mk8d-random", author = "UserOfNames", version, about)]
struct Args {
    #[arg(short, value_enum, default_value_t = Mode::REPL, help = "Mode for the UI")]
    mode: Mode,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let Some(mut saves_dir) = dirs::data_dir() else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not find data directory",
        ));
    };

    saves_dir.push("mk8d-random");
    if !saves_dir.try_exists()? {
        create_dir(&saves_dir)?;
    }

    let saves: Vec<DirEntry> = std::fs::read_dir(&saves_dir)?.collect::<Result<Vec<_>, _>>()?;

    if saves.is_empty() {
        println!("No saves found. Creating a default (mk8d), will add more later, maybe");
        let course_list = mk8d::make_mk8d(saves_dir);
        course_list.dump_list()?;
        return Ok(());
    }

    match args.mode {
        Mode::TUI => {
            let mut terminal = ratatui::init();
            let app_result = TUI::new().run(&mut terminal);
            ratatui::restore();
            app_result
        }

        Mode::REPL => repl(saves),
    }
}
