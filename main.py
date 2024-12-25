from src import CourseListClass as cl

from textual.app import App, ComposeResult
from textual.widgets import Header, Footer

c = cl.CourseList("mk8d")

class Main(App):
    BINDINGS = [
        ("s", "scan_lists", "Scan for new lists"),
    ]

    def compose(self) -> ComposeResult:
        yield Header()
        yield Footer()

    def action_scan_lists(self) -> None:
        pass


# app = Main()
# app.run()


# Initial screen: List of CL
# Scan for CL: Popup or screen of new lists, checkable

# Select a CL: List of remaining courses, checkable?
    # Back: Go back to list of CL
