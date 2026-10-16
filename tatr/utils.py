import os
from datetime import datetime


def generate_huid() -> str:
    return datetime.strftime(datetime.now(), "%Y%m%d-%H%M%S")


def get_tasks_dir() -> str | None:
    path: str = os.getcwd()

    while not os.path.exists(f"{path}/tasks"):
        if not path:
            return None
        path_pieces: list[str] = path.split("/")[:-1]

        if len(path_pieces) == 1:
            maybe_task_path = f"{path_pieces[0]}/tasks"
            return maybe_task_path if os.path.exists(maybe_task_path) else None

        path = os.path.join(*path_pieces)

    return f"{path}/tasks"
