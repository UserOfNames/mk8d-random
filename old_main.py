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


def reset_active_list():
    active_course_list.backup_list()
    active_course_list.course_list = full_course_list.course_list.copy()


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
