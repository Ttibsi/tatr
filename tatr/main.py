import argparse
import os
from collections.abc import Sequence

from tatr.cmds import *

def main(argv: Sequence[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(required=True)

    init_parser = subparsers.add_parser("init", help="Create tasks directory")
    init_parser.set_defaults(func=init_tasks_dir)

    new_parser = subparsers.add_parser("new", help="Create new task")
    new_parser.add_argument("title", type=str)
    new_parser.add_argument("-t", "--tags", type=str)
    new_parser.add_argument("-p", "--priority", type=int, default=100)
    new_parser.set_defaults(func=new_cmd)

    ls_parser = subparsers.add_parser("ls", help="Search for tasks")
    ls_parser.add_argument("-p", "--priority", type=str)
    ls_parser.set_defaults(func=ls_cmd)

    tag_subparser = ls_parser.add_mutually_exclusive_group()
    tag_subparser.add_argument("-t", "--tags", type=str)
    tag_subparser.add_argument("-nt", "--untagged", action="store_true")

    summary_parser = subparsers.add_parser("summary", help="Display task summary")
    summary_parser.set_defaults(func=summary_cmd)

    args: argparse.Namespace = parser.parse_args(argv)
    args.func(args)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
