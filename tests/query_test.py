import os

import pytest

import tatr.query

TESTTASK = os.getcwd() + "/tests/fixture/tasks/20261005-180349"


def test_get_title():
    title = tatr.query.get_title(TESTTASK)
    assert title == "temp title"


def test_get_status():
    status = tatr.query.get_status(TESTTASK)
    assert status == "OPEN"


def test_get_priority():
    prio = tatr.query.get_priority(TESTTASK)
    assert prio == 100


def test_get_tags():
    tags = tatr.query.get_tags(TESTTASK)
    assert tags == "[foo,bar] "


@pytest.mark.parametrize(
    "msg", (
        "ge 50",
        "lt 120",
        "eq 100",
        "ne 120",
    ),
)
def test_check_priority(msg):
    assert tatr.query.check_priority(TESTTASK, msg)


def test_has_tags():
    assert tatr.query.has_tags(TESTTASK, {"foo"})
    assert tatr.query.has_tags(TESTTASK, {"foo", "bar"})
    assert tatr.query.has_tags(TESTTASK, set())


def test_has_no_tags():
    assert not tatr.query.has_no_tag(TESTTASK)
