use std::fs::DirEntry;
use std::io;

use crate::courses::course_list::CourseList;

use mk8d_random::continue_on_err;
use mk8d_random::utils::get_input;

pub fn repl(saves: Vec<DirEntry>) -> io::Result<()> {
    let mut input: String;
    let index: usize;

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

        match input.to_lowercase().as_ref() {
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
                println!("Quitting...");
                break;
            },

            "help" => help(),

            "remaining" | "re" | "ls" => course_list.print_current(),

            "used" => course_list.print_removed(),

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

            "add" => todo!(),

            "remove" | "rm" | "pop" => todo!(),

            "tier" => todo!(),

            _ => println!("Unrecognized command."),
        }
    }

    Ok(())
}

fn help() {
    println!("---------------------------------------------------");

    println!(
        "Blank input: Generate and remove a random course.\n\
        q, quit:      Stop the script.\n\
        help:         Show this help text.\n"
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
