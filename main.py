import random, os, pickle

class ChunkSizeError(Exception):
    pass


class Course():
    def __init__(self, rank, coord, name):
        self.rank = rank
        self.coord = coord
        self.name = name

    def __str__(self):
        return f"({self.coord}) {self.name}"


class CourseList():
    def __init__(self, course_list=[], is_tiered=False):
        self.course_list = course_list
        self.is_tiered = is_tiered
        self.old_list = course_list.copy()


    def __xor__(self, other):
        symdiff = []
        symdiff_ranks = set(course.rank for course in self.course_list) ^ set(course.rank for course in other.course_list)

        for course in full_course_list.course_list:
            if course.rank in symdiff_ranks:
                symdiff.append(course)

        symdiff.sort(key=lambda x: x.rank)
        return CourseList(symdiff)


    def overwrite_persistent_list(self):
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
        self.overwrite_persistent_list()


    def remove_course(self, *args):
        self.backup_list()
        for course in args:
            self.course_list.remove(course)
        self.overwrite_persistent_list()


    def undo_last_action(self):
        try:
            old_list_temp = self.old_list.copy()
            self.backup_list()
            self.course_list = old_list_temp
            self.overwrite_persistent_list()
        except:
            print("Error: You probably haven't done an undoable action.")


    def generate_course(self):
        course = self.course_list[random.randrange(len(self.course_list))]
        print(course)
        self.remove_course(course)
        self.overwrite_persistent_list()


    def chunk_list(self, size):
        chunked = []
        length = len(self.course_list)
        chunk_size = length // size

        if chunk_size != length / size:
            raise ChunkSizeError

        for i in range(0, length, chunk_size):
            chunked.append(self.course_list[i:i+chunk_size])

        return chunked


list_file = os.path.join(os.path.dirname(os.path.realpath(__file__)), 'list.json')


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


try:
    with open(list_file, 'rb') as rfile:
        active_course_list = CourseList(pickle.load(rfile))
except:
    with open(list_file, 'wb') as wfile:
        pickle.dump(full_course_list.course_list, wfile, pickle.HIGHEST_PROTOCOL)
    active_course_list = CourseList(full_course_list.course_list)


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


print("Enter 'help' for commands.")
while True:
    user_input = input(":> ").lower()
    used_courses = active_course_list ^ full_course_list

    if user_input in ['q', 'quit', 'exit']:
        break

    match user_input:
        case "help":
            print(help_block)
            continue


        case "remaining":
            active_course_list.print_list()
            print(f"There are {len(active_course_list.course_list)} courses in the list.")
            continue


        case "used":
            used_courses.print_list()
            print(f"{len(used_courses.course_list)} courses have been used.")
            continue


        case "reset":
            active_course_list.backup_list()
            active_course_list.course_list = full_course_list.course_list.copy()
            active_course_list.overwrite_persistent_list()
            print("Course list reset.")
            continue


        case "undo":
            active_course_list.undo_last_action()
            continue


        case "add":
            search_key = input("Search for a course (blank will search all courses): ")
            matches = used_courses.search_list(search_key)

            if len(matches) == 0:
                print("Error: No matches found.")
                continue

            for key, match in matches.items(): print(f"{key}: {match}")

            try:
                key_selection = input("Enter the key associated with the course you want to add ('all' to add all matches): ").lower()

                if key_selection == "all":
                    for match in matches.values(): active_course_list.add_course(match)
                    continue

                index = int(key_selection)
                active_course_list.add_course(matches[index])

            except ValueError:
                print("Error: Not a number")
            except KeyError:
                print("Error: Invalid key")
            finally:
                continue


        case "remove":
            search_key = input("Search for a course (blank will search all courses): ")
            matches = active_course_list.search_list(search_key)

            if len(matches) == 0:
                print("Error: No matches found.")
                continue

            for key, match in matches.items(): print(f"{key}: {match}")

            try:
                key_selection = input("Enter the key associated with the course you want to remove ('all' to remove all matches): ").lower()

                if key_selection == "all":
                    for match in matches.values(): active_course_list.remove_course(match)
                    continue

                index = int(key_selection)
                active_course_list.remove_course(matches[index])

            except ValueError:
                print("Error: Not a number")
            except KeyError:
                print("Error: Invalid key")
            finally:
                continue


        case "":
            active_course_list.generate_course()
            continue


    print("Error: Invalid input.")


# def make_tiered_list():
#
#     if len(course_list) == 0:
#         print("The course list is empty. Resetting.")
#         reset_course_list()
#
#     user_input = input("Enter the size of the prix: 4/6/8/12/16/24/32/48: ")
#     if user_input not in ['4', '6', '8', '12', '16', '24', '32', '48']:
#         print("Error: Invalid prix size. Resuming normal generation.")
#     prix_size = int(user_input)
#
#     chunked_list = chunk(prix_size)
#     tiered_list = []
#
#     for sublist in chunked_list:
#         index = random.randrange(len(sublist))
#         tiered_list.append(sublist[index])
#
#     return tiered_list
#
#
# def generate_tiered_course():
#     global old_list
#     global course_list
#
#     old_list = course_list.copy()
#     index = random.randrange(len(tiered_list))
#     course = tiered_list.pop(index)
#     course_list.remove(course)
#     print(f"({course[1]}) {course[2]}")
#     overwrite_list(course_list)
#
#
#
#         case "tier":
#             try:
#                 if is_tiered:
#                     user_input = input("You are already using a tiered list. Resume normal generation? 'y' to confirm: ")
#                     if user_input == "y":
#                         tiered_list = []
#                         is_tiered = False
#                         continue
#                     print("Continuing with tiered generation.")
#                     continue
#
#                 tiered_list = make_tiered_list()
#                 print("You are now using a tiered list.")
#                 is_tiered = True
#
#             except ChunkSizeError:
#                 print("Error: Remaining courses cannot be evenly divided by that number")
#             finally:
#                 continue
#
#         case "":
#             try:
#                 if is_tiered:
#                     if len(tiered_list) == 0:
#                         user_input = input("The tiered list is empty. Make a new one? 'y' to confirm: ")
#                         if user_input == "y":
#                             tiered_list = make_tiered_list()
#                             continue
#
#                         print("Resuming normal generation.")
#                         is_tiered = False
#                         continue
#
#                     generate_tiered_course()
#                     continue
#
#                 generate_normal_course()
#
#             except ValueError:
#                 print("The course list is empty. Resetting.")
#                 reset_course_list()
#             finally:
#                 continue
