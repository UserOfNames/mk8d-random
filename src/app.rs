#[derive(Debug)]
pub enum CurrentScreen {
    Load,      // Loading screen
    In,        // Displays courses in current
    Out,       // Displays courses in removed
    SearchIn,  // Filter current
    SearchOut, // Filter removed
    Tiered,    // Tiered sublist
}

#[derive(Debug)]
pub struct App {
    pub searched: String,
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> Self {
        App {
            searched: String::new(),
            current_screen: CurrentScreen::Load,
        }
    }
}
