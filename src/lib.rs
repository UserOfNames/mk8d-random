#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::path::PathBuf;
use std::sync::LazyLock;

pub mod courses;

pub static SAVES_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut saves_dir = dirs::data_dir().expect("Could not find data directory");
    saves_dir.push("mk8d-random");
    saves_dir
});
