import random, os, json

full_course_list = [
    # Coordinate (row cup course), rank, name
    [314, 1, "Wii Coconut Mall"],
    [354, 2, "Wii Maple Treeway"],
    [241, 3, "DS Tick Tock Clock"],
    [434, 4, "MK8 Squeaky Clean Sprint"],
    [211, 5, "Wii Moo Moo Meadows"],
    [362, 6, "DS Peach Gardens"],
    [433, 7, "Wii Moonview Highway"],
    [413, 8, "Wii DK Summit"],
    [253, 9, "MK8 Ice Ice Outpost"],
    [334, 10, "DS Waluigi Pinball"],
    [233, 11, "3DS Music Park"],
    [344, 12, "MK8 Sky-High Sundae"],
    [232, 13, "GCN Sherbet Land"],
    [464, 14, "Wii Rainbow Road"],
    [324, 15, "Tour Ninja Hideaway"],
    [353, 16, "3DS Rock Rock Mountain"],
    [131, 17, "MK8 Sunshine Airport"],
    [234, 18, "N64 Yoshi Valley"],
    [343, 19, "Wii Mushroom Gorge"],
    [244, 20, "N64 Rainbow Road"],
    [443, 21, "Wii Koopa Cape"],
    [322, 22, "DS Shroom Ridge"],
    [214, 23, "N64 Toad's Turnpike"],
    [134, 24, "MK8 Mount Wario"],
    [141, 25, "MK8 Cloudtop Cruise"],
    [432, 26, "GCN Daisy Cruiser"],
    [163, 27, "MK8 Wild Woods"],
    [364, 28, "3DS Rainbow Road"],
    [333, 29, "N64 Kalimari Desert"],
    [323, 30, "GBA Sky Garden"],
    [462, 31, "3DS Rosalina's Ice World"],
    [414, 32, "MK8 Yoshi's Island"],
    [453, 33, "Wii Daisy Circuit"],
    [122, 34, "MK8 Toad Harbor"],
    [264, 35, "MK8 Big Blue"],
    [154, 36, "MK8 Mute City"],
    [251, 37, "Wii Wario's Gold Mine"],
    [452, 38, "GCN DK Mountain"],
    [263, 39, "MK8 Super Bell Subway"],
    [114, 40, "MK8 Thwomp Ruins"],
    [143, 41, "MK8 Bowser's Castle"],
    [213, 42, "DS Cheep Cheep Beach"],
    [254, 43, "MK8 Hyrule Circuit"],
    [312, 44, "3DS Toad Circuit"],
    [243, 45, "Wii Grumble Volcano"],
    [153, 46, "MK8 Dragon Driftway"],
    [454, 47, "MK8 Piranha Plant Cove"],
    [144, 48, "MK8 Rainbow Road"],
    [242, 49, "3DS Piranha Plant Slide"],
    [224, 50, "3DS DK Jungle"],
    [132, 51, "MK8 Dolphin Shoals"],
    [423, 52, "GCN Waluigi Stadium"],
    [112, 53, "MK8 Water Park"],
    [223, 54, "N64 Royal Raceway"],
    [421, 55, "Tour Bangkok Rush"],
    [123, 56, "MK8 Twisted Mansion"],
    [342, 57, "GBA Snow Land"],
    [262, 58, "GBA Ribbon Road"],
    [152, 59, "MK8 Excitebike Arena"],
    [461, 60, "Tour Madrid Drive"],
    [424, 61, "Tour Singapore Speedway"],
    [111, 62, "MK8 Mario Kart Stadium"],
    [221, 63, "GCN Dry Dry Desert"],
    [363, 64, "Tour Merry Mountain"],
    [113, 65, "MK8 Sweet Sweet Canyon"],
    [133, 66, "MK8 Electrodrome"],
    [331, 67, "Tour New York Minute"],
    [164, 68, "MK8 Animal Crossing Circuit"],
    [442, 69, "GBA Sunset Wilds"],
    [411, 70, "Tour Amsterdam Drift"],
    [431, 71, "Tour Athens Dash"],
    [124, 72, "MK8 Shy Guy Falls"],
    [121, 73, "MK8 Mario Circuit"],
    [212, 74, "GBA Mario Circuit"],
    [361, 75, "Tour Berlin Byways"],
    [412, 76, "GBA Riverside Park"],
    [252, 77, "SNES Rainbow Road"],
    [332, 78, "SNES Mario Circuit 3"],
    [422, 79, "DS Mario Circuit"],
    [311, 80, "Tour Paris Promenade"],
    [313, 81, "N64 Choco Mountain"],
    [222, 82, "SNES Donut Plains 3"],
    [441, 83, "Tour Los Angeles Laps"],
    [451, 84, "Tour Rome Avanti"],
    [231, 85, "DS Wario Stadium"],
    [161, 86, "GCN Baby Park"],
    [142, 87, "MK8 Bone Dry Ruins"],
    [444, 88, "Tour Vancouver Velocity"],
    [351, 89, "Tour London Loop"],
    [261, 90, "3DS Neo Bowser City"],
    [151, 91, "GCN Yoshi's Circuit"],
    [321, 92, "Tour Tokyo Blur"],
    [341, 93, "Tour Sydney Sprint"],
    [162, 94, "GBA Cheese Land"],
    [463, 95, "SNES Bowser's Castle 3"],
    [352, 96, "GBA Boo Lake"],
]

list_file = os.path.join(os.path.dirname(os.path.realpath(__file__)), 'list.json')


def overwrite_list(course_list):
    with open(list_file, 'w') as wfile:
        json.dump(course_list, wfile, indent=2)


def read_list():
    with open(list_file, 'r') as rfile:
        course_list = json.load(rfile)
        return course_list


def chunk(course_list, prix_size):
    chunked = []
    length = len(course_list)
    chunk_size = length // prix_size

    if chunk_size != length / prix_size:
        print("Error: Remaining courses cannot be evenly divided into given prix size")
        return

    for i in range(0, length, chunk_size):
        chunked.append(course_list[i:i+chunk_size])

    return chunked


def get_used_courses():
    return sorted(list(set(course_list).symmetric_difference(set(full_course_list))))


def print_remaining_courses():
    for course in sorted(course_list, key=lambda x: x[0]):
        print(course)
    print(f"{len(course_list)} courses remaining.")


def print_used_courses():
    used_course_list = get_used_courses()
    for course in sorted(used_course_list, key=lambda x: x[0]):
        print(course)
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

    for key, value in matches.items():
        print(f"{key}: {value}")

    try:
        user_input = input("Enter the key associated with the course you want to remove ('all' to add all matches): ").lower()

        if user_input == "all":
            for match in matches.values():
                course_list.append(match)
            course_list.sort(key=lambda x: x[1])
            overwrite_list(course_list)

        index = int(user_input)
        course_list.append(matches[index])
        course_list.sort(key=lambda x: x[1])
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
    for course in course_list:
        if user_input in course[2].lower():
            matches[i] = course
            i += 1

    if len(matches) == 0:
        print("Error: No matches found.")
        return

    for key, match in matches.items():
        print(f"{key}: {match}")

    try:
        user_input = input("Enter the key associated with the course you want to remove ('all' to remove all matches): ").lower()

        if user_input == "all":
            for match in matches.values():
                course_list.remove(match)
            overwrite_list(course_list)
            course_list.sort(key=lambda x: x[1])
            return course_list

        index = int(user_input)
        course_list.remove(matches[index])
        course_list.sort(key=lambda x: x[1])
        overwrite_list(course_list)
        return course_list

    except ValueError:
        print("Error: Not a number")
    except KeyError:
        print("Error: Invalid key")


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

    while len(tiered_list) > 0:
        old_list = course_list.copy()
        course = tiered_list[random.randrange(len(tiered_list))]
        course_list.remove(course)
        print(course)
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
    print(course)
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
            generate_tiered_course()
            continue

        case "":
            # FIXME len 0 error
            generate_normal_course()
            continue

    if len(course_list) == 0:
        print("The course list is empty. Resetting.")
        reset_course_list()
        continue

    print("Error: Invalid input")
