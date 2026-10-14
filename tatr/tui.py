import time
import curses
import os

from tatr.query import get_tasks

def draw_menu(stdscr: curses.window, tasks: list[str]) -> int:
    inner_height = curses.LINES - 2
    menu_width = len("| 20261005-180349 |")
    menu = curses.newwin(curses.LINES, menu_width, 0, 0)
    
    menu_top = "\u250C" + ("\u2500" * (menu_width - 1))
    menu_btm = "\u2514" + ("\u2500" * (menu_width - 1))

    selected_row = 0
    curses.init_pair(1, curses.COLOR_WHITE, curses.COLOR_BLUE)

    menu.move(0, 0)
    menu.addstr(menu_top + "\r")
    for idx, task in enumerate(tasks, start=1):
        if idx >= inner_height:
            break

        menu.addstr(idx, 0, "\u2502 ")
        disp = task.split(os.sep)[-1]
        if idx - 1 != selected_row:
            menu.addstr(idx, 2, disp)
        else:
            menu.addstr(idx, 2, disp, curses.color_pair(1))

        menu.addstr(idx, menu_width - 1, "\u2502\r")

    if len(tasks) < inner_height:
        for i in range(len(tasks), inner_height - 1):
            menu.addstr("\u2502" + " " * (menu_width - 2) + "\u2502")

    menu.addstr(menu_btm)

    menu.refresh() 
    return selected_row


def draw_file_display(stdscr: curses.window, task_dir: str):
    window_width = curses.COLS - len("| 20261005-180349 |") - 1
    inner_height = curses.LINES - 2
    win = curses.newwin(curses.LINES, window_width, 0, curses.COLS - window_width - 1)

    # origin = (0, curses.COLS - window_width)
    win_top = ("\u2500" * (window_width - 1)) + "\u2510"
    win_btm = ("\u2500" * (window_width - 1)) + "\u2518"
    # win.move(*origin)
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
    curses.start_color()
    curses.curs_set(False)

    tasks: list[str] = get_tasks()

    while True:
        selected:int = draw_menu(stdscr, tasks)
        draw_file_display(stdscr, tasks[selected])
        stdscr.getch()

