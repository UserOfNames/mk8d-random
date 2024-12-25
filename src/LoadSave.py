import os, pickle

def load_save(save_name: str):
    base_dir = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
    target   = os.path.join(base_dir, "saves", f"{save_name}.pkl")

    if not os.path.exists(target):
        # TODO: Handle this case properly
        raise Exception()

    with open(target, "rb") as rfile:
        return pickle.load(rfile)
