import random, os, pickle

class SizeError(Exception):
    pass


class EmptyListError(Exception):
    pass


class NoMatchError(Exception):
    pass


class CourseList():
    def __init__(self, course_list=[]):
        self.is_tiered = False

        # shut up diagnostics
        self.tiered_list = []


    def __sub__(self, other):
        diff_list = list(set(self.course_list) - set(other.course_list))
        diff_list.sort(key = lambda x: x.rank)
        return CourseList(diff_list)


    def search_and_add(self, used_courses):
        search_key = input("Search for a course (blank will search all courses): ")
        matches = used_courses.search_list(search_key)

        if len(matches) == 0:
            raise NoMatchError

        for key, course in matches.items(): print(f"{key}: {course}")

        key_selection = input("Enter the key associated with the course you want to add ('all' to add all matches): ").lower()

        if key_selection == "all":
            for course in matches.values(): active_course_list.add_course(course)
            return

        index = int(key_selection)
        active_course_list.add_course(matches[index])


    def search_and_remove(self):
        search_key = input("Search for a course (blank will search all courses): ")
        matches = active_course_list.search_list(search_key)

        if len(matches) == 0:
            raise NoMatchError

        for key, course in matches.items(): print(f"{key}: {course}")

        key_selection = input("Enter the key associated with the course you want to remove ('all' to remove all matches): ").lower()

        if key_selection == "all":
            for course in matches.values(): active_course_list.remove_course(course)
            return

        index = int(key_selection)
        active_course_list.remove_course(matches[index])


class TieredList(CourseList):
    def __init__(self, prix_size, course_list = []):
        self.course_list = course_list
        self.static_list = course_list.copy()
        self.is_tiered = True


        if len(self.course_list) < prix_size:
            raise SizeError("Error: Not enough courses")


        if len(self.course_list) % prix_size != 0:
            if input("Cannot evenly divide the course list. Continuing will stagger the tiers. Proceed? ('y' to confirm): ").lower() in ["y", "yes"]:
                self.make_divisible(prix_size)


        chunked_list = self.chunk_list(prix_size)
        self.tiered_list = []

        for sublist in chunked_list:
            index = random.randrange(len(sublist))
            self.tiered_list.append(sublist[index])

        self.course_list = self.tiered_list.copy()


    def chunk_list(self, size):
        chunked = []
        length = len(self.course_list)
        chunk_size = length // size

        if chunk_size != length / size:
            raise SizeError("Error: Cannot evenly divide the course list by that number.")

        for i in range(0, length, chunk_size):
            chunked.append(self.course_list[i:i+chunk_size])

        return chunked


    def make_divisible(self, size):
        while len(self.course_list) % size != 0:
            index = random.randrange(len(self.course_list))
            self.course_list.pop(index)

try:
    with open(list_file, 'rb') as rfile:
        # This ensures course objects in the active list are the same as in the full list, makes some methods cleaner
        saved_list = pickle.load(rfile)


    saved_ranks = [course.rank for course in saved_list]

    saved_list_cleaned = [course for course in full_course_list.course_list if course.rank in saved_ranks]
    
    saved_list_cleaned.sort(key = lambda x: x.rank)

    active_course_list = CourseList(saved_list_cleaned)


except:
    with open(list_file, 'wb') as wfile:
        pickle.dump(full_course_list.course_list, wfile, pickle.HIGHEST_PROTOCOL)
    active_course_list = full_course_list
