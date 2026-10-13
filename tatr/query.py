import os
from tatr.utils import get_tasks_dir

def get_tasks() -> list[str]:
    tasks_path: str | None = get_tasks_dir()
    if tasks_path is None:
        print("No tasks dir found")
        return []

    tasks = [f"{tasks_path}{os.sep}{x}" for x in os.listdir(tasks_path)]
    return tasks


def get_title(task: str) -> str:
    with open(f"{task}/TASK.md", "r") as f:
        return f.readlines()[0].strip()[2:]


def get_status(task: str) -> str:
    with open(f"{task}/TASK.md", "r") as f:
        return f.readlines()[2].split(":")[1].strip()


def get_priority(task: str) -> int:
    with open(f"{task}/TASK.md", "r") as f:
        return int(f.readlines()[3].split(":")[1].strip())


def get_tags(task: str) -> str:
    with open(f"{task}/TASK.md", "r") as f:
        tagstr = f.readlines()[4].split(":")[1]
        if tagstr.isspace() or not tagstr:
            return ""

        return "[" + ",".join([x.strip() for x in tagstr.split(",")]) + "] "


# valid code: gt, ge, lt, le, eq, ne
# ex: gt 50 le 100
# TODO: support `and`, `or`
def check_priority(task: str, priority_str: str) -> bool:
    chunks = priority_str.split(" ")
    task_prio = get_priority(task)

    valid = False
    skip = False
    for idx, chunk in enumerate(chunks):
        if skip:
            continue

        if chunk == "eq":
            valid = int(chunks[idx + 1]) == task_prio
            break
        elif chunk == "ne":
            valid = int(chunks[idx + 1]) != task_prio
            break
        elif chunk == "ge":
            valid = task_prio >= int(chunks[idx + 1])
            skip = True
        elif chunk == "gt":
            valid = task_prio > int(chunks[idx + 1])
            skip = True
        elif chunk == "le":
            valid = task_prio <= int(chunks[idx + 1])
            skip = True
        elif chunk == "lt":
            valid = task_prio < int(chunks[idx + 1])
            skip = True

    return valid 


def has_tags(task: str, tags: set[str]) -> bool:
    with open(f"{task}/TASK.md", "r") as f:
        tagstr = f.readlines()[4].split(":")[1].strip()

    present_tags = set([tag.strip() for tag in tagstr.split(",")])
    if not present_tags:
        return False

    return tags == present_tags.intersection(tags)


def has_no_tag(task: str) -> bool:
    with open(f"{task}/TASK.md", "r") as f:
        return not bool(f.readlines()[4].split(":")[1].strip())

