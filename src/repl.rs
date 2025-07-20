use std::fs::DirEntry;
use std::io;

use crate::courses::course::Course;
use crate::courses::course_list::CourseList;

use mk8d_random::continue_on_err;
use mk8d_random::utils::get_input;
use rand::Rng;

pub fn repl(saves: Vec<DirEntry>) -> io::Result<()> {
    let mut input: String; // Used for all user input
    let mut index: usize;  // Used for all indexes

    println!("Enter the number of the save you want to use:");
    for (i, de) in saves.iter().enumerate() {
        println!("{}: {:?}", i, de.file_name());
    }
    input = get_input(":> ")?;

    index = match input.trim().parse() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            // TODO: Handle this error
            return Ok(());
        }
    };

    let selection = match saves.get(index) {
        Some(s) => s,
        None => {
            eprintln!("Error selecting list: Out of range selection");
            // TODO: Handle this error
            return Ok(());
        }
    };

    let mut course_list = CourseList::new(selection.path());
    match course_list.restore_list() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error while loading the saved course list: {}", e);
            // TODO: Handle this error
            return Ok(());
        }
    }

    println!("\nEnter 'help' for help information.");
    loop {
        input = continue_on_err!(get_input(":> "), "Error reading input");

        match input.trim().to_lowercase().as_ref() {
            "" => {
                let Some(course) = course_list.get_random() else {
                    println!("The course list is empty. Resetting.");
                    course_list.reset();
                    continue;
                };

                println!("{}", course);
                course_list.remove(course.clone());
            }

            "q" | "quit" => {
                println!("Save changes before quitting? (Y/N): ");
                input = continue_on_err!(get_input(":> "), "Error reading input");
                match input.trim().to_lowercase().as_ref() {
                    "y" => {
                        continue_on_err!(course_list.dump_list(), "Error saving list");
                        println!("Saved successfully.");
                    }

                    "n" => {}

                    _ => {
                        println!("Please select Y/N.");
                        continue;
                    }
                }

                println!("Quitting...");
                break;
            }

            "help" => help(),

            "save" => {
                continue_on_err!(course_list.dump_list(), "Error saving list");
                println!("Saved successfully.");
            }

            "remaining" | "re" | "ls" => {
                let current = course_list.get_current();
                if current.is_empty() {
                    println!("Course list is empty.");
                    continue;
                }

                let strings: Vec<String> = current.iter().map(|c| c.to_string()).collect();
                println!("{}", strings.join("\n"));
                println!("There are {} courses in the list.", current.len())
            }

            "used" => {
                let removed = course_list.get_removed();
                if removed.is_empty() {
                    println!("No courses have been used.");
                    continue;
                }

                let strings: Vec<String> = removed.iter().map(|c| c.to_string()).collect();
                println!("{}", strings.join("\n"));
                println!("{} courses have been used.", removed.len())
            }

            "history" => println!("{}", course_list.get_history()),

            "reset" => {
                input = continue_on_err!(
                    get_input("Are you sure? ('Y' to confirm): "),
                    "Error reading input"
                );

                match input.trim() {
                    "Y" => {
                        course_list.reset();
                        println!("Course list reset.");
                    }

                    _ => {
                        println!("Cancelled reset.");
                        continue;
                    }
                }
            }

            "back" => {
                if course_list.roll_back().is_err() {
                    eprintln!("Error rolling back: No history found");
                }
            }

            "forward" => {
                if course_list.roll_forward().is_err() {
                    eprintln!("Error rolling forward: No future found");
                }
            }

            "add" => {
                input = continue_on_err!(get_input("Search courses: "), "Error reading input");

                let mut results: Vec<&Course> =
                    course_list.search_removed(&input).into_iter().collect();
                results.sort();

                for (i, c) in results.iter().enumerate() {
                    println!("{}: {}", i + 1, c);
                }

                input = continue_on_err!(
                    get_input("Select the number of the course to add: "),
                    "Error reading input"
                );
                index = continue_on_err!(input.parse(), "Error parsing number");
                let Some(&selection) = results.get(index.wrapping_sub(1)) else {
                    eprintln!("Error selecting course: Out of bounds selection");
                    continue;
                };

                course_list.add(selection.clone());
            }

            "remove" | "rm" | "pop" => {
                input = continue_on_err!(get_input("Search courses: "), "Error reading input");

                let mut results: Vec<&Course> =
                    course_list.search_current(&input).into_iter().collect();
                results.sort();

                for (i, c) in results.iter().enumerate() {
                    println!("{}: {}", i + 1, c);
                }

                input = continue_on_err!(
                    get_input("Select the number of the course to remove: "),
                    "Error reading input"
                );
                index = continue_on_err!(input.parse(), "Error parsing number");
                let Some(&selection) = results.get(index.wrapping_sub(1)) else {
                    eprintln!("Error selecting course: Out of bounds selection");
                    continue;
                };

                course_list.remove(selection.clone());
            }

            "tier" => {
                input = continue_on_err!(get_input("Enter the size of the prix: "), "Error reading input");
                let size: usize = continue_on_err!(input.parse());

                let tiered_courses = match course_list.get_random_by_chunks(size) {
                    Ok(c) => c,
                    Err(_) => {
                        eprintln!(
                            "Error: Could not divide courses into tiers.\n\
                            This probably means the course list cannot be evenly divided by the given prix size.\n\
                            Consider adding or removing courses until the list is evenly divisible."
                        );
                        continue;
                    }
                };

                if run_tiered_list(tiered_courses.clone()) {
                    for course in tiered_courses {
                        course_list.remove(course);
                    }
                }
            }

            _ => println!("Unrecognized command."),
        }
    }

    Ok(())
}

fn help() {
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

fn run_tiered_list(mut list: Vec<Course>) -> bool {
    let mut rng = rand::rng();
    println!(
        "Entered tiered list. Type 'back' to return without removing the selected courses."
    );

    let mut input: String;
    while !list.is_empty() {
        input = continue_on_err!(get_input(":> "), "Error reading input");
        
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
