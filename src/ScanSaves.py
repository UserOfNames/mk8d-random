import os

def scan_saves() -> list:
    base_dir  = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
    saves_dir = os.path.join(base_dir, "saves")

    saves = os.listdir(saves_dir)

    try:
        saves.remove("__pycache__")
    except:
        pass

    return saves
