use std::env;
use std::ffi::CString;

use chrono::Local;

const DIGIT_HEIGHT: usize = 7;

// -- Block style (##), 6 wide --
const BLOCK_W: usize = 6;
const BLOCK_DIGITS: [[&str; DIGIT_HEIGHT]; 10] = [
    [" #### ", "##  ##", "##  ##", "##  ##", "##  ##", "##  ##", " #### "],
    ["  ##  ", " ###  ", "  ##  ", "  ##  ", "  ##  ", "  ##  ", " #### "],
    [" #### ", "##  ##", "    ##", "  ##  ", " ##   ", "##    ", "######"],
    [" #### ", "##  ##", "    ##", "  ### ", "    ##", "##  ##", " #### "],
    ["##  ##", "##  ##", "##  ##", "######", "    ##", "    ##", "    ##"],
    ["######", "##    ", "##    ", "##### ", "    ##", "##  ##", " #### "],
    [" #### ", "##  ##", "##    ", "##### ", "##  ##", "##  ##", " #### "],
    ["######", "    ##", "   ## ", "  ##  ", "  ##  ", "  ##  ", "  ##  "],
    [" #### ", "##  ##", "##  ##", " #### ", "##  ##", "##  ##", " #### "],
    [" #### ", "##  ##", "##  ##", " #####", "    ##", "##  ##", " #### "],
];
const BLOCK_COLON: [&str; DIGIT_HEIGHT] = ["  ", "##", "  ", "  ", "  ", "##", "  "];
const BLOCK_COLON_W: usize = 2;

// -- Fancy style (line drawing), 8 wide --
const FANCY_W: usize = 8;
const FANCY_DIGITS: [[&str; DIGIT_HEIGHT]; 10] = [
    [" /----\\ ", "|      |", "|      |", "|      |", "|      |", "|      |", " \\----/ "],
    ["    /|  ", "   / |  ", "     |  ", "     |  ", "     |  ", "     |  ", "  ------"],
    [" /----\\ ", "|      |", "      / ", "  ---/  ", " /      ", "|       ", " \\------"],
    [" /----\\ ", "      | ", "      | ", "  ---/  ", "      \\ ", "      | ", " \\----/ "],
    ["|      |", "|      |", "|      |", " \\-----|", "       |", "       |", "       |"],
    [" ------\\", "|       ", "|       ", " -----\\ ", "       |", "       |", " -----/ "],
    [" /----\\ ", "|      |", "|       ", "|-----\\ ", "|      |", "|      |", " \\----/ "],
    ["-------\\", "       |", "      / ", "     /  ", "    /   ", "   /    ", "  /     "],
    [" /----\\ ", "|      |", "|      |", " >----< ", "|      |", "|      |", " \\----/ "],
    [" /----\\ ", "|      |", "|      |", " \\-----|", "       |", "       |", " -----/ "],
];
const FANCY_COLON: [&str; DIGIT_HEIGHT] = ["  ", "oo", "  ", "  ", "  ", "oo", "  "];
const FANCY_COLON_W: usize = 2;

// -- Double-line style (box drawing), 10 wide, 9 tall --
const DOUBLE_HEIGHT: usize = 9;
const DOUBLE_W: usize = 10;
const DOUBLE_DIGITS: [[&str; DOUBLE_HEIGHT]; 10] = [
    // 0
    [
        " ╔══════╗ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ╚══════╝ ",
    ],
    // 1
    [
        "     ╔╗   ",
        "    ╔╝║   ",
        "    ║ ║   ",
        "    ║ ║   ",
        "    ║ ║   ",
        "    ║ ║   ",
        "    ║ ║   ",
        "    ║ ║   ",
        "   ╚══╝   ",
    ],
    // 2
    [
        " ╔══════╗ ",
        " ║      ║ ",
        "        ║ ",
        "        ║ ",
        " ╔══════╝ ",
        " ║        ",
        " ║        ",
        " ║      ╗ ",
        " ╚══════╝ ",
    ],
    // 3
    [
        " ╔══════╗ ",
        " ║      ║ ",
        "        ║ ",
        "        ║ ",
        "  ══════╣ ",
        "        ║ ",
        "        ║ ",
        " ║      ║ ",
        " ╚══════╝ ",
    ],
    // 4
    [
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ╚══════╣ ",
        "        ║ ",
        "        ║ ",
        "        ║ ",
        "        ║ ",
    ],
    // 5
    [
        " ╔══════╗ ",
        " ║      ║ ",
        " ║        ",
        " ║        ",
        " ╚══════╗ ",
        "        ║ ",
        "        ║ ",
        " ║      ║ ",
        " ╚══════╝ ",
    ],
    // 6
    [
        " ╔══════╗ ",
        " ║      ║ ",
        " ║        ",
        " ║        ",
        " ╠══════╗ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ╚══════╝ ",
    ],
    // 7
    [
        " ╔══════╗ ",
        " ║      ║ ",
        "        ║ ",
        "       ╔╝ ",
        "       ║  ",
        "      ╔╝  ",
        "      ║   ",
        "      ║   ",
        "      ║   ",
    ],
    // 8
    [
        " ╔══════╗ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ╠══════╣ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ╚══════╝ ",
    ],
    // 9
    [
        " ╔══════╗ ",
        " ║      ║ ",
        " ║      ║ ",
        " ║      ║ ",
        " ╚══════╣ ",
        "        ║ ",
        "        ║ ",
        " ║      ║ ",
        " ╚══════╝ ",
    ],
];
const DOUBLE_COLON: [&str; DOUBLE_HEIGHT] = [
    "    ",
    " ╔╗ ",
    " ╚╝ ",
    "    ",
    "    ",
    "    ",
    " ╔╗ ",
    " ╚╝ ",
    "    ",
];
const DOUBLE_COLON_W: usize = 4;

const GAP: usize = 1;

#[derive(Clone, Copy)]
enum Style {
    Block,
    Fancy,
    Double,
}

fn draw_clock(time_str: &str, date_str: &str, style: Style) {
    ncurses::erase();

    let max_y = ncurses::getmaxy(ncurses::stdscr()) as usize;
    let max_x = ncurses::getmaxx(ncurses::stdscr()) as usize;

    let (digit_w, colon_w, height) = match style {
        Style::Block => (BLOCK_W, BLOCK_COLON_W, DIGIT_HEIGHT),
        Style::Fancy => (FANCY_W, FANCY_COLON_W, DIGIT_HEIGHT),
        Style::Double => (DOUBLE_W, DOUBLE_COLON_W, DOUBLE_HEIGHT),
    };
    let total_width = 6 * digit_w + 5 * GAP + 2 * (colon_w + 2 * GAP);

    if max_y < height + 4 || max_x < total_width {
        ncurses::mvaddstr(0, 0, "Terminal too small!");
        ncurses::refresh();
        return;
    }

    let start_y = (max_y - height) / 2 - 1;
    let start_x = (max_x - total_width) / 2;

    let chars: Vec<char> = time_str.chars().collect();
    let has_colors = ncurses::has_colors();

    for row in 0..height {
        let mut x = start_x;
        for (i, &ch) in chars.iter().enumerate() {
            if ch == ':' {
                x += GAP;
                let line = match style {
                    Style::Block => BLOCK_COLON[row],
                    Style::Fancy => FANCY_COLON[row],
                    Style::Double => DOUBLE_COLON[row],
                };
                if has_colors {
                    ncurses::attron(ncurses::COLOR_PAIR(2));
                }
                ncurses::mvaddstr(start_y as i32 + row as i32, x as i32, line);
                if has_colors {
                    ncurses::attroff(ncurses::COLOR_PAIR(2));
                }
                x += colon_w + GAP;
            } else if let Some(d) = ch.to_digit(10) {
                let line = match style {
                    Style::Block => BLOCK_DIGITS[d as usize][row],
                    Style::Fancy => FANCY_DIGITS[d as usize][row],
                    Style::Double => DOUBLE_DIGITS[d as usize][row],
                };
                let color = if i < 2 { 1 } else if i < 5 { 3 } else { 4 };
                if has_colors {
                    ncurses::attron(ncurses::COLOR_PAIR(color));
                }
                ncurses::mvaddstr(start_y as i32 + row as i32, x as i32, line);
                if has_colors {
                    ncurses::attroff(ncurses::COLOR_PAIR(color));
                }
                x += digit_w + GAP;
            }
        }
    }

    let date_x = (max_x.saturating_sub(date_str.len())) / 2;
    let date_y = start_y + height + 2;
    if has_colors {
        ncurses::attron(ncurses::COLOR_PAIR(5));
    }
    ncurses::mvaddstr(date_y as i32, date_x as i32, date_str);
    if has_colors {
        ncurses::attroff(ncurses::COLOR_PAIR(5));
    }

    let title = "-- fine_clock --";
    let title_x = (max_x - title.len()) / 2;
    if has_colors {
        ncurses::attron(ncurses::COLOR_PAIR(6));
    }
    ncurses::mvaddstr(start_y as i32 - 2, title_x as i32, title);
    if has_colors {
        ncurses::attroff(ncurses::COLOR_PAIR(6));
    }

    ncurses::refresh();
}

fn main() {
    let style = if env::args().any(|a| a == "--double" || a == "-d") {
        Style::Double
    } else if env::args().any(|a| a == "--fancy" || a == "-f") {
        Style::Fancy
    } else {
        Style::Block
    };

    // Enable UTF-8 in ncurses
    let empty = CString::new("").unwrap();
    unsafe {
        libc::setlocale(libc::LC_ALL, empty.as_ptr());
    }

    ncurses::initscr();
    ncurses::noecho();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::timeout(200);
    ncurses::keypad(ncurses::stdscr(), true);

    if ncurses::has_colors() {
        ncurses::start_color();
        ncurses::use_default_colors();
        ncurses::init_pair(1, ncurses::COLOR_CYAN, -1);
        ncurses::init_pair(2, ncurses::COLOR_WHITE, -1);
        ncurses::init_pair(3, ncurses::COLOR_GREEN, -1);
        ncurses::init_pair(4, ncurses::COLOR_YELLOW, -1);
        ncurses::init_pair(5, ncurses::COLOR_MAGENTA, -1);
        ncurses::init_pair(6, ncurses::COLOR_BLUE, -1);
    }

    loop {
        let now = Local::now();
        let time_str = now.format("%H:%M:%S").to_string();
        let date_str = now.format("%A, %B %d, %Y").to_string();

        draw_clock(&time_str, &date_str, style);

        let ch = ncurses::getch();
        if ch == b'q' as i32 || ch == b'Q' as i32 {
            break;
        }
    }

    ncurses::endwin();
}
