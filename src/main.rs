mod app;
mod courses;
mod lists;
mod ui;

use std::io;

use app::App;
use courses::course_list::CourseList;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

// fn run() {
//     run();
//
//     let path = "../saves/mk8d.json";
//     let mut courses = CourseList::new(path);
//
//     match courses.restore_list() {
//         Ok(_) => (),
//         Err(_) => {
//             println!("Did not find a save file, creating one...");
//             courses.list = make_mk8d();
//             courses
//                 .dump_list()
//                 .expect("ERROR While creating initial course list");
//         }
//     }
//
//     println!("There are {} courses in the list.", courses.list.len());
// }
