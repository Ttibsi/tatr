import argparse
import curses
import os
from tatr.utils import generate_huid, get_tasks_dir
from tatr.query import get_title, get_priority, get_tags, get_status, has_tags, check_priority, has_no_tag, get_tasks
from collections import Counter
from tatr.tui import c_main

def init_tasks_dir(args: argparse.Namespace):
    if not os.path.exists("tasks"):
        os.mkdir("tasks")


def new_cmd(args: argparse.Namespace):
    huid: str = generate_huid()
    tasks_path: str | None = get_tasks_dir()
    if tasks_path is None:
        print("No tasks directory found. Creating tasks directory")
        init_tasks_dir(args)

    try:
        os.mkdir(f"{tasks_path}/{huid}")
    except FileExistsError:
        print("Task already exists, please wait one second and try again")
        return

    with open(f"{tasks_path}/{huid}/TASK.md", "w") as f:
        f.write(f"# {args.title}\n\n")
        f.write("- STATUS: OPEN\n")
        f.write(f"- PRIORITY: {args.priority}\n")
        f.write(f"- TAGS: {args.tags if args.tags else ''}\n")

    print(f"{tasks_path}/{huid}/TASK.md:1 {args.title}")


# TODO: Only show open tasks unless `-c` is passed to show closed tasks instead
def ls_cmd(args: argparse.Namespace) -> list[str]:
    tasks = get_tasks()
    if not tasks:
        return tasks

    if args.priority:
        temp_tasks: list[str] = []

        for task in tasks:
            if check_priority(task, args.priority):
                temp_tasks.append(task)

        tasks = temp_tasks

    if args.tags:
        temp_tasks: list[str] = []
        tags = [tag.strip() for tag in args.tags.split(",")]
        for task in tasks:
            if has_tags(task, set(args.tags.split(","))):
                temp_tasks.append(task)

        tasks = temp_tasks
    elif args.untagged:
        temp_tasks: list[str] = []
        for task in tasks:
            if has_no_tag(task):
                temp_tasks.append(task)

        tasks = temp_tasks


    for task in tasks:
        title = get_title(task)
        status = get_status(task)
        prio = f"[PRIORITY: {get_priority(task)}]"
        tags = get_tags(task)
        print(f"{task}/TASK.md:1:{status} {prio} {tags}{title}")

    return tasks


def summary_cmd(args: argparse.Namespace) -> dict[str, int | dict[str, int]]:
    tasks = get_tasks()
    if not tasks:
        return {}

    status = {
        "total": len(tasks),
        "open": 0,
        "untagged": 0,
        "tags": Counter()
    }

    for task in tasks:
        if get_status(task) == "OPEN":
            status["open"] += 1

        tags = [t.strip().lstrip().strip("]").lstrip("[") for t in get_tags(task).split(",")]
        if len(tags) == 1 and not tags[0]:
            status["untagged"] += 1
            continue

        for tag in tags:
            status["tags"][tag] += 1

    print(f"TOTAL:    {status['total']}")
    print(f"OPEN:     {status['open']}")
    print(f"UNTAGGED: {status['untagged']}")
    print(f"TAGGED:")
    for key, val in status["tags"].items():
        print(f"      {key} => {val}")

    return status

