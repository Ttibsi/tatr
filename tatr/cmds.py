import argparse
import curses
import os
from collections import Counter

from tatr.query import check_priority
from tatr.query import get_priority
from tatr.query import get_status
from tatr.query import get_tags
from tatr.query import get_tasks
from tatr.query import get_title
from tatr.query import has_no_tag
from tatr.query import has_tags
from tatr.tui import c_main
from tatr.utils import generate_huid
from tatr.utils import get_tasks_dir


def init_tasks_dir(args: argparse.Namespace) -> None:
    if not os.path.exists("tasks"):
        os.mkdir("tasks")


def new_cmd(args: argparse.Namespace) -> None:
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

    temp_tasks: list[str] = []
    if args.priority:

        for task in tasks:
            if check_priority(task, args.priority):
                temp_tasks.append(task)

        tasks = temp_tasks
        temp_tasks = []

    if args.tags:
        tags: list[str] = [tag.strip() for tag in args.tags.split(",")]
        for task in tasks:
            if has_tags(task, set(tags)):
                temp_tasks.append(task)

        tasks = temp_tasks
        temp_tasks = []

    elif args.untagged:
        for task in tasks:
            if has_no_tag(task):
                temp_tasks.append(task)

        tasks = temp_tasks
        temp_tasks = []

    for task in tasks:
        title = get_title(task)
        status = get_status(task)
        prio = f"[PRIORITY: {get_priority(task)}]"
        tag_display = get_tags(task)
        print(f"{task}/TASK.md:1:{status} {prio} {tag_display}{title}")

    return tasks


type summary_t = tuple[dict[str, int], dict[str, int]]


def summary_cmd(args: argparse.Namespace) -> summary_t:
    tasks = get_tasks()
    if not tasks:
        return ({}, {})

    status: dict[str, int] = {
        "total": len(tasks),
        "open": 0,
        "untagged": 0,
    }
    tag_status: dict[str, int] = Counter()

    for task in tasks:
        if get_status(task) == "OPEN":
            status["open"] += 1

        tags = [
            t.strip().lstrip().strip("]").lstrip("[")
            for t in get_tags(task).split(",")
        ]
        if len(tags) == 1 and not tags[0]:
            status["untagged"] += 1
            continue

        for tag in tags:
            tag_status[tag] += 1

    print(f"TOTAL:    {status['total']}")
    print(f"OPEN:     {status['open']}")
    print(f"UNTAGGED: {status['untagged']}")
    print("TAGGED:")
    for key, val in tag_status.items():
        print(f"      {key} => {val}")

    return status, tag_status


def tui_cmd(args: argparse.Namespace) -> None:
    curses.wrapper(c_main)
