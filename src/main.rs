mod lists;
use lists::mk8d::make_mk8d;

mod courses;
use courses::CourseList;

fn main() {
    run();
}

fn run() {
    let path = "../saves/mk8d.json";
    let mut courses = CourseList::new(path);

    match courses.restore_list() {
        Ok(_) => (),
        Err(_) => {
            println!("Did not find a save file, creating one...");
            courses.list = make_mk8d();
            courses
                .dump_list()
                .expect("ERROR While creating initial course list");
        }
    }

    println!("There are {} courses in the list.", courses.list.len());
}
