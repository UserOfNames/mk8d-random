import random, os, json

full_course_list = [
    "(111) MK8 Mario Stadium",
    "(112) MK8 Water Park",
    "(113) MK8 Sweet Sweet Canyon",
    "(114) MK8 Thwomp Ruins",
    "(121) MK8 Mario Circuit",
    "(122) MK8 Toad Harbor",
    "(123) MK8 Twisted Mansion",
    "(124) MK8 Shy Guy Falls",
    "(131) MK8 Sunshine Airport",
    "(132) MK8 Dolphin Shoals",
    "(133) MK8 Electrodrome",
    "(134) MK8 Mount Wario",
    "(141) MK8 Cloudtop Cruise",
    "(142) MK8 Bone Dry Ruins",
    "(143) MK8 Bowser's Castle",
    "(144) MK8 Rainbow Road",
    "(151) GCN Yoshi's Circuit",
    "(152) MK8 Excitebike Arena",
    "(153) MK8 Dragon Driftway",
    "(154) MK8 Mute City",
    "(161) GCN Baby Park",
    "(162) GBA Cheese Land",
    "(163) MK8 Wild Woods",
    "(164) MK8 Animal Crossing Circuit",
    "(211) Wii Moo Moo Meadows",
    "(212) GBA Mario Circuit",
    "(213) DS Cheep Cheep Beach",
    "(214) N64 Toad's Turnpike",
    "(221) GCN Dry Dry Desert",
    "(222) SNES Donut Plains 3",
    "(223) N64 Royal Raceway",
    "(224) 3DS DK Jungle",
    "(231) DS Wario Stadium",
    "(232) GCN Sherbet Land",
    "(233) 3DS Music Park",
    "(234) N64 Yoshi Valley",
    "(241) DS Tick Tock Clock",
    "(242) 3DS Piranha Plant Slide",
    "(243) Wii Grumble Volcano",
    "(244) N64 Rainbow Road",
    "(251) Wii Wario's Gold Mine",
    "(252) SNES Rainbow Road",
    "(253) MK8 Ice Ice Outpost",
    "(254) MK8 Hyrule Circuit",
    "(261) 3DS Neo Bowser City",
    "(262) GBA Ribbon Road",
    "(263) MK8 Super Bell Subway",
    "(264) MK8 Big Blue",
    "(311) Tour Paris Promenade",
    "(312) 3DS Toad Circuit",
    "(313) N64 Choco Mountain",
    "(314) Wii Coconut Mall",
    "(321) Tour Tokyo Blur",
    "(322) DS Shroom Ridge",
    "(323) GBA Sky Garden",
    "(324) Tour Ninja Hideaway",
    "(331) Tour New York Minute",
    "(332) SNES Mario Circuit 3",
    "(333) N64 Kalimari Desert",
    "(334) DS Waluigi Pinball",
    "(341) Tour Sydney Sprint",
    "(342) GBA Snow Land",
    "(343) Wii Mushroom Gorge",
    "(344) MK8 Sky-High Sundae",
    "(351) Tour London Loop",
    "(352) GBA Boo Lake",
    "(353) 3DS Rock Rock Mountain",
    "(354) Wii Maple Treeway",
    "(361) Tour Berlin Byways",
    "(362) DS Peach Gardens",
    "(363) Tour Merry Mountain",
    "(364) 3DS Rainbow Road",
    "(411) Tour Amsterdam Drift",
    "(412) GBA Riverside Park",
    "(413) Wii DK Summit",
    "(414) MK8 Yoshi's Island",
    "(421) Tour Bangkok Rush",
    "(422) DS Mario Circuit",
    "(423) GCN Waluigi Stadium",
    "(424) Tour Singapore Speedway",
    "(431) Tour Athens Dash",
    "(432) GCN Daisy Cruiser",
    "(433) Wii Moonview Highway",
    "(434) MK8 Squeaky Clean Sprint",
    "(441) Tour Los Angeles Laps",
    "(442) GBA Sunset Wilds",
    "(443) Wii Koopa Cape",
    "(444) Tour Vancouver Velocity",
    "(451) Tour Rome Avanti",
    "(452) GCN DK Mountain",
    "(453) Wii Daisy Circuit",
    "(454) MK8 Piranha Plant Cove",
    "(461) Tour Madrid Drive",
    "(462) 3DS Rosalina's Ice World",
    "(463) SNES Bowser's Castle 3",
    "(464) Wii Rainbow Road",
]

list_file = os.path.join(os.path.dirname(os.path.realpath(__file__)), 'list.json')


def overwrite_list(course_list):
    with open(list_file, 'w') as wfile:
        json.dump(course_list, wfile, indent=2)


def read_list():
    with open(list_file, 'r') as rfile:
        course_list = json.load(rfile)
        return course_list


def get_used_courses():
    return sorted(list(set(course_list).symmetric_difference(set(full_course_list))))


def remaining():
    for course in course_list:
        print(course)
    print(f"{len(course_list)} courses remaining.")


def used():
    used_course_list = get_used_courses()
    for course in used_course_list:
        print(course)
    print(f"{96-len(course_list)} courses used.")


def reset(course_list):
    global old_list
    old_list = course_list.copy()
    overwrite_list(full_course_list)
    return full_course_list.copy()


def undo(course_list):
    global old_list
    user_input = input("Are you sure? 'y' to confirm: ").lower()
    if user_input == "y":
        try:
            old_list_temp = old_list.copy()
            old_list = course_list
            overwrite_list(old_list_temp)
            return old_list_temp
        except:
            print("Error: You probably haven't done an undoable action.")
            return course_list
    return course_list


def add(course_list):
    global old_list
    old_list = course_list.copy()
    used_course_list = get_used_courses()
    matches, i = {}, 1

    user_input = input("Search for a course (blank will search all used courses): ").lower()
    for course in used_course_list:
        if user_input in course.lower():
            matches[i] = course
            i += 1

    if len(matches) == 0:
        print("Error: No matches found.")
        return course_list

    for key, value in matches.items():
        print(f"{key}: {value}")

    try:
        user_input = input("Enter the key associated with the course you want to remove ('all' to add all matches): ").lower()
        if user_input == "all":
            for match in matches.values():
                course_list.append(match)
            course_list.sort()
            overwrite_list(course_list)
            return course_list

        index = int(user_input)
        course_list.append(matches[index])
        course_list.sort()
        overwrite_list(course_list)
        return course_list

    except ValueError:
        print("Error: Not a number")
        return course_list
    except KeyError:
        print("Error: Invalid key")
        return course_list


def remove(course_list):
    global old_list
    old_list = course_list.copy()
    matches, i = {}, 1

    user_input = input("Search for a course (blank will search all used courses): ").lower()
    for course in course_list:
        if user_input in course.lower():
            matches[i] = course
            i += 1

    if len(matches) == 0:
        print("Error: No matches found.")
        return course_list

    for key, match in matches.items():
        print(f"{key}: {match}")

    try:
        user_input = input("Enter the key associated with the course you want to remove ('all' to remove all matches): ").lower()
        if user_input == "all":
            for match in matches.values():
                course_list.remove(match)
            overwrite_list(course_list)
            course_list.sort()
            return course_list

        index = int(user_input)
        course_list.remove(matches[index])
        course_list.sort()
        overwrite_list(course_list)
        return course_list

    except ValueError:
        print("Error: Not a number")
        return course_list
    except KeyError:
        print("Error: Invalid key")
        return course_list


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
            remaining()
            continue

        case "used":
            used()
            continue

        case "reset":
            course_list = reset(course_list)
            print("Course list reset.")
            continue

        case "undo":
            course_list = undo(course_list)
            continue

        case "add":
            course_list = add(course_list)
            continue

        case "remove":
            course_list = remove(course_list)
            continue

    if len(course_list) == 0:
        print("The course list is empty. Resetting.")
        overwrite_list(full_course_list)
        course_list = full_course_list.copy()
        continue
    
    if user_input == "":
        old_list = course_list.copy()
        index = random.randrange(len(course_list))
        course = course_list.pop(index)
        print(course)
        overwrite_list(course_list)
        continue

    print("Error: Invalid input")
