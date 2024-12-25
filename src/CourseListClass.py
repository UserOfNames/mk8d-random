import random, os, pickle, importlib
from collections import deque
from src.CourseClass import Course

class CourseList():
    def __init__(self, list_name):
        base_dir  = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
        lists_dir = os.path.join(base_dir, "lists")
        saves_dir = os.path.join(base_dir, "saves")

        list_file = importlib.import_module(f"lists.{list_name}")

        self.save_file   = os.path.join(saves_dir, f"{list_name}.pkl")
        self.course_list = list_file.course_list
        self.history     = deque()

        self.overwrite_save_file()


    def __sub__(self, other):
        other_set = set(other.course_list)
        res = [course for course in self.course_list if course not in other_set]
        return CourseList(res)


    def overwrite_save_file(self) -> None:
        with open(self.save_file, "wb") as wfile:
            pickle.dump(self, wfile, pickle.HIGHEST_PROTOCOL)


    def print_list(self) -> None:
        for course in sorted(self.course_list, key=lambda x: x.coord):
            print(course)


    # TODO: Finish implementing CourseList
