//! Library for course manager frontends.

#![warn(missing_docs, clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::path::PathBuf;
use std::sync::LazyLock;

pub mod courses;

/// Path to the directory holding serialized `CourseList`s, called 'saves.'
/// Because each platform has its own data directory, this can only be determined at runtime. As
/// such, it is unsuitable for storage inside a save. Instead, we determine it on program startup
/// and place it in a lazily initialized static.
pub static SAVES_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut saves_dir = dirs::data_dir().expect("Could not find data directory");
    saves_dir.push("mk8d-random");
    saves_dir
});
