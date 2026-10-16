import contextlib
import os

import freezegun

import tatr.utils


@freezegun.freeze_time("2026-10-06")
def test_generate_huid():
    assert tatr.utils.generate_huid() == "20261006-000000"


def test_get_tasks_dir():
    with contextlib.chdir(os.getcwd() + os.sep.join(["", "tests", "fixture"])):
        actual = tatr.utils.get_tasks_dir()
        assert actual is not None

        pieces = actual.split(os.sep)
        assert pieces[-1] == "tasks"
        assert pieces[-2] == "fixture"
