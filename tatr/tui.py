import curses
import os
import subprocess

from tatr.query import get_tasks

def draw_menu(stdscr: curses.window, tasks: list[str], selected: int, inner_height: int):
    menu_width = len("| 20261005-180349 |")
    menu = curses.newwin(curses.LINES, menu_width, 0, 0)

    menu_top = "\u250C" + ("\u2500" * (menu_width - 1))
    menu_btm = "\u2514" + ("\u2500" * (menu_width - 1))

    curses.init_pair(1, curses.COLOR_WHITE, curses.COLOR_BLUE)

    menu.move(0, 0)
    menu.addstr(menu_top + "\r")
    for idx, task in enumerate(tasks, start=1):
        if idx >= inner_height:
            break

        menu.addstr(idx, 0, "\u2502 ")
        disp = task.split(os.sep)[-1]
        if idx - 1 != selected:
            menu.addstr(idx, 2, disp)
        else:
            menu.addstr(idx, 2, disp, curses.color_pair(1))

        menu.addstr(idx, menu_width - 1, "\u2502\r")

    if len(tasks) < inner_height:
        for i in range(len(tasks), inner_height - 1):
            menu.addstr("\u2502" + " " * (menu_width - 2) + "\u2502")

    menu.addstr(menu_btm)
    menu.refresh()


def draw_file_display(stdscr: curses.window, task_dir: str, inner_height: int):
    window_width = curses.COLS - len("| 20261005-180349 |") - 1
    win = curses.newwin(curses.LINES, window_width, 0, curses.COLS - window_width - 1)

    win_top = ("\u2500" * (window_width - 1)) + "\u2510"
    win_btm = ("\u2500" * (window_width - 1)) + "\u2518"
    win.move(0, 0)

    with open(f"{task_dir}/TASK.md") as f:
        lines = f.readlines()

    win.addstr(win_top + "\r")
    for idx, line in enumerate(lines):
        if idx > inner_height:
            break

        visible_line = line.strip()[:window_width - 3]
        if len(visible_line) < (window_width - 2):
            visible_line += (" " * (window_width - 3 - len(visible_line)))

        win.addstr(idx + 1, 2, visible_line + "\u2502\r");

    if len(lines) < inner_height:
        for i in range(len(lines), inner_height - 1):
            win.addstr(" " * (window_width - 1) + "\u2502")

    win.addstr(win_btm)
    win.refresh()


def c_main(stdscr: curses.window):
    stdscr.clear()
    stdscr.nodelay(True)
    stdscr.keypad(True)
    curses.start_color()
    curses.curs_set(False)
    inner_height = curses.LINES - 2

    tasks: list[str] = get_tasks()
    selected: int = 0
    visible_offset = 0
    viewable_tasks = tasks[:inner_height]

    while True:
        draw_menu(stdscr, viewable_tasks, selected, inner_height)
        draw_file_display(stdscr, tasks[selected], inner_height)

        curses.doupdate()

        try:
            key = stdscr.getkey()
        except curses.error:
            continue

        if key == "j":
            if selected < len(tasks):
                selected += 1

                if selected > inner_height:
                    visible_offset += 1
                    viewable_tasks = tasks[visible_offset: visible_offset + inner_height]
        elif key == "k":
            if selected > 0:
                selected -= 1

                if selected < visible_offset:
                    visible_offset -= 1
                    viewable_tasks = tasks[visible_offset: visible_offset + inner_height]
        elif key == "q":
            break
        elif key == "\n":
            curses.endwin()
            editor = os.environ.get("EDITOR", "vi")
            file = f"{tasks[selected]}{os.sep}TASK.md"
            # This needs to repeat `editor` twice for old unix reasons
            os.execlp(editor, editor, file)
            return

