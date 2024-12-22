import random, os, pickle
from collections import deque
from lists import mk8d


class SizeError(Exception):
    pass


class EmptyListError(Exception):
    pass


class NoMatchError(Exception):
    pass


class CourseList():
    def __init__(self, list_name):
        cwd       = os.path.dirname(os.path.realpath(__file__))
        lists_dir = os.path.join(cwd, "lists")
        saves_dir = os.path.join(cwd, "lists", "saves")

        # Create buttons corresponding to lists
        lists = os.listdir(lists_dir)

        self.history = deque()


    def __sub__(self, other):
        diff_list = list(set(self.course_list) - set(other.course_list))
        diff_list.sort(key = lambda x: x.rank)
        return CourseList(diff_list)


    def overwrite_list_file(self):
        with open(self.list_file, 'wb') as wfile:
            pickle.dump(self, wfile, pickle.HIGHEST_PROTOCOL)


    def print_list(self):
        for course in sorted(self.course_list, key=lambda x: x.coord):
            print(course)


c = CourseList(mk8d.course_list)
