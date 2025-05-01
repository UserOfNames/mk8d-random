use lists::mk8d::make_mk8d;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File, create_dir_all};
use std::io::{self, Write};
use std::path::PathBuf;

mod lists;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Course {
    name: String,
    coord: u32,
    rank: u8,
}

impl Course {
    fn new(rank: u8, coord: u32, name: &str) -> Self {
        Course {
            name: name.to_string(),
            coord,
            rank,
        }
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) {}", self.coord, self.rank, self.name)
    }
}

impl PartialEq for Course {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Course {}

impl PartialOrd for Course {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl Ord for Course {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

// An action the user takes, e.g. adding or removing a course
enum Action {
    Add(Course),
    Remove(Course),
}

struct CourseList {
    list: Vec<Course>,
    file: PathBuf,
    history: Vec<Action>,
}

impl CourseList {
    fn new(path: PathBuf) -> CourseList {
        CourseList {
            list: Vec::new(),
            file: path.into(),
            history: Vec::new(),
        }
    }

    fn dump_list(&self) -> io::Result<()> {
        let par = self.file.parent().unwrap();
        if !par.exists() {
            create_dir_all(par)?;
        }

        let mut file = File::create(&self.file)?;
        let data = serde_json::to_string_pretty(&self.list)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn restore_list(&mut self) -> io::Result<()> {
        let data = fs::read_to_string(&self.file)?;
        self.list = serde_json::from_str(&data)?;
        Ok(())
    }

    fn push(&mut self, course: Course) {
        self.insert_in_order(course);
    }

    fn remove(&mut self, course: Course) {
        match self.list.binary_search(&course) {
            Ok(index) => {
                self.list.remove(index);
            }

            Err(_) => eprintln!(
                "ERROR While removing course: Course {} not found in list",
                course
            ),
        };
    }

    fn search_list(&self, searched: &str) -> Vec<Course> {
        let mut res: Vec<Course> = Vec::new();

        for c in &self.list {
            if c.name.contains(searched) {
                res.push(c.clone());
            }
        }

        res.sort();
        res
    }

    fn generate(&mut self) -> Option<()> {
        if self.list.len() == 0 {
            return None;
        }

        let to_pop: usize = rand::rng().random_range(0..self.list.len());
        println!("{}", self.list[to_pop]);
        self.list.remove(to_pop);
        Some(())
    }

    fn insert_in_order(&mut self, course: Course) {
        match self.list.binary_search(&course) {
            Ok(_) => eprintln!(
                "ERROR: While adding course: Duplicate course entry {}",
                course
            ),
            Err(index) => self.list.insert(index, course),
        };
    }
}

impl Display for CourseList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let strings: Vec<String> = self.list.iter().map(|c| c.to_string()).collect();
        write!(f, "{}", strings.join("\n"))
    }
}

fn main() {
    let path: PathBuf = "../saves/mk8d.json".into();
    let mut courses = CourseList::new(path);

    match courses.restore_list() {
        Ok(_)  => (),
        Err(_) => {
            println!("Did not find a save file, creating one...");
            courses.list = make_mk8d();
            courses.dump_list().expect("ERROR While creating initial course list");
        },
    }

    println!("There are {} courses in the list.", courses.list.len());
    println!("{}", courses);
}
