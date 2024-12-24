import os

# Get a list of user-defined lists that haven't been saved yet
def scan_lists():
    base_dir  = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
    lists_dir = os.path.join(base_dir, "lists")
    saves_dir = os.path.join(base_dir, "lists", "saves")

    lists = os.listdir(lists_dir)
    lists.remove("saves")
    saves = set(os.listdir(saves_dir))

    return [l for l in lists if l not in saves]
