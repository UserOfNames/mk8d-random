from textual.events import Mount
from src import CourseListClass as cl
from src import ScanLists as sl

from textual.app import App, ComposeResult
from textual.widgets import Header, Footer, Button, Label
from textual.containers import VerticalScroll

c = cl.CourseList("mk8d")

class Main(App):
    BINDINGS = [
        ("s", "scan_lists", "Scan for new lists"),
    ]

    def compose(self) -> ComposeResult:
        yield Header()
        yield Footer()

    def action_scan_lists(self) -> None:
        Label("hello").mount()


app = Main()
app.run()
