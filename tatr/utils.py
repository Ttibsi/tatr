import os
from datetime import datetime

def generate_huid() -> str:
    return datetime.strftime(datetime.now(), "%Y%m%d-%H%M%S")

def get_tasks_dir() -> str | None:
    path = os.getcwd()

    while not os.path.exists(f"{path}/tasks"):
        if not path:
            return None
        path = os.path.join(path.split("/")[:-1])

    return f"{path}/tasks"




