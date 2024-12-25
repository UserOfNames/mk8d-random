import os

# Get a list of user-defined lists that haven't been saved yet
def scan_lists() -> list:
    base_dir  = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
    lists_dir = os.path.join(base_dir, "lists")
    saves_dir = os.path.join(base_dir, "saves")

    lists = os.listdir(lists_dir)
    saves = set(os.listdir(saves_dir))

    try:
        lists.remove("__pycache__")
    except:
        pass

    return [l for l in lists if l not in saves]
