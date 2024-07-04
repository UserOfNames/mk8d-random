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
        return f"({self.coord}) {self.name}"


class CourseList():
    def __init__(self, course_list=[]):
        self.course_list = course_list
        self.old_list = course_list.copy()
        self.is_tiered = False


    def __xor__(self, other):
        symdiff = []
        symdiff_ranks = set(course.rank for course in self.course_list) ^ set(course.rank for course in other.course_list)

        for course in full_course_list.course_list:
            if course.rank in symdiff_ranks:
                symdiff.append(course)

        symdiff.sort(key=lambda x: x.rank)
        return CourseList(symdiff)


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


    def search_and_add(self):
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
        self.course_list, self.original_course_list = course_list, course_list
        self.prix_size = prix_size
        self.is_tiered = True

        if len(self.course_list) == 0:
            raise EmptyListError

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


def reset_active_list():
    active_course_list.backup_list()
    active_course_list.course_list = full_course_list.course_list.copy()


full_course_list = CourseList([
    Course(1,  314, "Wii  Coconut Mall"),
    Course(2,  354, "Wii  Maple Treeway"),
    Course(3,  241, "DS   Tick Tock Clock"),
    Course(4,  434, "MK8  Squeaky Clean Sprint"),
    Course(5,  211, "Wii  Moo Moo Meadows"),
    Course(6,  362, "DS   Peach Gardens"),
    Course(7,  433, "Wii  Moonview Highway"),
    Course(8,  413, "Wii  DK Summit"),
    Course(9,  253, "MK8  Ice Ice Outpost"),
    Course(10, 334, "DS   Waluigi Pinball"),
    Course(11, 233, "3DS  Music Park"),
    Course(12, 344, "MK8  Sky-High Sundae"),
    Course(13, 232, "GCN  Sherbet Land"),
    Course(14, 464, "Wii  Rainbow Road"),
    Course(15, 324, "Tour Ninja Hideaway"),
    Course(16, 353, "3DS  Rock Rock Mountain"),
    Course(17, 131, "MK8  Sunshine Airport"),
    Course(18, 234, "N64  Yoshi Valley"),
    Course(19, 343, "Wii  Mushroom Gorge"),
    Course(20, 244, "N64  Rainbow Road"),
    Course(21, 443, "Wii  Koopa Cape"),
    Course(22, 322, "DS   Shroom Ridge"),
    Course(23, 214, "N64  Toad's Turnpike"),
    Course(24, 134, "MK8  Mount Wario"),
    Course(25, 141, "MK8  Cloudtop Cruise"),
    Course(26, 432, "GCN  Daisy Cruiser"),
    Course(27, 163, "MK8  Wild Woods"),
    Course(28, 364, "3DS  Rainbow Road"),
    Course(29, 333, "N64  Kalimari Desert"),
    Course(30, 323, "GBA  Sky Garden"),
    Course(31, 462, "3DS  Rosalina's Ice World"),
    Course(32, 414, "MK8  Yoshi's Island"),
    Course(33, 453, "Wii  Daisy Circuit"),
    Course(34, 122, "MK8  Toad Harbor"),
    Course(35, 264, "MK8  Big Blue"),
    Course(36, 154, "MK8  Mute City"),
    Course(37, 251, "Wii  Wario's Gold Mine"),
    Course(38, 452, "GCN  DK Mountain"),
    Course(39, 263, "MK8  Super Bell Subway"),
    Course(40, 114, "MK8  Thwomp Ruins"),
    Course(41, 143, "MK8  Bowser's Castle"),
    Course(42, 213, "DS   Cheep Cheep Beach"),
    Course(43, 254, "MK8  Hyrule Circuit"),
    Course(44, 312, "3DS  Toad Circuit"),
    Course(45, 243, "Wii  Grumble Volcano"),
    Course(46, 153, "MK8  Dragon Driftway"),
    Course(47, 454, "MK8  Piranha Plant Cove"),
    Course(48, 144, "MK8  Rainbow Road"),
    Course(49, 242, "3DS  Piranha Plant Slide"),
    Course(50, 224, "3DS  DK Jungle"),
    Course(51, 132, "MK8  Dolphin Shoals"),
    Course(52, 423, "GCN  Waluigi Stadium"),
    Course(53, 112, "MK8  Water Park"),
    Course(54, 223, "N64  Royal Raceway"),
    Course(55, 421, "Tour Bangkok Rush"),
    Course(56, 123, "MK8  Twisted Mansion"),
    Course(57, 342, "GBA  Snow Land"),
    Course(58, 262, "GBA  Ribbon Road"),
    Course(59, 152, "MK8  Excitebike Arena"),
    Course(60, 461, "Tour Madrid Drive"),
    Course(61, 424, "Tour Singapore Speedway"),
    Course(62, 111, "MK8  Mario Kart Stadium"),
    Course(63, 221, "GCN  Dry Dry Desert"),
    Course(64, 363, "Tour Merry Mountain"),
    Course(65, 113, "MK8  Sweet Sweet Canyon"),
    Course(66, 133, "MK8  Electrodrome"),
    Course(67, 331, "Tour New York Minute"),
    Course(68, 164, "MK8  Animal Crossing Circuit"),
    Course(69, 442, "GBA  Sunset Wilds"),
    Course(70, 411, "Tour Amsterdam Drift"),
    Course(71, 431, "Tour Athens Dash"),
    Course(72, 124, "MK8  Shy Guy Falls"),
    Course(73, 121, "MK8  Mario Circuit"),
    Course(74, 212, "GBA  Mario Circuit"),
    Course(75, 361, "Tour Berlin Byways"),
    Course(76, 412, "GBA  Riverside Park"),
    Course(77, 252, "SNES Rainbow Road"),
    Course(78, 332, "SNES Mario Circuit 3"),
    Course(79, 422, "DS   Mario Circuit"),
    Course(80, 311, "Tour Paris Promenade"),
    Course(81, 313, "N64  Choco Mountain"),
    Course(82, 222, "SNES Donut Plains 3"),
    Course(83, 441, "Tour Los Angeles Laps"),
    Course(84, 451, "Tour Rome Avanti"),
    Course(85, 231, "DS   Wario Stadium"),
    Course(86, 161, "GCN  Baby Park"),
    Course(87, 142, "MK8  Bone Dry Ruins"),
    Course(88, 444, "Tour Vancouver Velocity"),
    Course(89, 351, "Tour London Loop"),
    Course(90, 261, "3DS  Neo Bowser City"),
    Course(91, 151, "GCN  Yoshi's Circuit"),
    Course(92, 321, "Tour Tokyo Blur"),
    Course(93, 341, "Tour Sydney Sprint"),
    Course(94, 162, "GBA  Cheese Land"),
    Course(95, 463, "SNES Bowser's Castle 3"),
    Course(96, 352, "GBA  Boo Lake"),
])


help_block = '''
q/quit/exit: Stop the script.
To avoid accidental generation, any input besides these commands will do nothing.

Enter to generate a new course. (Blank input)

Information:
  remaining: Print a list of remaining courses.
       used: Print a list of used courses.

List editing:
  reset: Reset the course list.
   undo: Undo the previous list edit. Will undo itself as well.
    add: Add a previously generated course back into the cycle.
 remove: Remove a course from the current cycle.

Special:
  tier: Pick a prix size N. Evenly split the list into tiers
          of size N. Pick one random course from each tier
          removing that course from the FULL list as you go.
          After drawing one course from each tier, return to
          standard generation (will notify when this happens).
          If you are already using a tiered list, this will abort
          and resume normal genration.
'''


list_file = os.path.join(os.path.dirname(os.path.realpath(__file__)), 'list.json')


try:
    with open(list_file, 'rb') as rfile:
        active_course_list = CourseList(pickle.load(rfile))
except:
    with open(list_file, 'wb') as wfile:
        pickle.dump(full_course_list.course_list, wfile, pickle.HIGHEST_PROTOCOL)
    active_course_list = CourseList(full_course_list.course_list)

# testlist = [
#     Course(1,  314, "Wii  Coconut Mall"),
#     Course(2,  354, "Wii  Maple Treeway"),
#     Course(3,  241, "DS   Tick Tock Clock"),
#     Course(4,  434, "MK8  Squeaky Clean Sprint"),
#     Course(5,  211, "Wii  Moo Moo Meadows"),
#     Course(6,  362, "DS   Peach Gardens"),
# ]
# active_course_list = CourseList(testlist)


print("Enter 'help' for commands.")
while True:
    user_input = input(":> ").lower()
    used_courses = active_course_list ^ full_course_list


    if user_input in ['q', 'quit', 'exit']:
        break


    try:
        match user_input:
            case "help":
                print(help_block)


            case "remaining":
                active_course_list.print_list()
                print(f"There are {len(active_course_list.course_list)} courses in the list.")


            case "used":
                used_courses.print_list()
                print(f"{len(used_courses.course_list)} courses have been used.")


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
                    active_course_list.search_and_add()
                    active_course_list.overwrite_persistent_list()
                    print("Course added successfully.")

                except ValueError:
                    print("Error: Not a number.")
                except KeyError:
                    print("Error: Invalid key.")


            case "remove":
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
                        active_course_list = CourseList(active_course_list.original_course_list)
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
            confirm_save_tiered_changes = input("The tiered list is empty. Resuming normal generation. Remove tiered courses from the main list? 'n' to deny: ").lower()
            if confirm_save_tiered_changes == "n":
                active_course_list = CourseList(active_course_list.original_course_list)
            else:
                active_course_list = CourseList(active_course_list.original_course_list) ^ CourseList(active_course_list.tiered_list)

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
