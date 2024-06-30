import random, os, json

full_course_list = [
    [1,  314, "Wii  Coconut Mall"],
    [2,  354, "Wii  Maple Treeway"],
    [3,  241, "DS   Tick Tock Clock"],
    [4,  434, "MK8  Squeaky Clean Sprint"],
    [5,  211, "Wii  Moo Moo Meadows"],
    [6,  362, "DS   Peach Gardens"],
    [7,  433, "Wii  Moonview Highway"],
    [8,  413, "Wii  DK Summit"],
    [9,  253, "MK8  Ice Ice Outpost"],
    [10, 334, "DS   Waluigi Pinball"],
    [11, 233, "3DS  Music Park"],
    [12, 344, "MK8  Sky-High Sundae"],
    [13, 232, "GCN  Sherbet Land"],
    [14, 464, "Wii  Rainbow Road"],
    [15, 324, "Tour Ninja Hideaway"],
    [16, 353, "3DS  Rock Rock Mountain"],
    [17, 131, "MK8  Sunshine Airport"],
    [18, 234, "N64  Yoshi Valley"],
    [19, 343, "Wii  Mushroom Gorge"],
    [20, 244, "N64  Rainbow Road"],
    [21, 443, "Wii  Koopa Cape"],
    [22, 322, "DS   Shroom Ridge"],
    [23, 214, "N64  Toad's Turnpike"],
    [24, 134, "MK8  Mount Wario"],
    [25, 141, "MK8  Cloudtop Cruise"],
    [26, 432, "GCN  Daisy Cruiser"],
    [27, 163, "MK8  Wild Woods"],
    [28, 364, "3DS  Rainbow Road"],
    [29, 333, "N64  Kalimari Desert"],
    [30, 323, "GBA  Sky Garden"],
    [31, 462, "3DS  Rosalina's Ice World"],
    [32, 414, "MK8  Yoshi's Island"],
    [33, 453, "Wii  Daisy Circuit"],
    [34, 122, "MK8  Toad Harbor"],
    [35, 264, "MK8  Big Blue"],
    [36, 154, "MK8  Mute City"],
    [37, 251, "Wii  Wario's Gold Mine"],
    [38, 452, "GCN  DK Mountain"],
    [39, 263, "MK8  Super Bell Subway"],
    [40, 114, "MK8  Thwomp Ruins"],
    [41, 143, "MK8  Bowser's Castle"],
    [42, 213, "DS   Cheep Cheep Beach"],
    [43, 254, "MK8  Hyrule Circuit"],
    [44, 312, "3DS  Toad Circuit"],
    [45, 243, "Wii  Grumble Volcano"],
    [46, 153, "MK8  Dragon Driftway"],
    [47, 454, "MK8  Piranha Plant Cove"],
    [48, 144, "MK8  Rainbow Road"],
    [49, 242, "3DS  Piranha Plant Slide"],
    [50, 224, "3DS  DK Jungle"],
    [51, 132, "MK8  Dolphin Shoals"],
    [52, 423, "GCN  Waluigi Stadium"],
    [53, 112, "MK8  Water Park"],
    [54, 223, "N64  Royal Raceway"],
    [55, 421, "Tour Bangkok Rush"],
    [56, 123, "MK8  Twisted Mansion"],
    [57, 342, "GBA  Snow Land"],
    [58, 262, "GBA  Ribbon Road"],
    [59, 152, "MK8  Excitebike Arena"],
    [60, 461, "Tour Madrid Drive"],
    [61, 424, "Tour Singapore Speedway"],
    [62, 111, "MK8  Mario Kart Stadium"],
    [63, 221, "GCN  Dry Dry Desert"],
    [64, 363, "Tour Merry Mountain"],
    [65, 113, "MK8  Sweet Sweet Canyon"],
    [66, 133, "MK8  Electrodrome"],
    [67, 331, "Tour New York Minute"],
    [68, 164, "MK8  Animal Crossing Circuit"],
    [69, 442, "GBA  Sunset Wilds"],
    [70, 411, "Tour Amsterdam Drift"],
    [71, 431, "Tour Athens Dash"],
    [72, 124, "MK8  Shy Guy Falls"],
    [73, 121, "MK8  Mario Circuit"],
    [74, 212, "GBA  Mario Circuit"],
    [75, 361, "Tour Berlin Byways"],
    [76, 412, "GBA  Riverside Park"],
    [77, 252, "SNES Rainbow Road"],
    [78, 332, "SNES Mario Circuit 3"],
    [79, 422, "DS   Mario Circuit"],
    [80, 311, "Tour Paris Promenade"],
    [81, 313, "N64  Choco Mountain"],
    [82, 222, "SNES Donut Plains 3"],
    [83, 441, "Tour Los Angeles Laps"],
    [84, 451, "Tour Rome Avanti"],
    [85, 231, "DS   Wario Stadium"],
    [86, 161, "GCN  Baby Park"],
    [87, 142, "MK8  Bone Dry Ruins"],
    [88, 444, "Tour Vancouver Velocity"],
    [89, 351, "Tour London Loop"],
    [90, 261, "3DS  Neo Bowser City"],
    [91, 151, "GCN  Yoshi's Circuit"],
    [92, 321, "Tour Tokyo Blur"],
    [93, 341, "Tour Sydney Sprint"],
    [94, 162, "GBA  Cheese Land"],
    [95, 463, "SNES Bowser's Castle 3"],
    [96, 352, "GBA  Boo Lake"],
]

list_file = os.path.join(os.path.dirname(os.path.realpath(__file__)), 'list.json')


class ChunkSizeError(Exception):
    pass


def overwrite_list(course_list):
    with open(list_file, 'w') as wfile:
        json.dump(course_list, wfile, indent=2)


def read_list():
    with open(list_file, 'r') as rfile:
        course_list = json.load(rfile)
        return course_list


def get_used_courses():
    used_courses = []
    symmetric_difference = set(course[0] for course in course_list) ^ set(course[0] for course in full_course_list)

    for course in full_course_list:
        if course[0] in symmetric_difference:
            used_courses.append(course)

    used_courses.sort(key=lambda x: x[0])
    return used_courses


def print_remaining_courses():
    for course in sorted(course_list, key=lambda x: x[1]):
        print(f"({course[1]}) {course[2]}")
    print(f"{len(course_list)} courses remaining.")


def print_used_courses():
    used_course_list = get_used_courses()
    for course in sorted(used_course_list, key=lambda x: x[1]):
        print(f"({course[1]}) {course[2]}")
    print(f"{96-len(course_list)} courses used.")


def reset_course_list():
    global old_list
    global course_list

    old_list = course_list.copy()
    overwrite_list(full_course_list)
    course_list = full_course_list.copy()


def undo():
    global old_list
    global course_list

    user_input = input("Are you sure? 'y' to confirm: ").lower()
    if user_input == "y":
        try:
            old_list_temp = old_list.copy()
            old_list = course_list
            overwrite_list(old_list_temp)
            course_list = old_list_temp
        except:
            print("Error: You probably haven't done an undoable action.")


def add_course():
    global old_list
    global course_list

    old_list = course_list.copy()
    used_course_list = get_used_courses()
    matches = {}
    i = 1

    user_input = input("Search for a course (blank will search all used courses): ").lower()
    for course in used_course_list:
        if user_input in course[2].lower():
            matches[i] = course
            i += 1

    if len(matches) == 0:
        print("Error: No matches found.")
        return

    for key, match in matches.items():
        print(f"{key}: ({match[1]}) {match[2]}")

    try:
        user_input = input("Enter the key associated with the course you want to remove ('all' to add all matches): ").lower()

        if user_input == "all":
            for match in matches.values():
                course_list.append(match)
            course_list.sort(key=lambda x: x[0])
            overwrite_list(course_list)

        index = int(user_input)
        course_list.append(matches[index])
        course_list.sort(key=lambda x: x[0])
        overwrite_list(course_list)

    except ValueError:
        print("Error: Not a number")
    except KeyError:
        print("Error: Invalid key")


def remove_course():
    global old_list
    global course_list

    old_list = course_list.copy()
    matches = {}
    i = 1

    user_input = input("Search for a course (blank will search all used courses): ").lower()
    for course in sorted(course_list, key=lambda x: x[1]):
        if user_input in course[2].lower():
            matches[i] = course
            i += 1

    if len(matches) == 0:
        print("Error: No matches found.")
        return

    for key, match in matches.items():
        print(f"{key}: ({match[1]}) {match[2]}")

    try:
        user_input = input("Enter the key associated with the course you want to remove ('all' to remove all matches): ").lower()

        if user_input == "all":
            for match in matches.values():
                course_list.remove(match)
            overwrite_list(course_list)
            course_list.sort(key=lambda x: x[0])
            return course_list

        index = int(user_input)
        course_list.remove(matches[index])
        course_list.sort(key=lambda x: x[0])
        overwrite_list(course_list)
        return course_list

    except ValueError:
        print("Error: Not a number")
    except KeyError:
        print("Error: Invalid key")


def chunk(course_list, prix_size):
    chunked = []
    length = len(course_list)
    chunk_size = length // prix_size

    if chunk_size != length / prix_size:
        raise ChunkSizeError

    for i in range(0, length, chunk_size):
        chunked.append(course_list[i:i+chunk_size])

    return chunked


def make_tiered_list(course_list, prix_size):
    chunked_list = chunk(course_list, prix_size)
    tiered_list = []

    for sublist in chunked_list:
        index = random.randrange(len(sublist))
        tiered_list.append(sublist[index])

    return tiered_list


def generate_tiered_course():
    global old_list
    global course_list

    user_input = input("Enter the size of the prix: 4/6/8/12/16/24/32/48: ")
    if user_input not in ['4', '6', '8', '12', '16', '24', '32', '48']:
        print("Error: Invalid prix size. Resuming normal generation.")
        return

    prix_size = int(user_input)
    tiered_list = make_tiered_list(course_list, prix_size)
    print(tiered_list)

    while len(tiered_list) > 0:
        old_list = course_list.copy()
        course = tiered_list[random.randrange(len(tiered_list))]
        course_list.remove(course)
        print(f"({course[1]}) {course[2]}")
        overwrite_list(course_list)

    user_input = input("Tiered list empty. Make a new one? 'y' to confirm: ").lower()
    if user_input == "y":
        generate_tiered_course()

    print("Resuming normal generation.")
    return


def generate_normal_course():
    global old_list
    global course_list

    old_list = course_list.copy()
    index = random.randrange(len(course_list))
    course = course_list.pop(index)
    print(f"({course[1]}) {course[2]}")
    overwrite_list(course_list)


try:
    course_list = read_list()
except:
    overwrite_list(full_course_list)
    course_list = full_course_list.copy()


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
  tiered: Pick a prix size N. Evenly split the list into tiers
          of size N. Pick one random course from each tier
          removing that course from the FULL list as you go.
          After drawing one course from each tier, return to
          standard generation (will notify when this happens).
'''


print("Enter 'help' for commands.")
while True:
    user_input = input(":> ").lower()

    if user_input in ['q', 'quit', 'exit']:
        break

    match user_input:
        case "help":
            print(help_block)
            continue

        case "remaining":
            print_remaining_courses()
            continue

        case "used":
            print_used_courses()
            continue

        case "reset":
            reset_course_list()
            print("Course list reset.")
            continue

        case "undo":
            undo()
            continue

        case "add":
            add_course()
            continue

        case "remove":
            remove_course()
            continue

        case "tiered":
            try:
                generate_tiered_course()
            except ChunkSizeError:
                print("Error: Remaining courses cannot be evenly divided by that number")
            finally:
                continue

        case "":
            try:
                generate_normal_course()
            except ValueError:
                print("The course list is empty. Resetting.")
                reset_course_list()
            finally:
                continue


    print("Error: Invalid input")
