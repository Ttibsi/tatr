import argparse
import tempfile
import os

import tatr.cmds

import pytest

def test_init_tasks_dir():
    curdir = os.getcwd()
    tmpdir = tempfile.mkdtemp()
    os.chdir(tmpdir)
    assert not os.path.exists("tasks")

    noargs = argparse.Namespace()
    tatr.cmds.init_tasks_dir(noargs)

    assert os.path.exists("tasks")
    assert os.path.isdir("tasks")
    os.chdir(curdir)


def test_new_cmd():
    curdir = os.getcwd()
    tmpdir = tempfile.mkdtemp()
    os.chdir(tmpdir)
    os.mkdir("tasks")
    
    args = argparse.Namespace()
    args.title = "filler title"
    args.priority = 50
    args.tags = "one,two"

    tatr.cmds.new_cmd(args)

    assert len(os.listdir("tasks")) == 1
    taskdir = os.listdir("tasks")[0]
    assert len(os.listdir(f"tasks/{taskdir}")) == 1

    with open(f"tasks/{taskdir}/TASK.md", "r") as f:
        lines = f.readlines()

    assert len(lines) == 5
    assert lines[0].strip() == "# filler title"
    assert not lines[1].strip()
    assert lines[2].strip() == "- STATUS: OPEN"
    assert lines[3].strip() == "- PRIORITY: 50"
    assert lines[4].strip() == "- TAGS: one,two"

    os.chdir(curdir)

# TODO: Figure out how to create Namespace objects on the fly
@pytest.mark.parametrize(
    ("namespace", "result_len"),
    (
        ({}, 4),
        ({"untagged": True}, 3),
        ({"tags": "foo"}, 1),
        ({"priority": "ge 50"}, 3),
    ),
)
def test_ls_cmd(namespace, result_len):
    default = {
        "untagged": False,
        "tags": "",
        "priority": "",
    }

    args = argparse.Namespace(**{**default, **namespace})
    tasks = tatr.cmds.ls_cmd(args)
    assert len(tasks) == result_len


def test_summary_cmd():
    os.chdir(os.getcwd() + os.sep.join(["", "tests", "fixture"]))
    noargs = argparse.Namespace()
    status = tatr.cmds.summary_cmd(noargs)

    assert status["total"] == 4
    assert status["open"] == 4
    assert status["untagged"] == 2
    assert status["tags"] == {"foo": 2, "bar": 2}
