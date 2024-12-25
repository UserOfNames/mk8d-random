import random, os, pickle

class SizeError(Exception):
    pass


class EmptyListError(Exception):
    pass


class NoMatchError(Exception):
    pass


class Course():
    def __init__(self, rank, coord, name):
        self.rank = rank
        self.coord = coord
        self.name = name

    def __str__(self):
        return f"({self.coord}) {self.name} ({self.rank})"


class CourseList():
    def __init__(self, course_list=[]):
        # course_list will change over time, static_list is a permanent backup
        self.course_list = course_list
        self.static_list = course_list.copy()
        # old_list is a checkpoint that updates after each change for undoing
        self.old_list = course_list.copy()
        self.is_tiered = False

        # shut up diagnostics
        self.tiered_list = []


    def __sub__(self, other):
        diff_list = list(set(self.course_list) - set(other.course_list))
        diff_list.sort(key = lambda x: x.rank)
        return CourseList(diff_list)


    def overwrite_persistent_list(self):
        if self.is_tiered:
            return

        with open(list_file, 'wb') as wfile:
            pickle.dump(self.course_list, wfile, pickle.HIGHEST_PROTOCOL)


    def read_persistent_list(self):
        with open(list_file, 'rb') as rfile:
            self.course_list = pickle.load(rfile)


    def print_list(self):
        for course in sorted(self.course_list, key=lambda x: x.coord):
            print(course)


    def backup_list(self):
        self.old_list = self.course_list.copy()


    def search_list(self, key):
        matches = {}
        i = 1

        for course in sorted(self.course_list, key=lambda x: x.coord):
            if key in course.name.lower():
                matches[i] = course
                i += 1

        return matches


    def add_course(self, *args):
        self.backup_list()
        for course in args:
            self.course_list.append(course)
        self.course_list.sort(key=lambda x: x.rank)


    def remove_course(self, *args):
        self.backup_list()
        for course in args:
            self.course_list.remove(course)


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


    def undo_last_action(self):
        old_list_temp = self.old_list.copy()
        self.backup_list()
        self.course_list = old_list_temp


    def generate_course(self):
        if len(self.course_list) == 0:
            raise EmptyListError

        course = self.course_list[random.randrange(len(self.course_list))]
        print(course)
        self.remove_course(course)


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


def reset_active_list():
    active_course_list.backup_list()
    active_course_list.course_list = full_course_list.course_list.copy()

full_course_list = CourseList([
    Course(1, 314, "Wii  Coconut Mall"),
    Course(2, 354, "Wii  Maple Treeway"),
    Course(3, 211, "Wii  Moo Moo Meadows"),
    Course(4, 434, "MK8  Squeaky Clean Sprint"),
    Course(5, 241, "DS   Tick Tock Clock"),
    Course(6, 362, "DS   Peach Gardens"),
    Course(7, 433, "Wii  Moonview Highway"),
    Course(8, 413, "Wii  DK Summit"),
    Course(9, 253, "MK8  Ice Ice Outpost"),
    Course(10, 353, "3DS  Rock Rock Mountain"),
    Course(11, 344, "MK8  Sky-High Sundae"),
    Course(12, 131, "MK8  Sunshine Airport"),
    Course(13, 232, "GCN  Sherbet Land"),
    Course(14, 334, "DS   Waluigi Pinball"),
    Course(15, 233, "3DS  Music Park"),
    Course(16, 464, "Wii  Rainbow Road"),
    Course(17, 443, "Wii  Koopa Cape"),
    Course(18, 244, "N64  Rainbow Road"),
    Course(19, 234, "N64  Yoshi Valley"),
    Course(20, 343, "Wii  Mushroom Gorge"),
    Course(21, 134, "MK8  Mount Wario"),
    Course(22, 141, "MK8  Cloudtop Cruise"),
    Course(23, 432, "GCN  Daisy Cruiser"),
    Course(24, 214, "N64  Toad's Turnpike"),
    Course(25, 322, "DS   Shroom Ridge"),
    Course(26, 364, "3DS  Rainbow Road"),
    Course(27, 251, "Wii  Wario's Gold Mine"),
    Course(28, 333, "N64  Kalimari Desert"),
    Course(29, 453, "Wii  Daisy Circuit"),
    Course(30, 133, "MK8  Electrodrome"),
    Course(31, 323, "GBA  Sky Garden"),
    Course(32, 324, "Tour Ninja Hideaway"),
    Course(33, 452, "GCN  DK Mountain"),
    Course(34, 462, "3DS  Rosalina's Ice World"),
    Course(35, 122, "MK8  Toad Harbor"),
    Course(36, 264, "MK8  Big Blue"),
    Course(37, 154, "MK8  Mute City"),
    Course(38, 263, "MK8  Super Bell Subway"),
    Course(39, 414, "MK8  Yoshi's Island"),
    Course(40, 143, "MK8  Bowser's Castle"),
    Course(41, 163, "MK8  Wild Woods"),
    Course(42, 242, "3DS  Piranha Plant Slide"),
    Course(43, 213, "DS   Cheep Cheep Beach"),
    Course(44, 254, "MK8  Hyrule Circuit"),
    Course(45, 243, "Wii  Grumble Volcano"),
    Course(46, 114, "MK8  Thwomp Ruins"),
    Course(47, 153, "MK8  Dragon Driftway"),
    Course(48, 144, "MK8  Rainbow Road"),
    Course(49, 224, "3DS  DK Jungle"),
    Course(50, 312, "3DS  Toad Circuit"),
    Course(51, 132, "MK8  Dolphin Shoals"),
    Course(52, 363, "Tour Merry Mountain"),
    Course(53, 423, "GCN  Waluigi Stadium"),
    Course(54, 112, "MK8  Water Park"),
    Course(55, 223, "N64  Royal Raceway"),
    Course(56, 421, "Tour Bangkok Rush"),
    Course(57, 123, "MK8  Twisted Mansion"),
    Course(58, 164, "MK8  Animal Crossing Circuit"),
    Course(59, 454, "MK8  Piranha Plant Cove"),
    Course(60, 342, "GBA  Snow Land"),
    Course(61, 262, "GBA  Ribbon Road"),
    Course(62, 152, "MK8  Excitebike Arena"),
    Course(63, 461, "Tour Madrid Drive"),
    Course(64, 424, "Tour Singapore Speedway"),
    Course(65, 444, "Tour Vancouver Velocity"),
    Course(66, 113, "MK8  Sweet Sweet Canyon"),
    Course(67, 331, "Tour New York Minute"),
    Course(68, 111, "MK8  Mario Kart Stadium"),
    Course(69, 442, "GBA  Sunset Wilds"),
    Course(70, 451, "Tour Rome Avanti"),
    Course(71, 124, "MK8  Shy Guy Falls"),
    Course(72, 121, "MK8  Mario Circuit"),
    Course(73, 212, "GBA  Mario Circuit"),
    Course(74, 361, "Tour Berlin Byways"),
    Course(75, 412, "GBA  Riverside Park"),
    Course(76, 252, "SNES Rainbow Road"),
    Course(77, 332, "SNES Mario Circuit 3"),
    Course(78, 221, "GCN  Dry Dry Desert"),
    Course(79, 422, "DS   Mario Circuit"),
    Course(80, 311, "Tour Paris Promenade"),
    Course(81, 463, "SNES Bowser's Castle 3"),
    Course(82, 352, "GBA  Boo Lake"),
    Course(83, 431, "Tour Athens Dash"),
    Course(84, 313, "N64  Choco Mountain"),
    Course(85, 222, "SNES Donut Plains 3"),
    Course(86, 441, "Tour Los Angeles Laps"),
    Course(87, 411, "Tour Amsterdam Drift"),
    Course(88, 142, "MK8  Bone Dry Ruins"),
    Course(89, 351, "Tour London Loop"),
    Course(90, 261, "3DS  Neo Bowser City"),
    Course(91, 231, "DS   Wario Stadium"),
    Course(92, 151, "GCN  Yoshi's Circuit"),
    Course(93, 161, "GCN  Baby Park"),
    Course(94, 341, "Tour Sydney Sprint"),
    Course(95, 321, "Tour Tokyo Blur"),
    Course(96, 162, "GBA  Cheese Land"),
])


help_block = '''
q/quit/exit: Stop the script.
To avoid accidental generation, any input besides these commands will do nothing.

Enter to generate a new course. (Blank input)

Information:
  remaining/re/ls: Print a list of remaining courses.
             used: Print a list of used courses.

List editing:
         reset: Reset the course list.
          undo: Undo the previous list edit. Will undo itself as well.
           add: Add a previously generated course back into the cycle.
 remove/rm/pop: Remove a course from the current cycle.

Special:
  tier: Pick a prix size N. Evenly split the list into tiers
          of size N. Pick one random course from each tier
          removing that course from the FULL list as you go.
          After drawing one course from each tier, return to
          standard generation (will notify when this happens).
          If you are already using a tiered list, this will abort
          and resume normal genration.
'''


list_file = os.path.join(os.path.dirname(os.path.realpath(__file__)), 'list.pkl')


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


print("Enter 'help' for commands.")
while True:
    user_input = input(":> ").lower()
    used_course_list = full_course_list - active_course_list


    try:
        match user_input:
            case "q" | "quit" | "exit":
                break


            case "help":
                print(help_block)


            case "remaining" | "re" | "ls":
                active_course_list.print_list()
                print(f"There are {len(active_course_list.course_list)} courses in the list.")


            case "used":
                used_course_list.print_list()
                print(f"{len(used_course_list.course_list)} courses have been used.")


            case "reset":
                reset_active_list()
                active_course_list.overwrite_persistent_list()
                print("Course list reset.")


            case "undo":
                active_course_list.undo_last_action()
                active_course_list.overwrite_persistent_list()
                print("Previous action undone.")


            case "add":
                try:
                    active_course_list.search_and_add(used_course_list)
                    active_course_list.overwrite_persistent_list()
                    print("Course added successfully.")

                except ValueError:
                    print("Error: Not a number.")
                except KeyError:
                    print("Error: Invalid key.")


            case "remove" | "rm" | "pop":
                try:
                    active_course_list.search_and_remove()
                    active_course_list.overwrite_persistent_list()
                    print("Course removed successfully.")

                except ValueError:
                    print("Error: Not a number.")
                except KeyError:
                    print("Error: Invalid key.")


            case "tier":
                if active_course_list.is_tiered:
                    exit_tiered = input("Already using a tiered list. Resume normal generation? 'y' to confirm (this will prevent the course list from updating!): ").lower()
                    if exit_tiered == "y":
                        print("Resuming normal generation.")
                        active_course_list = CourseList(active_course_list.static_list)
                        continue
                    else:
                        print("Continuing tiered generation.")

                prix_size = input("Enter the size of the prix: 4/6/8/12/16/24/32/48: ")
                if prix_size not in ['4', '6', '8', '12', '16', '24', '32', '48']:
                    raise SizeError("Error: Invalid prix size.")
                prix_size = int(prix_size)

                active_course_list = TieredList(prix_size, active_course_list.course_list)


            case "":
                active_course_list.generate_course()
                active_course_list.overwrite_persistent_list()
            

            case _:
                print("Error: Invalid input.")


    except EmptyListError:
        if active_course_list.is_tiered:
            print("The tiered list is empty. Resuming normal generation.")
            active_course_list = CourseList(active_course_list.static_list) - CourseList(active_course_list.tiered_list)
            active_course_list.overwrite_persistent_list()

        else:
            print("The course list is empty. Resetting.")
            reset_active_list()
            active_course_list.overwrite_persistent_list()

    except NoMatchError:
        print("Error: No matches found.")

    except SizeError as err:
        print(err)

    except:
        print("I don't know what you just did, but you generated an error I didn't anticipate. Congratulations.")
