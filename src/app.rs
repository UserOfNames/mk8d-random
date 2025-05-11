#[derive(Debug)]
pub enum CurrentScreen {
    Load,   // Loading a list
    In,     // Displays courses in the loaded list
    Out,    // Displays courses removed from the loaded list
    Search, // Search a specific course in or out
    Tiered, // Tiered sublist
}

#[derive(Debug)]
pub struct App {
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::Load,
        }
    }
}
