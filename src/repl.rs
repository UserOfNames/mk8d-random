use std::fs::DirEntry;
use std::path::PathBuf;

use anyhow::{self, Context};
use my_lib::continue_on_err;
use my_lib::io::input::update_input;
use rand::Rng;

use crate::courses::course::Course;
use crate::courses::course_list::CourseList;
use crate::templates::mk8d;

pub struct Repl {
    course_list: CourseList,
    input: String,
}

impl Repl {
    pub fn new(saves: Vec<DirEntry>, saves_dir: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let mut input: String = String::new();

        if saves.is_empty() {
            println!("No saves found. Creating a default (mk8d), will add more later, maybe");
            let course_list = mk8d::make_mk8d(saves_dir.into());
            course_list.dump_list()?;
            return Err(anyhow::anyhow!("Loading course list"));
        }

        println!("Enter the number of the save you want to use:");
        for (i, dir_entry) in saves.iter().enumerate() {
            println!("{}: {:?}", i, dir_entry.file_name());
        }
        update_input(&mut input, ":> ")?;

        let index: usize = input
            .trim()
            .parse()
            .context(format!("Parsing input '{}' into number", input))?;

        let selection = saves
            .get(index)
            .ok_or(anyhow::anyhow!("Out of range selection: {}", index))?;

        Ok(Self {
            course_list: CourseList::restore_save(selection.path())
                .context("Loading the saved course list")?,
            input: String::new(),
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        println!("Running. Enter 'help' for help information.");
        loop {
            continue_on_err!(update_input(&mut self.input, ":> "), "Error reading input");

            match self.input.trim().to_lowercase().as_ref() {
                "" => self.generate(),

                "q" | "quit" => {
                    continue_on_err!(self.quit());
                    println!("Quitting...");
                    break;
                }

                "help" => self.help(),

                "save" => continue_on_err!(self.save()),

                "remaining" | "re" | "ls" => self.remaining(),

                "used" => self.used(),

                "history" => self.history(),

                "reset" => continue_on_err!(self.reset()),

                "back" => self.back(),

                "forward" => self.forward(),

                "add" => continue_on_err!(self.add()),

                "remove" => continue_on_err!(self.remove()),

                "tier" => continue_on_err!(self.tier()),

                _ => eprintln!("Unrecognized command."),
            }
        }

        Ok(())
    }

    fn generate(&mut self) {
        let Some(course) = self.course_list.get_random() else {
            println!("The course list is empty. Resetting.");
            self.course_list.reset();
            return;
        };

        println!("{}", course);
        self.course_list.remove(course.clone());
    }

    fn quit(&mut self) -> anyhow::Result<()> {
        println!("Save changes before quitting? (Y/N): ");
        update_input(&mut self.input, ":> ").context("Reading input")?;
        match self.input.trim().to_lowercase().as_ref() {
            "y" => {
                self.course_list.dump_list().context("Saving list")?;
                println!("Saved successfully.");
            }

            "n" => {}

            _ => {
                return Err(anyhow::anyhow!("Must select Y or N"));
            }
        }

        Ok(())
    }

    fn save(&self) -> anyhow::Result<()> {
        self.course_list.dump_list().context("Saving list")?;
        println!("Saved successfully.");
        Ok(())
    }

    fn remaining(&self) {
        let current = self.course_list.get_current();
        if current.is_empty() {
            println!("The course list is empty.");
            return;
        }

        let strings: Vec<String> = current.iter().map(|c| c.to_string()).collect();
        println!("{}", strings.join("\n"));
        println!("There are {} courses in the list.", current.len())
    }

    fn used(&self) {
        let removed = self.course_list.get_removed();
        if removed.is_empty() {
            println!("No courses have been used.");
            return;
        }

        let strings: Vec<String> = removed.iter().map(|c| c.to_string()).collect();
        println!("{}", strings.join("\n"));
        println!("{} courses have been used.", removed.len())
    }

    fn history(&self) {
        println!("{}", self.course_list.get_history());
    }

    fn reset(&mut self) -> anyhow::Result<()> {
        update_input(&mut self.input, "Are you sure? (capital 'Y' to confirm): ")
            .context("Reading input")?;

        match self.input.trim() {
            "Y" => {
                self.course_list.reset();
                println!("Course list reset.");
            }

            _ => {
                println!("Cancelled reset.");
            }
        }

        Ok(())
    }

    fn back(&mut self) {
        if self.course_list.roll_back().is_err() {
            eprintln!("Error rolling back: No history found");
        }
    }

    fn forward(&mut self) {
        if self.course_list.roll_forward().is_err() {
            eprintln!("Error rolling forward: No future found");
        }
    }

    fn add(&mut self) -> anyhow::Result<()> {
        update_input(&mut self.input, "Search courses: ").context("Reading input")?;

        let results: Vec<&Course> = self.course_list.search_removed(&self.input).collect();

        for (i, c) in results.iter().enumerate() {
            println!("{}: {}", i + 1, c);
        }

        update_input(&mut self.input, "Select the number of the course to add: ")
            .context("Reading input")?;

        let index: usize = self
            .input
            .parse()
            .context(format!("Parsing input '{}' into number", self.input))?;

        let &selection = results
            .get(index.wrapping_sub(1))
            .ok_or(anyhow::anyhow!("Selecting course: Out of bounds selection"))?;

        self.course_list.add(selection.clone());
        Ok(())
    }

    fn remove(&mut self) -> anyhow::Result<()> {
        update_input(&mut self.input, "Search courses: ").context("Reading input")?;

        let results: Vec<&Course> = self.course_list.search_current(&self.input).collect();

        for (i, c) in results.iter().enumerate() {
            println!("{}: {}", i + 1, c);
        }

        update_input(
            &mut self.input,
            "Select the number of the course to remove: ",
        )
        .context("Reading input")?;

        let index: usize = self
            .input
            .parse()
            .context(format!("Parsing input '{}' into number", self.input))?;

        let &selection = results
            .get(index.wrapping_sub(1))
            .ok_or(anyhow::anyhow!("Selecting course: Out of bounds selection"))?;

        self.course_list.remove(selection.clone());
        Ok(())
    }

    fn tier(&mut self) -> anyhow::Result<()> {
        update_input(&mut self.input, "Enter the size of the prix: ").context("Reading input")?;
        let size: usize = self
            .input
            .parse()
            .context(format!("Parsing input '{}' into number", self.input))?;

        let tiered_courses: Vec<Course> = match self.course_list.get_random_by_chunks(size) {
            Ok(c) => c.collect(),
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "Error: Could not divide courses into tiers.\n\
                    This probably means the course list cannot be evenly divided by the given prix size.\n\
                    Consider adding or removing courses until the list is evenly divisible."
                ));
            }
        };

        if run_tiered_list(tiered_courses.clone()) {
            for course in tiered_courses {
                self.course_list.remove(course);
            }
        }

        Ok(())
    }

    fn help(&self) {
        println!("---------------------------------------------------");

        println!(
            "Blank input: Generate and remove a random course.\n\
            q, quit:      Exit.\n\
            help:         Show this help text.\n\
            save:         Save the list."
        );

        println!(
            "Information:\n\
            remaining, re, ls: List remaining courses.\n\
            used:              List used courses.\n\
            history:           Show history.\n"
        );

        println!(
            "List editing:\n\
            reset:           Reset the course list and delete all history.\n\
            back:            Roll back in history.\n\
            forward:         Roll forward in history.\n\
            add:             Add a previously removed course.\n\
            remove, rm, pop: Remove a currently active course.\n"
        );

        println!(
            "Special:\n\
            tier: Generate a tiered sub-list."
        );

        println!("---------------------------------------------------");
    }
}

fn run_tiered_list(mut list: Vec<Course>) -> bool {
    let mut rng = rand::rng();
    println!("Entered tiered list. Type 'back' to return without removing the selected courses.");

    let mut input: String = String::new();
    while !list.is_empty() {
        continue_on_err!(update_input(&mut input, ":> "), "Error reading input");

        match input.trim().to_lowercase().as_ref() {
            "" => {
                let index = rng.random_range(0..list.len());
                println!("{}", list[index]);
                list.remove(index);
            }

            "back" => {
                println!("Returning to main list...");
                return false;
            }

            "ls" => {
                let strings: Vec<String> = list.iter().map(|c| c.to_string()).collect();
                println!("{}", strings.join("\n"));
            }

            _ => {
                println!("Unrecognized command.");
            }
        }
    }

    println!("Tiered list exhausted. Returning to main list...");
    true
}
